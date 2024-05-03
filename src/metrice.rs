// 基本功能 inc/dec/snapshot
use anyhow::{Ok, Result};
use core::fmt;
use std::sync::Arc;

use dashmap::DashMap;
#[derive(Debug, Clone)]
pub struct Metrice {
    data: Arc<DashMap<String, i64>>,
}

impl Metrice {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Metrice {
        Metrice {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn inc(&mut self, key: impl Into<String>) -> Result<()> {
        let mut counts = self.data.entry(key.into()).or_insert(0);
        *counts += 1;
        Ok(())
    }

    // pub fn dec(&mut self, key: impl Into<String>) {
    //     let mut data = self.data.lock().map_err(|op| anyhow!(op.to_string()))?;
    //     let counter = data.entry(key.into()).or_insert(0);
    //     *counter -= 1;
    // }

    // pub fn snapshot(&self) -> Result<DashMap<String, i64>> {
    //     Ok(self
    //         .data
    //         .read()
    //         .map_err(|op| anyhow!(op.to_string()))?
    //         .clone())
    // }
}

impl fmt::Display for Metrice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for entry in self.data.iter() {
            writeln!(f, "{}: {}", entry.key(), entry.value())?;
        }
        std::result::Result::Ok(())
    }
}
