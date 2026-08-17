use anyhow::Result;

use crate::domain::{Session, SessionState};
use crate::services::{RegistryRepository, StateStore};

pub fn list_sessions(repo: &dyn RegistryRepository, store: &dyn StateStore) -> Result<Vec<Session>> {
    let registry = repo.load()?;
    let sessions = registry
        .agents
        .into_iter()
        .map(|agent| {
            let state = match store.read_state(&agent.name)? {
                Some(s) => SessionState::from(s.as_str()), 
                None => SessionState::Unknown
            };
            Ok(Session {
                agent,
                state
            })
        })
        .collect();
    sessions
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

    struct DummyStateStore;
    impl StateStore for DummyStateStore {
        fn read_state(&self, _name: &str) -> Result<Option<String>> {
            Ok(None)
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

        let sessions = list_sessions(&repo, &DummyStateStore).expect("should succeed");

        assert_eq!(sessions.len(), 2);
        assert_eq!(sessions[0].agent.name, "millwright");
    }
}
