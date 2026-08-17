use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "crew", version, about)]
pub struct Config {
    /// Print the session list and exit, instead of the TUI.
    #[arg(long)]
    pub list: bool,

    /// Crew directory holding `registry` and `state/` [default: ~/.config/crew].
    #[arg(long, env = "CREW_DIRECTORY")]
    pub crew_dir: Option<PathBuf>,

    /// Poll interval, milliseconds.
    #[arg(long, default_value_t = 250)]
    pub tick_ms: u64,
}

impl Config {
    pub fn crew_dir(&self) -> PathBuf {
        self.crew_dir.clone().unwrap_or_else(|| {
            let home = std::env::var("HOME").unwrap_or_default();
            PathBuf::from(home).join(".config/crew")
        })
    }

    pub fn registry_path(&self) -> PathBuf {
        self.crew_dir().join("registry")
    }

    pub fn state_path(&self) -> PathBuf {
        self.crew_dir().join("state")
    }
}
