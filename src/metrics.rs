// metrics data structure
// 基础功能：inc/dec/snapshot
use anyhow::Result;
use dashmap::DashMap;
use std::{
    // collections::HashMap,
    fmt,
    sync::Arc,
};

#[derive(Debug, Clone, Default)]
pub struct Metrics {
    data: Arc<DashMap<String, i64>>, // Arc<RwLock<HashMap<String, i64>>> => Arc<DashMap<String, i64>>
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter += 1;

        Ok(())
    }

    pub fn dec(&self, key: impl Into<String>) -> Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter -= 1;

        Ok(())
    }
}

impl fmt::Display for Metrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // let data = self.data.read().map_err(|_e| fmt::Error)?;
        for entry in self.data.iter() {
            writeln!(f, "{}: {}", entry.key(), entry.value())?;
        }

        Ok(())
    }
}
