use std::borrow::Cow;

pub mod deps {
    pub use anyhow;
    pub use bincode;
    #[cfg(not(target_arch = "wasm32"))]
    pub use futures;
    #[cfg(not(target_arch = "wasm32"))]
    pub use futures_util;
    #[cfg(feature = "bfloat16")]
    pub use half;
    pub use serde;
    pub use thiserror;
    #[cfg(not(target_arch = "wasm32"))]
    pub use tokio;
    #[cfg(not(target_arch = "wasm32"))]
    pub use tokio_tungstenite;
    #[cfg(not(target_arch = "wasm32"))]
    pub use tokio_tungstenite::tungstenite;
}

pub type Result<T> = std::result::Result<T, Error>;


#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("an io error occurred, \"{message}\": error={err}")]
    SystemIo {
        //#[from]
        err:     std::io::Error,
        message: Cow<'static, str>,
    },

    #[error("an internal error occurred, \"{message}\": error={err}")]
    Internal {
        err:     Box<dyn std::error::Error>,
        message: Cow<'static, str>,
    },
    #[error("could not create an instance of `{to}` from the value `{value}` with type `{from}`")]
    BadValue {
        from:  Cow<'static, str>,
        to:    Cow<'static, str>,
        value: Cow<'static, str>,
    },
    #[cfg(not(target_arch = "wasm32"))]
    #[error("a websocket operation failed: error={err}")]
    WebSocket { err: crate::deps::tungstenite::Error },

    #[error("bincode serialization error: {err}")]
    BincodeSerialize { err: crate::deps::bincode::Error },
}


impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::SystemIo {
            err,
            message: "unhandled io error".into(),
        }
    }
}


impl From<crate::deps::anyhow::Error> for Error {
    fn from(err: crate::deps::anyhow::Error) -> Self {
        Error::Internal {
            err:     err.into(),
            message: Cow::Borrowed(""),
        }
    }
}


impl From<crate::deps::bincode::Error> for Error {
    fn from(err: crate::deps::bincode::Error) -> Self {
        Error::BincodeSerialize { err }
    }
}


#[cfg(not(target_arch = "wasm32"))]
impl From<crate::deps::tungstenite::Error> for Error {
    fn from(err: crate::deps::tungstenite::Error) -> Self {
        Error::WebSocket { err }
    }
}


pub mod messages {
    use crate::deps::serde;
    #[cfg(not(target_arch = "wasm32"))]
    pub use crate::deps::tungstenite::Message as WebSocketMessage;

    #[cfg(not(target_arch = "wasm32"))]
    pub trait ToWebSocketMessage {
        type Error: std::error::Error;
        fn to_message(&self) -> std::result::Result<WebSocketMessage, Self::Error>;
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Blanket implementation for all local types that implement serialize
    /// to shove them into a websocket message.
    impl<T> ToWebSocketMessage for T
    where
        T: serde::Serialize,
    {
        type Error = crate::Error;

        fn to_message(&self) -> crate::Result<WebSocketMessage> {
            let message = bincode::serialize::<T>(self).map(|bytes| WebSocketMessage::Binary(bytes))?;
            Ok(message)
        }
    }


    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum Message {
        Test,
    }


    #[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
    pub struct SimulationState {
        pub tick:         u64,
        pub entity_count: u64,
        pub entities:     Vec<Entity>,
    }


    #[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
    pub struct SpawnRequest {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }


    #[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
    pub struct Entity {
        pub id:  u64,
        pub tag: u16,
        #[cfg_attr(
            all(feature = "bfloat16", feature = "serde_f32_as_bfloat16"),
            serde(with = "as_bfloat16")
        )]
        pub x:   f32,
        #[cfg_attr(
            all(feature = "bfloat16", feature = "serde_f32_as_bfloat16"),
            serde(with = "as_bfloat16")
        )]
        pub y:   f32,
        #[cfg_attr(
            all(feature = "bfloat16", feature = "serde_f32_as_bfloat16"),
            serde(with = "as_bfloat16")
        )]
        pub z:   f32,
    }

    #[cfg(feature = "bfloat16")]
    mod as_bfloat16 {
        use crate::deps::{
            half::bf16,
            serde::{
                Deserialize,
                Deserializer,
                Serializer,
            },
        };

        pub fn serialize<S>(
            value: &f32,
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let v = bf16::from_f32(*value);
            serializer.serialize_u16(v.to_bits())
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<f32, D::Error>
        where
            D: Deserializer<'de>,
        {
            let value: bf16 = bf16::deserialize(deserializer)?;
            Ok(value.to_f32())
        }
    }
}
