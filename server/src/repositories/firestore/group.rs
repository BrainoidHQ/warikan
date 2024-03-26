use crate::{
    entities::{Group, GroupID, UserID},
    repositories::{FirestoreRepository, GroupRepository, FIRESTORE_COLLECTION_GROUPS},
};
use async_trait::async_trait;
use firestore::path;
use itertools::Itertools;

#[async_trait]
impl GroupRepository for FirestoreRepository {
    async fn create_group(
        &self,
        group: Group,
    ) -> Result<Group, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self
            .database
            .fluent()
            .insert()
            .into(FIRESTORE_COLLECTION_GROUPS)
            .document_id(&group.id)
            .object(&group)
            .execute()
            .await?)
    }

    async fn update_group(
        &self,
        group: Group,
    ) -> Result<Group, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self
            .database
            .fluent()
            .update()
            .in_col(FIRESTORE_COLLECTION_GROUPS)
            .document_id(&group.id)
            .object(&group)
            .execute()
            .await?)
    }

    async fn delete_group(
        &self,
        id: &GroupID,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(self
            .database
            .fluent()
            .delete()
            .from(FIRESTORE_COLLECTION_GROUPS)
            .document_id(id)
            .execute()
            .await?)
    }

    async fn get_group(
        &self,
        id: &GroupID,
    ) -> Result<Option<Group>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self
            .database
            .fluent()
            .select()
            .by_id_in(FIRESTORE_COLLECTION_GROUPS)
            .obj()
            .one(&id)
            .await?)
    }

    async fn get_groups_by_user(
        &self,
        id: &UserID,
    ) -> Result<Vec<Group>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self
            .database
            .fluent()
            .select()
            .from(FIRESTORE_COLLECTION_GROUPS)
            .filter(|q| q.field(path!(Group::participants)).array_contains(id))
            .obj()
            .query()
            .await?
            .into_iter()
            .sorted()
            .collect())
    }
}
