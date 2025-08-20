use std::collections::BTreeMap;

use argon2::password_hash;
use argon2::password_hash::PasswordHashString;

#[derive(Clone, Debug)]
pub struct Accounts(BTreeMap<String, PasswordHashString>);

impl Accounts {
    /// Return the respective account's password hash.
    pub fn get(&self, name: &str) -> Option<&PasswordHashString> {
        self.0.get(name)
    }
}

impl TryFrom<BTreeMap<String, String>> for Accounts {
    type Error = password_hash::Error;

    fn try_from(map: BTreeMap<String, String>) -> Result<Self, Self::Error> {
        let mut inner = BTreeMap::new();

        for (key, value) in map {
            inner.insert(key, PasswordHashString::new(&value)?);
        }

        Ok(Self(inner))
    }
}
