pub trait SessionHost {
    fn is_alive(&self, name: &str) -> bool;
}
