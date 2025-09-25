use std::ops::Deref;
use std::time::Instant;

/// A pastebin data record.
#[derive(Debug)]
pub struct Record {
    created: Instant,
    content: Box<[u8]>,
}

impl Record {
    /// Create a new record.
    pub fn new(content: Box<[u8]>) -> Self {
        Self {
            created: Instant::now(),
            content,
        }
    }

    /// Return the instant when the record was crated.
    pub const fn created(&self) -> Instant {
        self.created
    }
}

impl Deref for Record {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}
