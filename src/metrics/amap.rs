use std::{
    collections::HashMap,
    fmt,
    sync::{
        atomic::{AtomicI64, Ordering},
        Arc,
    },
};

use anyhow::Result;

#[derive(Debug)]
pub struct AmapMetrics {
    data: Arc<HashMap<&'static str, AtomicI64>>,
}

impl AmapMetrics {
    pub fn new(metric_names: &[&'static str]) -> Self {
        let map = metric_names
            .iter()
            .map(|&name| (name, AtomicI64::new(0)))
            .collect();
        AmapMetrics {
            data: Arc::new(map),
        }
    }

    pub fn inc(&self, name: impl AsRef<str>) -> Result<()> {
        let name = name.as_ref();
        let counter = self
            .data
            .get(name)
            .ok_or_else(|| anyhow::anyhow!("metric name {} not found", name))?;
        counter.fetch_add(1, Ordering::Relaxed);

        Ok(())
    }

    pub fn dec(&self, name: impl AsRef<str>) -> Result<()> {
        let name = name.as_ref();
        let counter = self
            .data
            .get(name)
            .ok_or_else(|| anyhow::anyhow!("metric name {} not found", name))?;
        counter.fetch_sub(1, Ordering::Relaxed);
        Ok(())
    }
}

impl Clone for AmapMetrics {
    fn clone(&self) -> Self {
        AmapMetrics {
            data: Arc::clone(&self.data),
        }
    }
}

impl fmt::Display for AmapMetrics {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for (name, counter) in self.data.iter() {
            s.push_str(&format!("{}: {}\n", name, counter.load(Ordering::Relaxed)));
        }
        write!(f, "{}", s)
    }
}
