use crate::{
    entities::{GroupID, Payment, PaymentID},
    repositories::{FirestoreRepository, PaymentRepository, FIRESTORE_COLLECTION_PAYMENTS},
};
use async_trait::async_trait;
use firestore::path;
use itertools::Itertools;

#[async_trait]
impl PaymentRepository for FirestoreRepository {
    async fn create_payment(
        &self,
        payment: Payment,
    ) -> Result<Payment, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self
            .database
            .fluent()
            .insert()
            .into(FIRESTORE_COLLECTION_PAYMENTS)
            .document_id(&payment.id)
            .object(&payment)
            .execute()
            .await?)
    }

    async fn update_payment(
        &self,
        payment: Payment,
    ) -> Result<Payment, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self
            .database
            .fluent()
            .update()
            .in_col(FIRESTORE_COLLECTION_PAYMENTS)
            .document_id(&payment.id)
            .object(&payment)
            .execute()
            .await?)
    }

    async fn delete_payment(
        &self,
        id: &PaymentID,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(self
            .database
            .fluent()
            .delete()
            .from(FIRESTORE_COLLECTION_PAYMENTS)
            .document_id(id)
            .execute()
            .await?)
    }

    async fn get_payment(
        &self,
        id: &PaymentID,
    ) -> Result<Option<Payment>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self
            .database
            .fluent()
            .select()
            .by_id_in(FIRESTORE_COLLECTION_PAYMENTS)
            .obj()
            .one(&id)
            .await?)
    }

    async fn get_payments_by_group(
        &self,
        group: &GroupID,
    ) -> Result<Vec<Payment>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self
            .database
            .fluent()
            .select()
            .from(FIRESTORE_COLLECTION_PAYMENTS)
            .filter(|q| q.field(path!(Payment::group)).eq(group))
            .obj()
            .query()
            .await?
            .into_iter()
            .sorted()
            .collect())
    }
}
