pub(crate) mod deps {
    pub(crate) use holodeck_core;
    pub(crate) use holodeck_macros;
    pub(crate) use holodeck_net;

    pub(crate) use log;
    #[cfg(feature = "devserver")]
    pub(crate) use rand;
    pub(crate) use structopt;
    pub(crate) use tokio;
    pub(crate) use tracing;
    pub(crate) use tracing_log;
    pub(crate) use tracing_subscriber;
}


use crate::deps::{
    holodeck_core::Result,
    holodeck_macros::holodeck,
    structopt::StructOpt,
    tracing::Level,
};

#[cfg(feature = "devserver")]
mod devserver;

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(flatten)]
    common: CommonArgs,
    #[structopt(subcommand)]
    action: Action,
}

#[derive(Debug, StructOpt)]
pub struct CommonArgs {
    #[structopt(long, default_value = "info")]
    pub log: Level,
}

#[derive(Debug, StructOpt)]
enum Action {
    Run(Command),
}


#[derive(Debug, StructOpt)]
enum Command {
    #[cfg(feature = "devserver")]
    DevServer(devserver::Args),
}


#[holodeck(call_once)]
fn init_logging(level: Level) {
    use crate::deps::{
        tracing::subscriber::set_global_default,
        tracing_log::LogTracer,
        tracing_subscriber::{
            fmt::Subscriber,
            EnvFilter,
        },
    };

    LogTracer::init().unwrap();

    let filter = EnvFilter::from_default_env().add_directive(level.into());

    let subscriber = Subscriber::builder().with_env_filter(filter).finish();

    set_global_default(subscriber).unwrap_or_else(|err| {
        panic!(
            "testing::logging::initialize() could not setup global log even subscriber due to {:?}",
            err
        )
    });
}


fn main() -> Result<()> {
    let args = Args::from_args();
    init_logging(args.common.log);

    let action = &args.action;
    let common = &args.common;

    match action {
        #[cfg(feature = "devserver")]
        Action::Run(Command::DevServer(cmd_args)) => devserver::Server::run(common, cmd_args),
    }
}
