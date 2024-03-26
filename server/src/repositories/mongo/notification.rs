use crate::{
    entities::{GroupID, Notification, NotificationID},
    repositories::{
        MongoRepository, MongoRepositoryError, NotificationRepository,
        MONGO_COLLECTION_NOTIFICATIONS,
    },
};
use async_trait::async_trait;
use futures::TryStreamExt;
use itertools::Itertools;
use mongodb::{
    bson::{doc, Bson},
    options::IndexOptions,
    Collection, IndexModel,
};

impl From<NotificationID> for Bson {
    fn from(value: NotificationID) -> Self {
        Bson::String(value.0.to_string())
    }
}

impl MongoRepository {
    pub async fn create_notification_index(&self) -> Result<(), MongoRepositoryError> {
        {
            let model = IndexModel::builder()
                .keys(doc! {"id": 1})
                .options(IndexOptions::builder().unique(true).build())
                .build();

            self.database
                .collection::<Notification>(MONGO_COLLECTION_NOTIFICATIONS)
                .create_index(model, None)
                .await?;

            Ok(())
        }
    }
}

#[async_trait]
impl NotificationRepository for MongoRepository {
    async fn create_notification(
        &self,
        notification: Notification,
    ) -> Result<Notification, Box<dyn std::error::Error + Send + Sync>> {
        let notifications: Collection<Notification> =
            self.database.collection(MONGO_COLLECTION_NOTIFICATIONS);
        let _ = notifications.insert_one(&notification, None).await?;
        Ok(notification)
    }

    async fn delete_notification(
        &self,
        id: &NotificationID,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let notifications: Collection<Notification> =
            self.database.collection(MONGO_COLLECTION_NOTIFICATIONS);
        let filter = doc! { "id": id };
        let _ = notifications.delete_one(filter, None).await?;
        Ok(())
    }

    async fn get_notification(
        &self,
        id: &NotificationID,
    ) -> Result<Option<Notification>, Box<dyn std::error::Error + Send + Sync>> {
        let notifications: Collection<Notification> =
            self.database.collection(MONGO_COLLECTION_NOTIFICATIONS);

        let filter = doc! { "id": id };
        let result = notifications.find_one(filter, None).await?;

        Ok(result)
    }

    async fn get_notifications_by_group(
        &self,
        group: &GroupID,
    ) -> Result<Vec<Notification>, Box<dyn std::error::Error + Send + Sync>> {
        let notifications: Collection<Notification> =
            self.database.collection(MONGO_COLLECTION_NOTIFICATIONS);

        let filter = doc! { "group": group };
        let result = notifications
            .find(filter, None)
            .await?
            .try_collect::<Vec<Notification>>()
            .await?
            .into_iter()
            .sorted()
            .collect();

        Ok(result)
    }
}
