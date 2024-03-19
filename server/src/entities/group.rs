use crate::entities::UserID;
use async_graphql::{types::ID, NewType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(test)]
use fake::{Dummy, Faker};
#[cfg(test)]
use rand::Rng;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, NewType)]
pub struct GroupID(pub ID);

impl GroupID {
    pub fn new<T: ToString>(id: T) -> Self {
        GroupID(ID(id.to_string()))
    }
}

impl ToString for GroupID {
    fn to_string(&self) -> String {
        self.0 .0.to_string()
    }
}

#[cfg(test)]
impl Dummy<Faker> for GroupID {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        let id = String::dummy_with_rng(config, rng);
        GroupID::new(id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(Dummy))]
pub struct Group {
    pub id: GroupID,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub title: String,
    pub participants: Vec<UserID>,
}
