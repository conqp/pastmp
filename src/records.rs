use std::collections::BTreeMap;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::{PoisonError, RwLock};
use std::time::Duration;

use crate::Record;

const MAX_RETENTION_TIME: Duration = Duration::from_secs(60 * 60 * 24); // One day.

/// A collection of records.
#[derive(Debug, Default)]
pub struct Records {
    counter: AtomicUsize,
    records: RwLock<BTreeMap<usize, Record>>,
}

impl Records {
    /// Insert a new record.
    #[must_use]
    pub fn insert(&self, bytes: Box<[u8]>) -> usize {
        let id = self.next_id();
        self.records
            .write()
            .unwrap_or_else(PoisonError::into_inner)
            .insert(id, Record::new(bytes));
        id
    }

    /// Get the respective record.
    #[must_use]
    pub fn get(&self, id: usize) -> Option<Record> {
        self.records
            .read()
            .unwrap_or_else(PoisonError::into_inner)
            .get(&id)
            .cloned()
    }

    /// Remove the respective record.
    pub fn remove(&self, id: usize) -> Option<Record> {
        self.records
            .write()
            .unwrap_or_else(PoisonError::into_inner)
            .remove(&id)
    }

    /// Remove old entries.
    pub fn remove_old_entries(&self) {
        self.records
            .write()
            .unwrap_or_else(PoisonError::into_inner)
            .retain(|_, record| record.created().elapsed() < MAX_RETENTION_TIME);
    }

    /// Return the next ID to assign and increase the ID.
    fn next_id(&self) -> usize {
        self.counter.fetch_add(1, SeqCst)
    }
}
