mod adapters;
mod config;
mod domain;
mod present;
mod services;
mod ui;

use anyhow::Result;
use clap::Parser;

use adapters::{FsRegistryRepository, FsStateStore};
use config::{Command, Config, Format};
use services::{list_sessions, RegistryRepository};
use ui::TerminalState;

fn main() -> Result<()> {
    let config = Config::parse();

    match &config.command {
        Some(Command::List) => {
            let repo = FsRegistryRepository::new(config.registry_path());
            for agent in &repo.load()?.agents {
                println!("{}", agent.name);
            }
            Ok(())
        }
        Some(Command::Status { format }) => {
            let repo = FsRegistryRepository::new(config.registry_path());
            let store = FsStateStore::new(config.state_path());
            let sessions = list_sessions(&repo, &store)?;
            let output = match format {
                Format::Plain => present::plain(&sessions),
                Format::Tmux => present::tmux(&sessions),
            };
            println!("{output}");
            Ok(())
        }
        None => {
            let mut terminal_state = TerminalState::new()?;
            ui::run(&mut terminal_state, &config)
        }
    }
}
