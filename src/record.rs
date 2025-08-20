use std::time::Instant;

/// A pastebin data record.
#[derive(Clone, Debug)]
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
    pub fn created(&self) -> Instant {
        self.created
    }

    /// Return the content.
    pub fn into_content(self) -> Box<[u8]> {
        self.content
    }
}
