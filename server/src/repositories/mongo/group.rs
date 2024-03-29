use crate::{
    entities::{Group, GroupID, UserID},
    repositories::{
        GroupRepository, MongoRepository, MongoRepositoryError, MONGO_COLLECTION_GROUPS,
    },
};
use async_trait::async_trait;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, Bson},
    options::IndexOptions,
    Collection, IndexModel,
};

impl From<GroupID> for Bson {
    fn from(value: GroupID) -> Self {
        Bson::String(value.0.to_string())
    }
}

impl MongoRepository {
    pub async fn create_group_index(&self) -> Result<(), MongoRepositoryError> {
        {
            let model = IndexModel::builder()
                .keys(doc! {"id": 1})
                .options(IndexOptions::builder().unique(true).build())
                .build();

            self.database
                .collection::<Group>(MONGO_COLLECTION_GROUPS)
                .create_index(model, None)
                .await?;

            Ok(())
        }
    }
}

#[async_trait]
impl GroupRepository for MongoRepository {
    async fn create_group(
        &self,
        group: Group,
    ) -> Result<Group, Box<dyn std::error::Error + Send + Sync>> {
        let groups: Collection<Group> = self.database.collection(MONGO_COLLECTION_GROUPS);
        let _ = groups.insert_one(&group, None).await?;
        Ok(group)
    }

    async fn update_group(
        &self,
        group: Group,
    ) -> Result<Group, Box<dyn std::error::Error + Send + Sync>> {
        let groups: Collection<Group> = self.database.collection(MONGO_COLLECTION_GROUPS);
        let filter = doc! { "id": &group.id };
        let _ = groups.replace_one(filter, &group, None).await?;
        Ok(group)
    }

    async fn delete_group(
        &self,
        id: &GroupID,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let groups: Collection<Group> = self.database.collection(MONGO_COLLECTION_GROUPS);

        let filter = doc! { "id": id };
        let result = groups.delete_one(filter, None).await?;

        assert!(result.deleted_count == 1);
        Ok(())
    }

    async fn get_group(
        &self,
        id: &GroupID,
    ) -> Result<Option<Group>, Box<dyn std::error::Error + Send + Sync>> {
        let groups: Collection<Group> = self.database.collection(MONGO_COLLECTION_GROUPS);

        let filter = doc! { "id": id };
        let result = groups.find_one(filter, None).await?;

        Ok(result)
    }

    async fn get_groups_by_user(
        &self,
        id: &UserID,
    ) -> Result<Vec<Group>, Box<dyn std::error::Error + Send + Sync>> {
        let groups: Collection<Group> = self.database.collection(MONGO_COLLECTION_GROUPS);

        let filter = doc! { "participants": id };
        let result = groups.find(filter, None).await?.try_collect().await?;

        Ok(result)
    }
}
