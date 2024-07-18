use std::path::PathBuf;

use clap::{arg, command, Parser};
use tracing::{debug, level_filters::LevelFilter};
use tracing_subscriber::{layer::SubscriberExt, registry, util::SubscriberInitExt as _, EnvFilter};

#[derive(Parser, Debug)]
#[command(version, about)]
pub(crate) struct Opts {
    #[arg(short, long, env, default_value = ".")]
    path: PathBuf,

    #[arg(global = true, short, long, env, default_value_t = LevelFilter::INFO)]
    pub log_level: LevelFilter,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let opts = Opts::parse();
    registry()
        .with(
            EnvFilter::builder()
                .with_default_directive(opts.log_level.into())
                .with_env_var("LOG")
                .from_env_lossy(),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();


    debug!("Hello, world! {:?}", opts.path);

    Ok(())
}
