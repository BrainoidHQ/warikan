mod group;
mod notification;
mod payment;
mod user;

use mongodb::{Client, Database};
use thiserror::Error;

pub const MONGO_COLLECTION_GROUPS: &str = "groups";
pub const MONGO_COLLECTION_NOTIFICATIONS: &str = "notifications";
pub const MONGO_COLLECTION_PAYMENTS: &str = "payments";
pub const MONGO_COLLECTION_USERS: &str = "users";

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::{
        GroupRepositoryTester, NotificationRepositoryTester, PaymentRepositoryTester,
        UserRepositoryTester,
    };

    #[tokio::test]
    async fn test_mongo_group_repository() {
        GroupRepositoryTester::test(
            MongoRepository::new(MongoRepositoryConfig {
                uri: "mongodb://localhost:27017",
                database: "warikan",
            })
            .await
            .unwrap(),
        )
        .await;
    }

    #[tokio::test]
    async fn test_mongo_notification_repository() {
        NotificationRepositoryTester::test(
            MongoRepository::new(MongoRepositoryConfig {
                uri: "mongodb://localhost:27017",
                database: "warikan",
            })
            .await
            .unwrap(),
        )
        .await;
    }

    #[tokio::test]
    async fn test_mongo_payment_repository() {
        PaymentRepositoryTester::test(
            MongoRepository::new(MongoRepositoryConfig {
                uri: "mongodb://localhost:27017",
                database: "warikan",
            })
            .await
            .unwrap(),
        )
        .await;
    }

    #[tokio::test]
    async fn test_mongo_user_repository() {
        UserRepositoryTester::test(
            MongoRepository::new(MongoRepositoryConfig {
                uri: "mongodb://localhost:27017",
                database: "warikan",
            })
            .await
            .unwrap(),
        )
        .await;
    }
}
