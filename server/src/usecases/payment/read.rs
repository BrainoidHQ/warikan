use crate::{
    entities::{warikan, AuthState, GroupID, Payment, PaymentID, Warikan},
    usecases::{UseCase, UseCaseError},
};

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
}
