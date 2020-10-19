pub(crate) mod deps {
    pub(crate) use holodeck_core;


    pub(crate) use holodeck_core::deps::{
        bincode,
        futures,
        futures_util,
        serde,
        tokio,
        tokio_tungstenite,
    };
    pub(crate) use log;
    #[cfg(feature = "tracing")]
    pub(crate) use tracing;
}

#[macro_use]
mod macros;
mod channel;
pub mod message;
pub mod protocol;
pub mod server;
mod utils;
