use super::agent::Agent;

#[derive(Clone)]
pub enum SessionState {
    Working,
    Done,
    Waiting,
    Idle,
    Live,
    Dead,
    Unknown,
}

#[derive(Clone)]
pub struct Session {
    pub agent: Agent,
    pub state: SessionState,
}
