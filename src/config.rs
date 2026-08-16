use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "crew", version, about)]
pub struct Config {

    /// Poll interval, milliseconds
    #[arg(long, default_value_t = 250)]
    pub tick_ms: u64,
}
