use crate::entities::UnreachableError;
use async_graphql::{types::ID, NewType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

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
