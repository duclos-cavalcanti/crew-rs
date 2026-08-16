use anyhow::Result;

mod domain;
mod ui;
mod config;

use clap::Parser;
use config::Config;
use ui::TerminalState;

fn main() -> Result<()> {
    let config = Config::parse();
    let mut terminal_state = TerminalState::new()?;
    ui::run(&mut terminal_state, &config)
}
