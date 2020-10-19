use std::time::Duration;

#[cfg(feature = "tracing")]
use crate::deps::tracing::tracing;
use crate::{
    channel::SimulationChannel,
    deps::{
        futures::SinkExt,
        log::info,
        tokio,
        tokio::{
            net::TcpStream,
            sync::mpsc::{
                channel,
                Receiver,
                Sender,
            },
        },
        tokio_tungstenite::WebSocketStream,
    },
    message::{
        SimulationState,
        SpawnRequest,
    },
    protocol::{
        FrontEnd,
        Recv,
    },
};
use std::{
    net::{
        IpAddr,
        SocketAddr,
    },
    sync::{
        atomic::{
            AtomicBool,
            Ordering,
        },
        Arc,
    },
};

use crate::deps::holodeck_core::Result;
use smallvec::SmallVec;
use std::mem;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Config {
    pub ip:                    IpAddr,
    pub port:                  u16,
    pub tick:                  Duration,
    pub simulation_world_size: f32,
    pub max_entities:          usize,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            ip:                    IpAddr::from([0, 0, 0, 0]),
            port:                  7000,
            tick:                  Duration::from_millis(33),
            simulation_world_size: 1000.0,
            max_entities:          1024,
        }
    }
}


pub struct WebSocketServer {
    config: Config,
}


impl WebSocketServer {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self, service, running)))]
    pub async fn run_until_shutdown(
        self,
        service: FrontEnd<SpawnRequest, SimulationState>,
        running: Arc<AtomicBool>,
    ) -> Result<()> {
        let Self { config } = self;



        let mut ws_server = ServerImpl::spawn(config);

        let mut clients = SmallVec::<[Box<SimulationChannel>; 32]>::new();

        let (forwarder, mut client_inputs) = channel(32);

        let _count = 0;
        'serve: while running.load(Ordering::Relaxed) {
            // add any new clients
            // just the client stream
            while let Some(client_stream) = ws_server.recv() {
                clients.push(Box::new(SimulationChannel::new(client_stream, forwarder.clone())));
            }

            // read state from agent app
            match service.recv() {
                Recv::Msg(state) => {
                    // send state to all clients

                    for mut client in mem::take(&mut clients).into_iter() {
                        if let Ok(_) = client.send(&state).await {
                            clients.push(client)
                        }
                    }
                }
                Recv::Invalid | Recv::Empty => { /* no-op */ }
                Recv::Disconnected => break 'serve,
            }

            while let Some(input) = client_inputs.try_recv().ok() {
                // forward?
                service.send(&input);
            }
        }

        info!("websocket server terminating gracefully");
        Ok(())
    }
}


struct ServerImpl {
    rx:     Receiver<WebSocketStream<TcpStream>>,
    handle: tokio::task::JoinHandle<()>,
}

impl ServerImpl {
    fn spawn(config: Config) -> ServerImpl {
        let (tx, rx) = channel(32);
        let handle = tokio::task::spawn(Self::listen(config, tx));
        ServerImpl { rx, handle }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip(config, socket_tx)))]
    async fn listen(
        config: Config,
        socket_tx: Sender<WebSocketStream<TcpStream>>,
    ) {
        use crate::deps::tokio::net::TcpListener;
        let addr = SocketAddr::new(config.ip, config.port);

        let mut listener = TcpListener::bind(&addr).await.unwrap_or_else(crash_on_err!(
            "cannot listen to {:?}, config: {:?}",
            addr,
            config
        ));
        info!("ready to accept connections, listening on: {}", addr);

        while let Ok((stream, _socketaddr)) = listener.accept().await {
            let peer = stream
                .peer_addr()
                .unwrap_or_else(crash_on_err!("connected streams should have a peer address"));
            info!("peer address: {}", peer);

            let mut tx_ws = socket_tx.clone();
            tokio::spawn(async move {
                let ws_stream = accept_connection(peer, stream).await;
                let _ = tx_ws.send(ws_stream).await;
            });
        }
    }

    async fn join(self) {
        self.handle
            .await
            .unwrap_or_else(crash_on_err!("could not join() the listener thread!"));
    }

    fn recv(&mut self) -> Option<WebSocketStream<TcpStream>> {
        // clients com out of here..
        self.rx.try_recv().ok()
    }
}


#[cfg_attr(feature = "tracing", tracing::instrument(skip(stream)))]
async fn accept_connection(
    peer: SocketAddr,
    stream: TcpStream,
) -> WebSocketStream<TcpStream> {
    let addr = stream
        .peer_addr()
        .unwrap_or_else(crash_on_err!("connected streams should have a peer address"));

    info!("Peer address: {}", addr);

    let ws_stream = crate::deps::tokio_tungstenite::accept_async(stream)
        .await
        .unwrap_or_else(crash_on_err!("Error during the websocket handshake occurred"));

    info!("New WebSocket connection: {}", addr);

    ws_stream
}
