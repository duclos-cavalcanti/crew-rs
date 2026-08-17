use anyhow::Result;

use crate::domain::{Session, SessionState};
use crate::services::RegistryRepository;

pub fn list_sessions(repo: &dyn RegistryRepository) -> Result<Vec<Session>> {
    let registry = repo.load()?;
    let sessions = registry
        .agents
        .into_iter()
        .map(|agent| Session {
            agent,
            state: SessionState::Unknown,
        })
        .collect();
    Ok(sessions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Agent, Registry};

    struct DummyRepository(Registry);
    impl RegistryRepository for DummyRepository {
        fn load(&self) -> Result<Registry> {
            Ok(self.0.clone())
        }
    }

    #[test]
    fn maps_agents_to_sessions() {
        let reg = Registry {
            agents: vec![
                Agent::new("millwright", "/a"),
                Agent::new("prover", "/b"),
            ],
        };
        let repo = DummyRepository(reg);

        let sessions = list_sessions(&repo).expect("should succeed");

        assert_eq!(sessions.len(), 2);
        assert_eq!(sessions[0].agent.name, "millwright");
    }
}
