use std::collections::BTreeMap;

use argon2::password_hash;
use argon2::password_hash::PasswordHashString;
use rocket::serde::de::Error;
use rocket::serde::{Deserialize, Deserializer};

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

impl<'de> Deserialize<'de> for Accounts {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Accounts::try_from(BTreeMap::<String, String>::deserialize(deserializer)?)
            .map_err(D::Error::custom)
    }
}
