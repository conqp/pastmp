use std::fs::OpenOptions;
use std::path::Path;

use argon2::Argon2;

use crate::accounts::Accounts;
use crate::records::Records;

#[derive(Debug)]
pub struct Settings {
    pub(crate) accounts: Accounts,
    pub(crate) records: Records,
    pub(crate) hasher: Argon2<'static>,
}

impl Settings {
    pub fn load(file: impl AsRef<Path>) -> anyhow::Result<Self> {
        Ok(Self {
            accounts: serde_json::from_reader(OpenOptions::new().read(true).open(file)?)?,
            records: Records::default(),
            hasher: Argon2::default(),
        })
    }
}
