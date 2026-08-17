#![allow(dead_code)]

use crate::domain::{SessionState, Session};

fn glyph(state: &SessionState) -> &'static str {
    match state {
        SessionState::Working => "»",
        SessionState::Done => "⏸",
        SessionState::Waiting => "⏸",
        SessionState::Idle => "●",
        SessionState::Live => "●",
        SessionState::Dead => "○",
        SessionState::Unknown => "○",
    }
}

fn color(state: &SessionState) -> &'static str {
    match state {
        SessionState::Working => "green",
        SessionState::Done => "red",
        SessionState::Waiting => "white",
        SessionState::Idle => "brightblack",
        SessionState::Live => "green",
        SessionState::Dead => "brightblack",
        SessionState::Unknown => "brightblack",
    }
}

pub fn tmux(sessions: &[Session]) -> String {
    sessions 
        .iter() 
        .map(|s|
            format!("#[fg={}]{} {}#[fg=default]", 
                color(&s.state), 
                glyph(&s.state), 
                s.agent.name
        ))
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn plain(sessions: &[Session]) -> String {
    sessions 
        .iter() 
        .map(|s| format!("{} {}", glyph(&s.state), s.agent.name))
        .collect::<Vec<_>>()
        .join(" ")
}
