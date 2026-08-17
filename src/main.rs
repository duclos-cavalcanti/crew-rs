mod adapters;
mod config;
mod domain;
mod services;
mod ui;

use anyhow::Result;
use clap::Parser;

use adapters::FsRegistryRepository;
use config::{Command, Config};
use services::{list_sessions, RegistryRepository};
use ui::TerminalState;

fn main() -> Result<()> {
    let config = Config::parse();

    match &config.command {
        // Registry only: the registered session names, one per line.
        Some(Command::List) => {
            let repo = FsRegistryRepository::new(config.registry_path());
            for agent in &repo.load()?.agents {
                println!("{}", agent.name);
            }
            Ok(())
        }
        // Enriched: sessions + state. Placeholder output until state
        // resolution (#17) and the tmux presenter (#18) land.
        Some(Command::Status { .. }) => {
            let repo = FsRegistryRepository::new(config.registry_path());
            for session in &list_sessions(&repo)? {
                println!("{}", session.agent.name);
            }
            Ok(())
        }
        // No subcommand: the live TUI.
        None => {
            let mut terminal_state = TerminalState::new()?;
            ui::run(&mut terminal_state, &config)
        }
    }
}
