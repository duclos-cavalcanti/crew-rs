use super::agent::Agent;

#[derive(Clone)]
pub struct Registry {
    pub agents: Vec<Agent>,
}

impl Registry {
    pub fn new() -> Self {
        Registry {
            agents: Vec::new(),
        }
    }
}
