mod adapters;
mod config;
mod domain;
mod services;
mod ui;

use anyhow::Result;
use clap::Parser;

use adapters::FsRegistryRepository;
use config::Config;
use services::list_sessions;
use ui::TerminalState;

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
