use crate::{
    entities::{GroupID, Notification, NotificationID},
    repositories::{
        FirestoreRepository, NotificationRepository, FIRESTORE_COLLECTION_NOTIFICATIONS,
    },
};
use async_trait::async_trait;
use firestore::path;
use itertools::Itertools;

#[async_trait]
impl NotificationRepository for FirestoreRepository {
    async fn create_notification(
        &self,
        notification: Notification,
    ) -> Result<Notification, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self
            .database
            .fluent()
            .insert()
            .into(FIRESTORE_COLLECTION_NOTIFICATIONS)
            .document_id(&notification.id)
            .object(&notification)
            .execute()
            .await?)
    }

    async fn delete_notification(
        &self,
        id: &NotificationID,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(self
            .database
            .fluent()
            .delete()
            .from(FIRESTORE_COLLECTION_NOTIFICATIONS)
            .document_id(id)
            .execute()
            .await?)
    }

    async fn get_notification(
        &self,
        id: &NotificationID,
    ) -> Result<Option<Notification>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self
            .database
            .fluent()
            .select()
            .by_id_in(FIRESTORE_COLLECTION_NOTIFICATIONS)
            .obj()
            .one(&id)
            .await?)
    }

    async fn get_notifications_by_group(
        &self,
        group: &GroupID,
    ) -> Result<Vec<Notification>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self
            .database
            .fluent()
            .select()
            .from(FIRESTORE_COLLECTION_NOTIFICATIONS)
            .filter(|q| q.field(path!(Notification::group)).eq(group))
            .obj()
            .query()
            .await?
            .into_iter()
            .sorted()
            .collect())
    }
}
