use std::collections::BTreeMap;
use std::ops::Deref;

use argon2::password_hash;
use argon2::password_hash::PasswordHashString;
use rocket::serde::de::Error;
use rocket::serde::{Deserialize, Deserializer};

type Inner = BTreeMap<String, PasswordHashString>;

#[derive(Clone, Debug)]
pub struct Accounts(Inner);

impl Deref for Accounts {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<BTreeMap<String, String>> for Accounts {
    type Error = password_hash::Error;

    fn try_from(map: BTreeMap<String, String>) -> Result<Self, Self::Error> {
        let mut inner = Inner::new();

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
        Self::try_from(BTreeMap::<String, String>::deserialize(deserializer)?)
            .map_err(D::Error::custom)
    }
}
