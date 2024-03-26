mod group;
mod notification;
mod payment;
mod user;

use firestore::FirestoreDb;
use thiserror::Error;

pub const FIRESTORE_COLLECTION_GROUPS: &str = "groups";
pub const FIRESTORE_COLLECTION_NOTIFICATIONS: &str = "notifications";
pub const FIRESTORE_COLLECTION_PAYMENTS: &str = "payments";
pub const FIRESTORE_COLLECTION_USERS: &str = "users";

#[derive(Debug)]
pub struct FirestoreRepository {
    pub database: FirestoreDb,
}

#[derive(Debug, Error)]
pub enum FirestoreRepositoryError {
    #[error("firestore error")]
    Firestore(#[from] firestore::errors::FirestoreError),
}

#[derive(Debug)]
pub struct FirestoreRepositoryConfig<'a> {
    pub project_id: &'a str,
}

impl FirestoreRepository {
    pub async fn new(
        config: FirestoreRepositoryConfig<'_>,
    ) -> Result<Self, FirestoreRepositoryError> {
        let database = FirestoreDb::new(config.project_id).await?;
        let firestore = FirestoreRepository { database };
        Ok(firestore)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::{
        FirestoreRepositoryConfig, GroupRepositoryTester, NotificationRepositoryTester,
        PaymentRepositoryTester, UserRepositoryTester,
    };

    #[tokio::test]
    async fn test_firestore_group_repository() {
        GroupRepositoryTester::test(
            FirestoreRepository::new(FirestoreRepositoryConfig {
                project_id: "yoseio-warikan",
            })
            .await
            .unwrap(),
        )
        .await;
    }

    #[tokio::test]
    async fn test_firestore_notification_repository() {
        NotificationRepositoryTester::test(
            FirestoreRepository::new(FirestoreRepositoryConfig {
                project_id: "yoseio-warikan",
            })
            .await
            .unwrap(),
        )
        .await;
    }

    #[tokio::test]
    async fn test_firestore_payment_repository() {
        PaymentRepositoryTester::test(
            FirestoreRepository::new(FirestoreRepositoryConfig {
                project_id: "yoseio-warikan",
            })
            .await
            .unwrap(),
        )
        .await;
    }

    #[tokio::test]
    async fn test_firestore_user_repository() {
        UserRepositoryTester::test(
            FirestoreRepository::new(FirestoreRepositoryConfig {
                project_id: "yoseio-warikan",
            })
            .await
            .unwrap(),
        )
        .await;
    }
}
