use crate::{
    entities::{AuthState, GroupID, Payment, PaymentID},
    usecases::{UseCase, UseCaseError},
};
use async_graphql::InputObject;
use chrono::Utc;
use nanoid::nanoid;

#[cfg(test)]
use fake::Dummy;

#[derive(InputObject)]
#[cfg_attr(test, derive(Dummy))]
pub struct CreatePaymentInput {
    pub title: String,
    pub group: GroupID,
}

impl UseCase {
    pub async fn create_payment(
        &self,
        auth: &AuthState,
        input: CreatePaymentInput,
    ) -> Result<Payment, UseCaseError> {
        if self.get_group(auth, &input.group).await?.is_some() {
            let now = Utc::now();
            let payment = Payment {
                id: PaymentID::new(nanoid!()),
                created_at: now,
                updated_at: now,
                title: input.title,
                creditors: Vec::new(),
                debtors: Vec::new(),
                group: input.group,
            };
            let payment = self
                .repository
                .create_payment(payment)
                .await
                .or(Err(UseCaseError::InternalServerError))?;
            Ok(payment)
        } else {
            Err(UseCaseError::NotFound)?
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        entities::{Claims, Group, UserID},
        repositories::MockRepository,
    };
    use fake::{Fake, Faker};
    use std::sync::Arc;

    #[tokio::test]
    async fn create_payment_200() {
        let claims: Claims = Faker.fake();
        let mut group: Group = Faker.fake();
        group.participants.push(UserID::new(&claims.sub));
        let mut input: CreatePaymentInput = Faker.fake();
        input.group = group.id.clone();
        let id = group.id.clone();

        let mut mock = MockRepository::new();
        mock.expect_get_group()
            .returning(move |_| Ok(Some(group.clone())));
        mock.expect_create_payment()
            .returning(move |payment| Ok(payment));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let create = usecase.create_payment(&auth, input).await.unwrap();
        assert_eq!(create.group, id);
    }

    #[tokio::test]
    async fn create_payment_404() {
        let claims: Claims = Faker.fake();
        let input: CreatePaymentInput = Faker.fake();

        let mut mock = MockRepository::new();
        mock.expect_get_group().returning(move |_| Ok(None));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let create = usecase.create_payment(&auth, input).await;
        assert_eq!(create, Err(UseCaseError::NotFound));
    }

    #[tokio::test]
    async fn create_payment_500() {
        let claims: Claims = Faker.fake();
        let mut group: Group = Faker.fake();
        group.participants.push(UserID::new(&claims.sub));
        let mut input: CreatePaymentInput = Faker.fake();
        input.group = group.id.clone();

        let mut mock = MockRepository::new();
        mock.expect_get_group()
            .returning(move |_| Ok(Some(group.clone())));
        mock.expect_create_payment()
            .returning(move |_| Err(Box::new(UseCaseError::InternalServerError)));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let create = usecase.create_payment(&auth, input).await;
        assert_eq!(create, Err(UseCaseError::InternalServerError));
    }
}
