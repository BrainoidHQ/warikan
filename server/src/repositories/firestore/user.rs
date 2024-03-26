use crate::{
    entities::{User, UserID},
    repositories::{FirestoreRepository, UserRepository, FIRESTORE_COLLECTION_USERS},
};
use async_trait::async_trait;

#[async_trait]
impl UserRepository for FirestoreRepository {
    async fn create_user(
        &self,
        user: User,
    ) -> Result<User, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self
            .database
            .fluent()
            .insert()
            .into(FIRESTORE_COLLECTION_USERS)
            .document_id(&user.id)
            .object(&user)
            .execute()
            .await?)
    }

    async fn update_user(
        &self,
        user: User,
    ) -> Result<User, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self
            .database
            .fluent()
            .update()
            .in_col(FIRESTORE_COLLECTION_USERS)
            .document_id(&user.id)
            .object(&user)
            .execute()
            .await?)
    }

    async fn delete_user(
        &self,
        id: &UserID,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(self
            .database
            .fluent()
            .delete()
            .from(FIRESTORE_COLLECTION_USERS)
            .document_id(id)
            .execute()
            .await?)
    }

    async fn get_user(
        &self,
        id: &UserID,
    ) -> Result<Option<User>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self
            .database
            .fluent()
            .select()
            .by_id_in(FIRESTORE_COLLECTION_USERS)
            .obj()
            .one(&id)
            .await?)
    }
}
