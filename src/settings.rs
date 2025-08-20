use std::collections::BTreeMap;
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
        let file = OpenOptions::new().read(true).open(file)?;
        let accounts: BTreeMap<String, String> = serde_json::from_reader(file)?;
        Ok(Self {
            accounts: Accounts::try_from(accounts)?,
            records: Records::default(),
            hasher: Argon2::default(),
        })
    }
}
