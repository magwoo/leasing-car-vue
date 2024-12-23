use anyhow::{Context, Result};
use sled::Db;
use std::sync::Arc;

#[derive(Clone)]
pub struct Database(Arc<Db>);

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let db = sled::open(path).with_context(|| format!("failed to open {path}"))?;

        Ok(Self(Arc::new(db)))
    }

    pub fn add_submit(&self, name: Option<&str>, phone: u64) -> Result<()> {
        let is_exists = self
            .0
            .contains_key(phone.to_le_bytes())
            .context("failed to check exists")?;

        if is_exists && name.is_none() {
            return Ok(());
        }

        self.0
            .insert(phone.to_le_bytes(), name.unwrap_or("#"))
            .with_context(|| format!("failed to insert submit: {name:?}:{phone}"))?;

        Ok(())
    }
}
