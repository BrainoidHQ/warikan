use crate::{
    entities::{AuthState, NotificationID},
    usecases::{UseCase, UseCaseError},
};
use async_graphql::InputObject;

#[cfg(test)]
use fake::Dummy;

#[derive(InputObject)]
#[cfg_attr(test, derive(Dummy))]
pub struct DeleteNotificationInput {
    pub id: NotificationID,
}

impl UseCase {
    pub async fn delete_notification(
        &self,
        auth: &AuthState,
        input: DeleteNotificationInput,
    ) -> Result<NotificationID, UseCaseError> {
        if let Some(notification) = self.get_notification(auth, &input.id).await? {
            self.repository
                .delete_notification(&notification.id)
                .await
                .or(Err(UseCaseError::InternalServerError))?;
            Ok(notification.id)
        } else {
            Err(UseCaseError::NotFound)?
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        entities::{Claims, Group, Notification, UserID},
        repositories::MockRepository,
    };
    use fake::{Fake, Faker};
    use std::sync::Arc;

    #[tokio::test]
    async fn delete_notification_200() {
        let claims: Claims = Faker.fake();
        let notification: Notification = Faker.fake();
        let input = DeleteNotificationInput {
            id: notification.id.clone(),
        };
        let mut group: Group = Faker.fake();
        group.id = notification.group.clone();
        group.participants.push(UserID::new(&claims.sub));
        let id = notification.id.clone();

        let mut mock = MockRepository::new();
        mock.expect_get_group()
            .returning(move |_| Ok(Some(group.clone())));
        mock.expect_get_notification()
            .returning(move |_| Ok(Some(notification.clone())));
        mock.expect_delete_notification().returning(|_| Ok(()));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let delete = usecase.delete_notification(&auth, input).await.unwrap();
        assert_eq!(delete, id);
    }

    #[tokio::test]
    async fn delete_notification_404() {
        let claims: Claims = Faker.fake();
        let input: DeleteNotificationInput = Faker.fake();

        let mut mock = MockRepository::new();
        mock.expect_get_notification().returning(move |_| Ok(None));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let delete = usecase.delete_notification(&auth, input).await;
        assert_eq!(delete, Err(UseCaseError::NotFound));
    }

    #[tokio::test]
    async fn delete_notification_500() {
        let claims: Claims = Faker.fake();
        let notification: Notification = Faker.fake();
        let input = DeleteNotificationInput {
            id: notification.id.clone(),
        };
        let mut group: Group = Faker.fake();
        group.id = notification.group.clone();
        group.participants.push(UserID::new(&claims.sub));

        let mut mock = MockRepository::new();
        mock.expect_get_group()
            .returning(move |_| Ok(Some(group.clone())));
        mock.expect_get_notification()
            .returning(move |_| Ok(Some(notification.clone())));
        mock.expect_delete_notification()
            .returning(move |_| Err(Box::new(UseCaseError::InternalServerError)));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let delete = usecase.delete_notification(&auth, input).await;
        assert_eq!(delete, Err(UseCaseError::InternalServerError));
    }
}
