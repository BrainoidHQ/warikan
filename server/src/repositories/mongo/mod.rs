mod group;
mod notification;
mod payment;
mod user;

use crate::repositories::Repository;
use mongodb::{Client, Database};
use shaku::Component;
use thiserror::Error;

pub const MONGO_COLLECTION_GROUPS: &str = "groups";
pub const MONGO_COLLECTION_NOTIFICATIONS: &str = "notifications";
pub const MONGO_COLLECTION_PAYMENTS: &str = "payments";
pub const MONGO_COLLECTION_USERS: &str = "users";

#[derive(Debug, Component)]
#[shaku(interface = Repository)]
pub struct MongoRepository {
    pub database: Database,
}

#[derive(Debug, Error)]
pub enum MongoRepositoryError {
    #[error("mongodb error")]
    Mongo(#[from] mongodb::error::Error),
}

#[derive(Debug)]
pub struct MongoRepositoryConfig<'a> {
    pub uri: &'a str,
    pub database: &'a str,
}

impl MongoRepository {
    pub async fn new(config: MongoRepositoryConfig<'_>) -> Result<Self, MongoRepositoryError> {
        let client = Client::with_uri_str(config.uri).await?;
        let database = client.database(config.database);
        let mongo = MongoRepository { database };
        mongo.create_index().await?;
        Ok(mongo)
    }

    pub async fn create_index(&self) -> Result<(), MongoRepositoryError> {
        self.create_group_index().await?;
        self.create_notification_index().await?;
        self.create_payment_index().await?;
        self.create_user_index().await?;

        Ok(())
    }
}
