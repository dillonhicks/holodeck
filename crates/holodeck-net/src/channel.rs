use crate::{
    deps::{
        futures::{
            stream::{
                SplitSink,
                SplitStream,
            },
            SinkExt,
        },
        futures_util::StreamExt,
        holodeck_core::Result,
        log::{
            error,
            info,
        },
        tokio,
        tokio::{
            net::TcpStream,
            sync::mpsc::Sender,
        },
        tokio_tungstenite::{
            tungstenite::Message,
            WebSocketStream,
        },
    },
    message::{
        SimulationState,
        SpawnRequest,
        ToWebSocketMessage,
    },
};


pub struct SimulationChannel {
    sink:       SplitSink<WebSocketStream<TcpStream>, Message>,
    fwd_handle: tokio::task::JoinHandle<()>,
}


impl SimulationChannel {
    pub fn new(
        client_stream: WebSocketStream<TcpStream>,
        fwd: Sender<SpawnRequest>,
    ) -> SimulationChannel {
        let (sink, incoming) = client_stream.split();
        SimulationChannel {
            fwd_handle: tokio::task::spawn(simulation_message_forwarding(incoming, fwd)),
            sink,
        }
    }

    pub async fn send(
        &mut self,
        state: &SimulationState,
    ) -> Result<()> {
        if let Ok(message) = state.to_message().map_err(peek_warn!()) {
            self.sink
                .send(message)
                .await
                .map_err(crate::deps::holodeck_core::Error::from)
                .map_err(peek_warn!("could not send simulation state"))?;
        }

        Ok(())
    }
}


async fn simulation_message_forwarding(
    mut stream: SplitStream<WebSocketStream<TcpStream>>,
    mut forwarder: Sender<SpawnRequest>,
) {
    while let Some(msg) = stream.next().await {
        match msg {
            Ok(Message::Binary(serialized)) => {
                match crate::deps::bincode::deserialize::<SpawnRequest>(&serialized) {
                    Ok(input) => {
                        info!("recv client input: {:?}", input);
                        forwarder
                            .send(input)
                            .await
                            .unwrap_or_else(crash_on_err!("failed to forward message: {:?}", input));
                    }
                    Err(e) => error!("unable to decode message: {:?}, {:?}", serialized, e,),
                }
            }
            m => info!("Got invalid message kind: {:?}", m),
        };
    }
}
