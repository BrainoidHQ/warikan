use crate::entities::{Node, UnreachableError};
use async_graphql::{types::ID, NewType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, str::FromStr};

#[cfg(test)]
use fake::{Dummy, Faker};
#[cfg(test)]
use rand::Rng;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, NewType)]
pub struct UserID(pub ID);

impl UserID {
    pub fn new<T: ToString>(id: T) -> Self {
        UserID(ID(id.to_string()))
    }
}

impl ToString for UserID {
    fn to_string(&self) -> String {
        self.0 .0.to_string()
    }
}

impl AsRef<str> for UserID {
    fn as_ref(&self) -> &str {
        &self.0 .0
    }
}

impl FromStr for UserID {
    type Err = UnreachableError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(UserID::new(s))
    }
}

#[cfg(test)]
impl Dummy<Faker> for UserID {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        let id = String::dummy_with_rng(config, rng);
        UserID::new(id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(Dummy))]
pub struct User {
    pub id: UserID,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub name: String,
}

impl<'a> Node<'a> for User {
    fn id(&self) -> &str {
        self.id.as_ref()
    }

    fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}

impl PartialOrd for User {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for User {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let id_order = self.id.cmp(&other.id);
        let updated_at_order = self.updated_at.cmp(&other.updated_at);

        match updated_at_order {
            Ordering::Equal => id_order,
            _ => updated_at_order,
        }
    }
}
