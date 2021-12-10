#![warn(clippy::all)]
use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
struct Opts {
    #[clap(
        short = 'f',
        long,
        env = "BTKNMLE_VAR_FILE",
        default_value = "/var/lib/btknmle/db.toml"
    )]
    var_file: PathBuf,

    #[clap(short = 'd', long, env = "BTKNMLE_DEVID", default_value = "0")]
    device_id: u16,

    #[clap(long, env = "BTKNMLE_GRAB")]
    grab: bool,

    #[clap(short = 'v', long, parse(from_occurrences), conflicts_with_all = &["debug", "trace"])]
    verbosity: usize,

    #[clap(short = 'D', long, env = "BTKNMLE_DEBUG", conflicts_with_all = &["trace", "verbosity"])]
    debug: bool,

    #[clap(short = 'T', long, env = "BTKNMLE_TRACE", conflicts_with_all = &["debug", "verbosity"])]
    trace: bool,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let Opts {
        var_file,
        device_id,
        grab,
        mut verbosity,
        debug,
        trace,
    } = Opts::parse();

    if trace {
        verbosity += 2;
    }
    if debug {
        verbosity += 1;
    }

    simplelog::TermLogger::init(
        match verbosity {
            0 => simplelog::LevelFilter::Info,
            1 => simplelog::LevelFilter::Debug,
            _ => simplelog::LevelFilter::Trace,
        },
        simplelog::ConfigBuilder::new()
            .set_thread_level(simplelog::LevelFilter::Off)
            .set_time_level(simplelog::LevelFilter::Off)
            .set_target_level(simplelog::LevelFilter::Off)
            .build(),
        simplelog::TerminalMode::Stderr,
        simplelog::ColorChoice::Auto,
    )?;
    btknmle::run(var_file, device_id, grab).await
}
