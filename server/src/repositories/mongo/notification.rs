use crate::{
    entities::{GroupID, Notification, NotificationID},
    repositories::{
        MongoRepository, MongoRepositoryError, NotificationRepository,
        MONGO_COLLECTION_NOTIFICATIONS,
    },
};
use async_trait::async_trait;
use futures::TryStreamExt;
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
            .try_collect()
            .await?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::MongoRepositoryConfig;
    use fake::{Fake, Faker};

    #[tokio::test]
    async fn create_notification() {
        let mongo = MongoRepository::new(MongoRepositoryConfig {
            uri: "mongodb://localhost:27017",
            database: "warikan",
        })
        .await
        .unwrap();

        let notification: Notification = Faker.fake();

        let create = mongo.create_notification(notification).await.unwrap();
        let get = mongo.get_notification(&create.id).await.unwrap();

        assert_eq!(Some(create), get);
    }

    #[tokio::test]
    async fn delete_notification() {
        let mongo = MongoRepository::new(MongoRepositoryConfig {
            uri: "mongodb://localhost:27017",
            database: "warikan",
        })
        .await
        .unwrap();

        let notification: Notification = Faker.fake();

        let create = mongo.create_notification(notification).await.unwrap();
        mongo.delete_notification(&create.id).await.unwrap();
        let delete = mongo.get_notification(&create.id).await.unwrap();

        assert_eq!(delete, None);
    }

    #[tokio::test]
    async fn get_notifications_by_group() {
        let mongo = MongoRepository::new(MongoRepositoryConfig {
            uri: "mongodb://localhost:27017",
            database: "warikan",
        })
        .await
        .unwrap();

        let mut notification1: Notification = Faker.fake();
        let mut notification2: Notification = Faker.fake();
        let notification3: Notification = Faker.fake();

        let group: GroupID = Faker.fake();
        notification1.group = group.clone();
        notification2.group = group.clone();

        mongo
            .create_notification(notification1.clone())
            .await
            .unwrap();
        mongo
            .create_notification(notification2.clone())
            .await
            .unwrap();
        mongo
            .create_notification(notification3.clone())
            .await
            .unwrap();

        let get = mongo.get_notifications_by_group(&group).await.unwrap();

        assert_eq!(vec![notification1, notification2], get);
    }
}
