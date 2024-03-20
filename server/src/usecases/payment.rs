use crate::{
    entities::{warikan, Amount, AuthState, GroupID, Payment, PaymentID, UserID, Warikan},
    usecases::{UseCase, UseCaseError},
};
use async_graphql::InputObject;
use chrono::Utc;
use nanoid::nanoid;

#[cfg(test)]
use fake::Dummy;

impl UseCase {
    pub async fn get_payment(
        &self,
        auth: &AuthState,
        id: &PaymentID,
    ) -> Result<Option<Payment>, UseCaseError> {
        if let Some(payment) = self
            .repository
            .get_payment(id)
            .await
            .or(Err(UseCaseError::InternalServerError))?
        {
            let payment = self
                .get_group(auth, &payment.group)
                .await?
                .and(Some(payment));
            Ok(payment)
        } else {
            Ok(None)
        }
    }

    pub async fn get_payments_by_group(
        &self,
        auth: &AuthState,
        id: &GroupID,
    ) -> Result<Vec<Payment>, UseCaseError> {
        if self.get_group(auth, id).await?.is_some() {
            let payments = self
                .repository
                .get_payments_by_group(id)
                .await
                .or(Err(UseCaseError::InternalServerError))?;
            Ok(payments)
        } else {
            Err(UseCaseError::NotFound)?
        }
    }

    pub async fn warikan_by_group(
        &self,
        auth: &AuthState,
        id: &GroupID,
    ) -> Result<Vec<Warikan>, UseCaseError> {
        let payments = self.get_payments_by_group(auth, id).await?;
        warikan(&payments).ok_or(UseCaseError::BadRequest)
    }
}

#[derive(InputObject)]
#[cfg_attr(test, derive(Dummy))]
pub struct CreatePaymentInput {
    pub title: String,
    pub group: GroupID,
}

#[derive(InputObject)]
#[cfg_attr(test, derive(Dummy))]
pub struct UpdatePaymentInput {
    pub id: PaymentID,
    pub title: Option<String>,
    pub creditors: Option<Vec<AmountInput>>,
    pub debtors: Option<Vec<AmountInput>>,
}

#[derive(InputObject)]
#[cfg_attr(test, derive(Dummy))]
pub struct DeletePaymentInput {
    pub id: PaymentID,
}

// https://github.com/async-graphql/async-graphql/issues/218
#[derive(InputObject)]
#[cfg_attr(test, derive(Dummy))]
pub struct AmountInput {
    pub user: UserID,
    pub amount: i32,
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

impl From<AmountInput> for Amount {
    fn from(input: AmountInput) -> Self {
        Amount {
            user: input.user,
            amount: input.amount,
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

    // -------------------------------------------------------------------------
    //  get_payment
    // -------------------------------------------------------------------------

    #[tokio::test]
    async fn get_payment_200() {
        let claims: Claims = Faker.fake();
        let payment: Payment = Faker.fake();
        let mut group: Group = Faker.fake();
        group.id = payment.group.clone();
        group.participants.push(UserID::new(&claims.sub));
        let id = payment.id.clone();

        let mut mock = MockRepository::new();
        mock.expect_get_group()
            .returning(move |_| Ok(Some(group.clone())));
        mock.expect_get_payment()
            .returning(move |_| Ok(Some(payment.clone())));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let get = usecase.get_payment(&auth, &id).await.unwrap();
        assert_eq!(get.map(|g| g.id), Some(id))
    }

    #[tokio::test]
    async fn get_payment_500() {
        let claims: Claims = Faker.fake();
        let id: PaymentID = Faker.fake();

        let mut mock = MockRepository::new();
        mock.expect_get_payment()
            .returning(move |_| Err(Box::new(UseCaseError::InternalServerError)));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let get = usecase.get_payment(&auth, &id).await;
        assert_eq!(get, Err(UseCaseError::InternalServerError));
    }

    // -------------------------------------------------------------------------
    //  get_payment_by_group
    // -------------------------------------------------------------------------

    #[tokio::test]
    async fn get_payments_by_group_200() {
        let claims: Claims = Faker.fake();
        let payment: Payment = Faker.fake();
        let mut group: Group = Faker.fake();
        group.id = payment.group.clone();
        group.participants.push(UserID::new(&claims.sub));
        let id = group.id.clone();

        let mut mock = MockRepository::new();
        mock.expect_get_group()
            .returning(move |_| Ok(Some(group.clone())));
        mock.expect_get_payments_by_group()
            .returning(move |_| Ok(vec![payment.clone()]));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let get = usecase.get_payments_by_group(&auth, &id).await.unwrap();
        assert_eq!(get.len(), 1);
    }

    #[tokio::test]
    async fn get_payments_by_group_404() {
        let claims: Claims = Faker.fake();
        let id: GroupID = Faker.fake();

        let mut mock = MockRepository::new();
        mock.expect_get_group().returning(move |_| Ok(None));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let get = usecase.get_payments_by_group(&auth, &id).await;
        assert_eq!(get, Err(UseCaseError::NotFound));
    }

    #[tokio::test]
    async fn get_payments_by_group_500() {
        let claims: Claims = Faker.fake();
        let mut group: Group = Faker.fake();
        group.participants.push(UserID::new(&claims.sub));
        let id = group.id.clone();

        let mut mock = MockRepository::new();
        mock.expect_get_group()
            .returning(move |_| Ok(Some(group.clone())));
        mock.expect_get_payments_by_group()
            .returning(move |_| Err(Box::new(UseCaseError::InternalServerError)));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let get = usecase.get_payments_by_group(&auth, &id).await;
        assert_eq!(get, Err(UseCaseError::InternalServerError));
    }

    // -------------------------------------------------------------------------
    //  create_payment
    // -------------------------------------------------------------------------

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

    // -------------------------------------------------------------------------
    //  update_payment
    // -------------------------------------------------------------------------

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

    // -------------------------------------------------------------------------
    //  delete_payment
    // -------------------------------------------------------------------------

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
