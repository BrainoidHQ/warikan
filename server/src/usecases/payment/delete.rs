use crate::{
    entities::{AuthState, PaymentID},
    usecases::{UseCase, UseCaseError},
};
use async_graphql::InputObject;

#[cfg(test)]
use fake::Dummy;

#[derive(InputObject)]
#[cfg_attr(test, derive(Dummy))]
pub struct DeletePaymentInput {
    pub id: PaymentID,
}

impl UseCase {
    pub async fn delete_payment(
        &self,
        auth: &AuthState,
        input: DeletePaymentInput,
    ) -> Result<PaymentID, UseCaseError> {
        if let Some(payment) = self.get_payment(auth, &input.id).await? {
            self.repository
                .delete_payment(&payment.id)
                .await
                .or(Err(UseCaseError::InternalServerError))?;
            Ok(payment.id)
        } else {
            Err(UseCaseError::NotFound)?
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        entities::{Claims, Group, Payment, UserID},
        repositories::MockRepository,
    };
    use fake::{Fake, Faker};
    use std::sync::Arc;

    #[tokio::test]
    async fn delete_payment_200() {
        let claims: Claims = Faker.fake();
        let mut group: Group = Faker.fake();
        group.participants.push(UserID::new(&claims.sub));
        let mut payment: Payment = Faker.fake();
        payment.group = group.id.clone();
        let mut input: DeletePaymentInput = Faker.fake();
        input.id = payment.id.clone();
        let id = payment.id.clone();

        let mut mock = MockRepository::new();
        mock.expect_get_group()
            .returning(move |_| Ok(Some(group.clone())));
        mock.expect_get_payment()
            .returning(move |_| Ok(Some(payment.clone())));
        mock.expect_delete_payment().returning(move |_| Ok(()));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let delete = usecase.delete_payment(&auth, input).await.unwrap();
        assert_eq!(delete, id);
    }

    #[tokio::test]
    async fn delete_payment_404() {
        let claims: Claims = Faker.fake();
        let input: DeletePaymentInput = Faker.fake();

        let mut mock = MockRepository::new();
        mock.expect_get_payment().returning(move |_| Ok(None));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let delete = usecase.delete_payment(&auth, input).await;
        assert_eq!(delete, Err(UseCaseError::NotFound));
    }

    #[tokio::test]
    async fn delete_payment_500() {
        let claims: Claims = Faker.fake();
        let mut group: Group = Faker.fake();
        group.participants.push(UserID::new(&claims.sub));
        let mut payment: Payment = Faker.fake();
        payment.group = group.id.clone();
        let mut input: DeletePaymentInput = Faker.fake();
        input.id = payment.id.clone();

        let mut mock = MockRepository::new();
        mock.expect_get_group()
            .returning(move |_| Ok(Some(group.clone())));
        mock.expect_get_payment()
            .returning(move |_| Ok(Some(payment.clone())));
        mock.expect_delete_payment()
            .returning(move |_| Err(Box::new(UseCaseError::InternalServerError)));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let delete = usecase.delete_payment(&auth, input).await;
        assert_eq!(delete, Err(UseCaseError::InternalServerError));
    }
}
