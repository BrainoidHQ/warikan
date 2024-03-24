use crate::{
    entities::{AuthState, GroupID},
    usecases::{UseCase, UseCaseError},
};
use async_graphql::InputObject;
use futures::future::try_join_all;

#[cfg(test)]
use fake::Dummy;

#[derive(InputObject)]
#[cfg_attr(test, derive(Dummy))]
pub struct DeleteGroupInput {
    pub id: GroupID,
}

impl UseCase {
    pub async fn delete_group(
        &self,
        auth: &AuthState,
        input: DeleteGroupInput,
    ) -> Result<GroupID, UseCaseError> {
        if self.get_group(auth, &input.id).await?.is_some() {
            self.repository
                .delete_group(&input.id)
                .await
                .or(Err(UseCaseError::InternalServerError))?;
            try_join_all(
                self.repository
                    .get_payments_by_group(&input.id)
                    .await
                    .or(Err(UseCaseError::InternalServerError))?
                    .iter()
                    .map(|payment| async {
                        self.repository
                            .delete_payment(&payment.id)
                            .await
                            .or(Err(UseCaseError::InternalServerError))
                    }),
            )
            .await?;
            try_join_all(
                self.repository
                    .get_notifications_by_group(&input.id)
                    .await
                    .or(Err(UseCaseError::InternalServerError))?
                    .iter()
                    .map(|payment| async {
                        self.repository
                            .delete_notification(&payment.id)
                            .await
                            .or(Err(UseCaseError::InternalServerError))
                    }),
            )
            .await?;
            Ok(input.id)
        } else {
            Err(UseCaseError::NotFound)?
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        entities::{Claims, Group, Notification, Payment, UserID},
        repositories::MockRepository,
    };
    use fake::{Fake, Faker};
    use std::sync::Arc;

    #[tokio::test]
    async fn delete_group_200() {
        let claims: Claims = Faker.fake();
        let mut group: Group = Faker.fake();
        group.participants.push(UserID::new(&claims.sub));
        let input = DeleteGroupInput {
            id: group.id.clone(),
        };
        let id = group.id.clone();
        let payments: Vec<Payment> = Faker.fake();

        let notifications: Vec<Notification> = Faker.fake();

        let mut mock = MockRepository::new();
        mock.expect_get_group()
            .returning(move |_| Ok(Some(group.clone())));
        mock.expect_delete_group().returning(move |_| Ok(()));
        mock.expect_delete_payment()
            .times(payments.len())
            .returning(move |_| Ok(()));
        mock.expect_delete_notification()
            .times(notifications.len())
            .returning(move |_| Ok(()));
        mock.expect_get_payments_by_group()
            .returning(move |_| Ok(payments.clone()));
        mock.expect_get_notifications_by_group()
            .returning(move |_| Ok(notifications.clone()));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let delete = usecase.delete_group(&auth, input).await.unwrap();
        assert_eq!(delete, id);
    }

    #[tokio::test]
    async fn delete_group_404() {
        let claims: Claims = Faker.fake();
        let input: DeleteGroupInput = Faker.fake();

        let mut mock = MockRepository::new();
        mock.expect_get_group().returning(move |_| Ok(None));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let delete = usecase.delete_group(&auth, input).await;
        assert_eq!(delete, Err(UseCaseError::NotFound));
    }

    #[tokio::test]
    async fn delete_group_500() {
        let claims: Claims = Faker.fake();
        let input: DeleteGroupInput = Faker.fake();
        let mut group: Group = Faker.fake();
        group.participants.push(UserID::new(&claims.sub));

        let mut mock = MockRepository::new();
        mock.expect_get_group()
            .returning(move |_| Ok(Some(group.clone())));
        mock.expect_delete_group()
            .returning(move |_| Err(Box::new(UseCaseError::InternalServerError)));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let delete = usecase.delete_group(&auth, input).await;
        assert_eq!(delete, Err(UseCaseError::InternalServerError));
    }
}
