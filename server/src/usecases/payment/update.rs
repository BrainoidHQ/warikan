use crate::{
    entities::{Amount, AuthState, Payment, PaymentID, UserID},
    usecases::{UseCase, UseCaseError},
};
use async_graphql::InputObject;
use chrono::Utc;

#[cfg(test)]
use fake::Dummy;

#[derive(InputObject)]
#[cfg_attr(test, derive(Dummy))]
pub struct UpdatePaymentInput {
    pub id: PaymentID,
    pub title: Option<String>,
    pub creditors: Option<Vec<AmountInput>>,
    pub debtors: Option<Vec<AmountInput>>,
}

// https://github.com/async-graphql/async-graphql/issues/218
#[derive(InputObject)]
#[cfg_attr(test, derive(Dummy))]
pub struct AmountInput {
    pub user: UserID,
    pub amount: i32,
}

impl From<AmountInput> for Amount {
    fn from(input: AmountInput) -> Self {
        Amount {
            user: input.user,
            amount: input.amount,
        }
    }
}

impl UseCase {
    pub async fn update_payment(
        &self,
        auth: &AuthState,
        input: UpdatePaymentInput,
    ) -> Result<Payment, UseCaseError> {
        if let Some(payment) = self.get_payment(auth, &input.id).await? {
            let payment = Payment {
                id: input.id,
                created_at: payment.created_at,
                updated_at: Utc::now(),
                title: input.title.unwrap_or(payment.title),
                creditors: input
                    .creditors
                    .map(|v| v.into_iter().map(|a| a.into()).collect())
                    .unwrap_or(payment.creditors),
                debtors: input
                    .debtors
                    .map(|v| v.into_iter().map(|a| a.into()).collect())
                    .unwrap_or(payment.debtors),
                group: payment.group,
            };
            let payment = self
                .repository
                .update_payment(payment)
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
        entities::{Claims, Group},
        repositories::MockRepository,
    };
    use fake::{Fake, Faker};
    use std::sync::Arc;

    #[tokio::test]
    async fn update_payment_200() {
        let claims: Claims = Faker.fake();
        let mut group: Group = Faker.fake();
        group.participants.push(UserID::new(&claims.sub));
        let mut payment: Payment = Faker.fake();
        payment.group = group.id.clone();
        let mut input: UpdatePaymentInput = Faker.fake();
        input.id = payment.id.clone();
        let id = group.id.clone();

        let mut mock = MockRepository::new();
        mock.expect_get_group()
            .returning(move |_| Ok(Some(group.clone())));
        mock.expect_get_payment()
            .returning(move |_| Ok(Some(payment.clone())));
        mock.expect_update_payment()
            .returning(move |payment| Ok(payment));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let update = usecase.update_payment(&auth, input).await.unwrap();
        assert_eq!(update.group, id);
    }

    #[tokio::test]
    async fn update_payment_404() {
        let claims: Claims = Faker.fake();
        let input: UpdatePaymentInput = Faker.fake();

        let mut mock = MockRepository::new();
        mock.expect_get_payment().returning(move |_| Ok(None));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let update = usecase.update_payment(&auth, input).await;
        assert_eq!(update, Err(UseCaseError::NotFound));
    }

    #[tokio::test]
    async fn update_payment_500() {
        let claims: Claims = Faker.fake();
        let mut group: Group = Faker.fake();
        group.participants.push(UserID::new(&claims.sub));
        let mut payment: Payment = Faker.fake();
        payment.group = group.id.clone();
        let mut input: UpdatePaymentInput = Faker.fake();
        input.id = payment.id.clone();

        let mut mock = MockRepository::new();
        mock.expect_get_group()
            .returning(move |_| Ok(Some(group.clone())));
        mock.expect_get_payment()
            .returning(move |_| Ok(Some(payment.clone())));
        mock.expect_update_payment()
            .returning(move |_| Err(Box::new(UseCaseError::InternalServerError)));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let update = usecase.update_payment(&auth, input).await;
        assert_eq!(update, Err(UseCaseError::InternalServerError));
    }
}
