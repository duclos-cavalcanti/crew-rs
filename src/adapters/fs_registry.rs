use crate::domain::{Agent, Registry};
use crate::services::RegistryRepository;

use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub struct FsRegistryRepository {
    path: PathBuf,
}

impl FsRegistryRepository {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }
}

impl RegistryRepository for FsRegistryRepository {
    fn load(&self) -> Result<Registry> {
        let text = fs::read_to_string(&self.path)?;
        let agents = text
            .lines()
            .filter(|line| !line.trim().is_empty())
            .filter_map(|line| line.split_once('='))
            .map(|(name, path)| Agent::new(name, path))
            .collect();
        Ok(Registry { agents })
    }
}
