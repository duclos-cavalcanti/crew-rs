use crate::services::SessionHost;

pub struct TmuxHost;

impl TmuxHost {
    fn has_session(name: &str) -> bool {
        std::process::Command::new("tmux")
            .args(["has-session", "-t", &format!("={name}")])
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }

}

impl SessionHost for TmuxHost {
    fn is_alive(&self, name: &str) -> bool {
        Self::has_session(name)
    }
}
