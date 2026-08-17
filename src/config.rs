use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "crew", version, about)]
pub struct Config {
    /// Print the session list and exit, instead of the TUI.
    #[arg(long)]
    pub list: bool,

    /// Path to the crew registry file [default: ~/.config/crew/registry].
    #[arg(long)]
    pub registry: Option<PathBuf>,

    /// Poll interval, milliseconds.
    #[arg(long, default_value_t = 250)]
    pub tick_ms: u64,
}

impl Config {
    pub fn registry_path(&self) -> PathBuf {
        self.registry.clone().unwrap_or_else(|| {
            let home = std::env::var("HOME").unwrap_or_default();
            PathBuf::from(home).join(".config/crew/registry")
        })
    }
}
