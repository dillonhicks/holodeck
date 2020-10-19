#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod deps {
    pub(crate) use holodeck_core;
    pub(crate) use holodeck_macros;
    pub(crate) use holodeck_net;
    pub(crate) use holodeck_viewer;

    pub(crate) use holodeck_core::deps::{
        bincode,
        futures,
        tokio,
        tokio_tungstenite,
    };
    pub(crate) use log;
    pub(crate) use structopt;
    pub(crate) use tracing;
    pub(crate) use tracing_subscriber;
}


use crate::deps::{
    structopt::StructOpt,
    tracing::Level,
};


#[derive(Clone, Debug, StructOpt)]
#[structopt(name = "Hologram Client (native)", about = "view a running holodeck instance")]
pub(crate) struct Args {
    #[structopt(long, default_value = "info")]
    pub(crate) log: Level,

    /// the host running the instance
    #[structopt(short, long, default_value = "localhost")]
    pub(crate) host: String,

    /// the host port on which to connect
    #[structopt(short, long, default_value = "7000")]
    pub(crate) port: u16,
    /* #[structopt(long, default_value = "WebSocket")]
     * pub(crate) transport: crate::deps::holodeck_net::protocol::Transport, */
}


#[cfg(not(target_arch = "wasm32"))]
mod native;


#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let args = Args::from_args();
    native::run(&args)
}

#[cfg(target_arch = "wasm32")]
fn main() {}
