use std::vec::Vec;

use super::Session;

pub struct SessionRegistry {
    pub sessions: Vec<Session>,
}

impl SessionRegistry {
    pub fn new() -> Self {
        SessionRegistry {
            sessions: Vec::new()
        }
    }
}
