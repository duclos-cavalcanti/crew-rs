use crate::domain::Registry;

use anyhow::Result;

pub trait RegistryRepository {
    fn load(&self) -> Result<Registry>;
}
