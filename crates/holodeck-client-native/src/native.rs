use std::{
    mem,
    sync::{
        Arc,
        Mutex,
    },
    thread,
    thread::JoinHandle,
    time::Duration,
};

use crate::deps::{
    futures::SinkExt,
    holodeck_core::messages::ToWebSocketMessage,
    holodeck_net::message::{
        SimulationState,
        SpawnRequest,
        WebSocketMessage,
    },
};

use crate::deps::{
    holodeck_viewer::app::BackendChannel,
    log::{
        debug,
        error,
        info,
        warn,
    },
};

use crate::deps::tokio::{
    io::AsyncReadExt,
    net::TcpStream,
    stream::StreamExt,
};

use crate::deps::{
    holodeck_macros::holodeck,
    holodeck_net::protocol::Transport,
    tokio_tungstenite::{
        connect_async,
        WebSocketStream,
    },
    tracing::{
        info_span,
        Instrument,
        Level,
    },
};

#[derive(Clone)]
pub struct BackendChannelWrapper {
    tx: Arc<Mutex<Vec<SpawnRequest>>>,
    rx: Arc<Mutex<Option<Box<SimulationState>>>>,
}


impl BackendChannel for BackendChannelWrapper {
    type Rx = Box<SimulationState>;
    type Tx = SpawnRequest;

    fn send(
        &self,
        value: Self::Tx,
    ) {
        let mut tx = self.tx.lock().expect("could not lock tx");
        tx.push(value);
    }

    fn recv(&self) -> Option<Self::Rx> {
        let mut rx = self.rx.lock().expect("could not lock rx");
        rx.take()
    }
}


#[holodeck(call_once)]
fn init_logging(level: Level) {
    use tracing_log::LogTracer;
    LogTracer::init().unwrap();

    // std::env::set_var("RUST_LOG", args.common.log.to_string());
    let filter = crate::deps::tracing_subscriber::EnvFilter::from_default_env().add_directive(level.into());

    let subscriber = crate::deps::tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(filter)
        .finish();


    crate::deps::tracing::subscriber::set_global_default(subscriber).unwrap_or_else(|err| {
        panic!(
            "testing::logging::initialize() could not setup global log even subscriber due to {:?}",
            err
        )
    });
}


pub struct SimulationWebSocketClient(JoinHandle<()>);

impl SimulationWebSocketClient {
    pub fn spawn<S>(
        url: S,
        frontend: BackendChannelWrapper,
    ) -> Self
    where
        S: AsRef<str>,
    {
        let url = url.as_ref().to_string();
        let handle = thread::spawn(move || {
            let mut rt = crate::deps::tokio::runtime::Builder::new()
                .enable_all()
                .basic_scheduler()
                .thread_name("ws-client")
                .build()
                .unwrap();

            let fut = Self::run_task(url, frontend).instrument(info_span!("net-worker"));

            rt.block_on(fut);
        });

        Self(handle)
    }

    async fn run_task(
        endpoint: String,
        frontend: BackendChannelWrapper,
    ) {
        use crate::deps::tokio_tungstenite::tungstenite::Error;

        crate::deps::tokio::spawn(Self::notify_running());
        let mut socket = Self::must_connect(&endpoint).await;

        loop {
            match socket.next().await {
                Some(Ok(message)) => Self::handle_message(message, &frontend).await,
                Some(Err(Error::ConnectionClosed))
                | Some(Err(Error::Protocol(_)))
                | Some(Err(Error::Io(_)))
                | None => {
                    socket = Self::must_connect(&endpoint).await;
                }
                unhandled => panic!("{:?}", unhandled),
            };

            // forward messages received from the backend
            // to the connection
            let mut rx = frontend.tx.lock().expect("data potato");

            let messages = mem::take(&mut *rx);
            'forward: for data in messages {
                if let Ok(ws_msg) = data.to_message() {
                    let _result = socket.send(ws_msg).await;
                } else {
                    break 'forward;
                }
            }
        }
    }

    /// occasionally send out a log message to let users know the process is still running
    async fn notify_running() {
        let start = std::time::Instant::now();
        loop {
            crate::deps::tokio::time::delay_for(Duration::from_secs(60)).await;
            log::info!("process running for {:?}", start.elapsed());
        }
    }

    async fn must_connect(url: &str) -> WebSocketStream<TcpStream> {
        'connect: loop {
            match connect_async(url).await {
                Ok((socket, _)) => {
                    info!("established connection to {}", url);
                    return socket;
                }
                Err(err) => {
                    log::warn!("connection failed, retrying in 3s: err={}", err);
                    crate::deps::tokio::time::delay_for(Duration::from_secs(3)).await;
                }
            }
        }
    }

    async fn handle_message(
        message: WebSocketMessage,
        frontend: &BackendChannelWrapper,
    ) {
        match message {
            WebSocketMessage::Binary(b) => {
                let message_size = b.len();
                if let Some(state) =
                    crate::deps::bincode::deserialize_from::<_, Box<SimulationState>>(&b[..]).ok()
                {
                    // send state to the backend
                    debug!(
                        "received simulation update message: tick={:?}; bytes={}",
                        state.tick, message_size
                    );
                    let mut tx = frontend.rx.lock().expect("could not lock message queue");
                    tx.replace(state);
                } else {
                    error!("bad message: len={:?}", b.len());
                }
            }
            WebSocketMessage::Text(_) | WebSocketMessage::Ping(_) | WebSocketMessage::Pong(_) => {
                error!("unexpected websocket message type");
            }
            WebSocketMessage::Close(_) => {
                warn!("connection closed!");
            }
        };
    }
}



pub(crate) fn run(args: &crate::Args) {
    let url = format!("ws://{}:{}", args.host, args.port);

    init_logging(args.log);

    let backend_channel = BackendChannelWrapper {
        tx: Arc::new(Mutex::new(vec![])),
        rx: Arc::new(Mutex::new(None)),
    };

    let _handle = SimulationWebSocketClient::spawn(url, backend_channel.clone());

    // start server
    info!("viewer up and running");
    let backend: Box<dyn BackendChannel<Tx = _, Rx = _>> = Box::new(backend_channel);
    holodeck_viewer::app::PlayerGameClient::run(Some(backend));
}
