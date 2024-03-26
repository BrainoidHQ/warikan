use crate::entities::{GroupID, Node, UserID};
use async_graphql::{types::ID, NewType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[cfg(test)]
use fake::{Dummy, Faker};
#[cfg(test)]
use rand::Rng;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, NewType)]
pub struct PaymentID(pub ID);

impl PaymentID {
    pub fn new<T: ToString>(id: T) -> Self {
        PaymentID(ID(id.to_string()))
    }
}

impl ToString for PaymentID {
    fn to_string(&self) -> String {
        self.0 .0.to_string()
    }
}

impl AsRef<str> for PaymentID {
    fn as_ref(&self) -> &str {
        &self.0 .0
    }
}

#[cfg(test)]
impl Dummy<Faker> for PaymentID {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        let id = String::dummy_with_rng(config, rng);
        PaymentID::new(id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(Dummy))]
pub struct Payment {
    pub id: PaymentID,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub title: String,
    pub creditors: Vec<Amount>,
    pub debtors: Vec<Amount>,

    pub group: GroupID,
}

impl<'a> Node<'a> for Payment {
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

impl<'a> PartialOrd for Payment {
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

impl<'a> Ord for Payment {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let id_order = self.id.cmp(&other.id);
        let updated_at_order = self.updated_at.cmp(&other.updated_at);

        match updated_at_order {
            Ordering::Equal => id_order,
            _ => updated_at_order,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(Dummy))]
pub struct Amount {
    pub user: UserID,
    pub amount: i32,
}
