use std::sync::mpsc::{
    channel,
    Receiver,
    Sender,
    TryRecvError,
};

use crate::message::{
    ToWebSocketMessage,
    WebSocketMessage,
};


#[derive(Copy, Clone, Debug)]
pub enum Transport {
    WebSocket,
}


impl Transport {
    pub fn from_str<S: AsRef<str>>(s: S) -> Option<Self> {
        match s.as_ref().to_ascii_lowercase().as_str() {
            "web-socket" => Some(Self::WebSocket),
            "websocket" => Some(Self::WebSocket),
            "ws" => Some(Self::WebSocket),
            _ => None,
        }
    }
}


impl std::str::FromStr for Transport {
    type Err = crate::deps::holodeck_core::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Transport::from_str(s).ok_or_else(|| {
            crate::deps::holodeck_core::Error::BadValue {
                from:  "str".into(),
                to:    "Transport".into(),
                value: s.to_string().into(),
            }
        })
    }
}


// #[derive(Copy, Clone, Debug)]
// pub enum Protocol {
//     Binary(BinaryVersion),
// }
//
//
// impl Protocol {
//     pub fn from_str<S: AsRef<str>>(s: S) -> Option<Self> {
//         match s.as_ref().to_ascii_lowercase().as_str() {
//             "web-socket" => Some(Self::WebSocket),
//             "websocket" => Some(Self::WebSocket),
//             "ws" => Some(Self::WebSocket),
//             _ => None
//         }
//     }
// }

// impl std::str::FromStr for Protocol {
//     type Err = crate::deps::holodeck_core::Error;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         Protocol::from_str(s).ok_or_else(|| {
//             crate::deps::holodeck_core::Error::BadValue {
//                 from: "str".into(),
//                 to: "Protocol".into(),
//                 value: s.to_string().into()
//             }
//         })
//     }
// }


pub struct BidirectionMessageStream<Tx, Rx> {
    tx: Sender<WebSocketMessage>,
    rx: Receiver<WebSocketMessage>,
    _p: std::marker::PhantomData<(Tx, Rx)>,
}


pub enum Recv<M> {
    Msg(M),
    Empty,
    Invalid,
    Disconnected,
}

impl<Tx, Rx> BidirectionMessageStream<Tx, Rx>
where
    Tx: ToWebSocketMessage,
{
    pub fn send(
        &self,
        value: &Tx,
    ) {
        value
            .to_message()
            .map(|msg| self.tx.send(msg).unwrap_or_else(crash_on_err!()))
            .unwrap_or_else(crash_on_err!());
    }
}


impl<Tx, Rx> BidirectionMessageStream<Tx, Rx>
where
    for<'de> Rx: crate::deps::serde::Deserialize<'de>,
{
    pub fn recv(&self) -> Recv<Rx> {
        match self.rx.try_recv() {
            Ok(WebSocketMessage::Binary(m)) => {
                Recv::Msg(crate::deps::bincode::deserialize(m.as_slice()).unwrap())
            }
            Err(TryRecvError::Empty) => Recv::Empty,
            Err(TryRecvError::Disconnected) => Recv::Disconnected,
            _ => Recv::Invalid,
        }
    }
}



pub struct BidirectionalStream<Tx, Rx> {
    tx: Sender<Tx>,
    rx: Receiver<Rx>,
}



impl<Tx, Rx> BidirectionalStream<Tx, Rx> {
    pub fn send(
        &self,
        value: Tx,
    ) {
        self.tx.send(value).unwrap_or_else(crash_on_err!())
    }

    pub fn recv(&self) -> Recv<Rx> {
        match self.rx.try_recv() {
            Ok(msg) => Recv::Msg(msg),
            Err(TryRecvError::Empty) => Recv::Empty,
            Err(TryRecvError::Disconnected) => Recv::Disconnected,
        }
    }
}


#[derive(derive_more::Deref, derive_more::DerefMut)]
pub struct Channel<Tx, Rx>(BidirectionalStream<Tx, Rx>);


pub fn channel_pair<S, C>() -> (Channel<C, S>, Channel<S, C>) {
    let (s_tx, s_rx) = channel();
    let (c_tx, c_rx) = channel();

    (
        Channel(BidirectionalStream { tx: c_tx, rx: s_rx }),
        Channel(BidirectionalStream { tx: s_tx, rx: c_rx }),
    )
}



#[derive(derive_more::Deref, derive_more::DerefMut)]
pub struct MessageChannel<Tx, Rx> {
    stream: BidirectionMessageStream<Tx, Rx>,
}


pub type FrontEnd<Tx, Rx> = MessageChannel<Tx, Rx>;
pub type BackEnd<Tx, Rx> = MessageChannel<Tx, Rx>;


pub fn server_channel<S, C>() -> (FrontEnd<C, S>, BackEnd<S, C>) {
    let (s_tx, s_rx) = channel(); // from server to sim
    let (c_tx, c_rx) = channel(); // from sim to server

    (
        FrontEnd {
            stream: BidirectionMessageStream {
                tx: c_tx,
                rx: s_rx,
                _p: std::marker::PhantomData,
            },
        },
        BackEnd {
            stream: BidirectionMessageStream {
                tx: s_tx,
                rx: c_rx,
                _p: std::marker::PhantomData,
            },
        },
    )
}
