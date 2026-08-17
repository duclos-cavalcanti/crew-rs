use anyhow::Result;

mod domain;
mod ui;
mod config;
mod services;
mod adapters;

use clap::Parser;
use config::Config;
use ui::TerminalState;
use adapters::FsRegistryRepository;
use services::list_sessions;

fn main() -> Result<()> {
    let config = Config::parse();
    let repo = FsRegistryRepository::new(config.registry_path());

    if config.statusline {
        let sessions = list_sessions(&repo)?;
        for session in &sessions {
            // Placeholder — the tmux presenter (#12) replaces this println.
            println!("{}", session.agent.name);
        }
        return Ok(());
    }

    let mut terminal_state = TerminalState::new()?;
    ui::run(&mut terminal_state, &config)
}
