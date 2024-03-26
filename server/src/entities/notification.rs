use crate::entities::{GroupID, Node};
use async_graphql::{types::ID, NewType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[cfg(test)]
use fake::{Dummy, Faker};
#[cfg(test)]
use rand::Rng;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, NewType)]
pub struct NotificationID(pub ID);

impl NotificationID {
    pub fn new<T: ToString>(id: T) -> Self {
        NotificationID(ID(id.to_string()))
    }
}

impl ToString for NotificationID {
    fn to_string(&self) -> String {
        self.0 .0.to_string()
    }
}

impl AsRef<str> for NotificationID {
    fn as_ref(&self) -> &str {
        &self.0 .0
    }
}

#[cfg(test)]
impl Dummy<Faker> for NotificationID {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        let id = String::dummy_with_rng(config, rng);
        NotificationID::new(id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(Dummy))]
pub struct Notification {
    pub id: NotificationID,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub message: String,

    pub group: GroupID,
}

impl<'a> Node<'a> for Notification {
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

impl<'a> PartialOrd for Notification {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let id_order = self.id.partial_cmp(&other.id);
        let updated_at_order = self.updated_at.partial_cmp(&other.updated_at);

        match updated_at_order {
            None => id_order,
            Some(Ordering::Equal) => id_order,
            _ => updated_at_order,
        }
    }
}

impl<'a> Ord for Notification {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let id_order = self.id.cmp(&other.id);
        let updated_at_order = self.updated_at.cmp(&other.updated_at);

        match updated_at_order {
            Ordering::Equal => id_order,
            _ => updated_at_order,
        }
    }
}
