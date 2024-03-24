use crate::{
    entities::{GroupID, Payment, PaymentID},
    repositories::{
        MongoRepository, MongoRepositoryError, PaymentRepository, MONGO_COLLECTION_PAYMENTS,
    },
};
use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, Bson},
    options::IndexOptions,
    Collection, IndexModel,
};

impl From<PaymentID> for Bson {
    fn from(value: PaymentID) -> Self {
        Bson::String(value.0.to_string())
    }
}

impl MongoRepository {
    pub async fn create_payment_index(&self) -> Result<(), MongoRepositoryError> {
        {
            let model = IndexModel::builder()
                .keys(doc! {"id": 1})
                .options(IndexOptions::builder().unique(true).build())
                .build();

            self.database
                .collection::<Payment>(MONGO_COLLECTION_PAYMENTS)
                .create_index(model, None)
                .await?;

            Ok(())
        }
    }
}

#[async_trait]
impl PaymentRepository for MongoRepository {
    async fn create_payment(
        &self,
        payment: Payment,
    ) -> Result<Payment, Box<dyn std::error::Error + Send + Sync>> {
        let payments: Collection<Payment> = self.database.collection(MONGO_COLLECTION_PAYMENTS);
        let _ = payments.insert_one(&payment, None).await?;
        Ok(payment)
    }

    async fn update_payment(
        &self,
        payment: Payment,
    ) -> Result<Payment, Box<dyn std::error::Error + Send + Sync>> {
        let payments: Collection<Payment> = self.database.collection(MONGO_COLLECTION_PAYMENTS);
        let filter = doc! { "id": &payment.id };
        let _ = payments.replace_one(filter, &payment, None).await?;
        Ok(payment)
    }

    async fn delete_payment(
        &self,
        id: &PaymentID,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let payments: Collection<Payment> = self.database.collection(MONGO_COLLECTION_PAYMENTS);

        let filter = doc! { "id": id };
        let result = payments.delete_one(filter, None).await?;

        assert!(result.deleted_count == 1);
        Ok(())
    }

    async fn get_payment(
        &self,
        id: &PaymentID,
    ) -> Result<Option<Payment>, Box<dyn std::error::Error + Send + Sync>> {
        let payments: Collection<Payment> = self.database.collection(MONGO_COLLECTION_PAYMENTS);

        let filter = doc! { "id": id };
        let result = payments.find_one(filter, None).await?;

        Ok(result)
    }

    async fn get_payments_by_group(
        &self,
        group: &GroupID,
    ) -> Result<Vec<Payment>, Box<dyn std::error::Error + Send + Sync>> {
        let payments: Collection<Payment> = self.database.collection(MONGO_COLLECTION_PAYMENTS);

        let filter = doc! { "group": group };
        let result = payments.find(filter, None).await?.try_collect().await?;

        Ok(result)
    }
}
