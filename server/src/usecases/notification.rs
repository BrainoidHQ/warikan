use crate::{
    entities::{AuthState, GroupID, Notification, NotificationID},
    usecases::{UseCase, UseCaseError},
};
use async_graphql::InputObject;
use chrono::Utc;
use nanoid::nanoid;

#[cfg(test)]
use fake::Dummy;

impl UseCase {
    pub async fn get_notification(
        &self,
        auth: &AuthState,
        id: &NotificationID,
    ) -> Result<Option<Notification>, UseCaseError> {
        if let Some(notification) = self
            .repository
            .get_notification(id)
            .await
            .or(Err(UseCaseError::InternalServerError))?
        {
            let notification = self
                .get_group(auth, &notification.group)
                .await?
                .and(Some(notification));
            Ok(notification)
        } else {
            Ok(None)
        }
    }

    pub async fn get_notifications_by_group(
        &self,
        auth: &AuthState,
        id: &GroupID,
    ) -> Result<Vec<Notification>, UseCaseError> {
        if self.get_group(auth, id).await?.is_some() {
            let notifications = self
                .repository
                .get_notifications_by_group(id)
                .await
                .or(Err(UseCaseError::InternalServerError))?;
            Ok(notifications)
        } else {
            Err(UseCaseError::NotFound)?
        }
    }
}

#[derive(InputObject)]
#[cfg_attr(test, derive(Dummy))]
pub struct CreateNotificationInput {
    pub message: String,
    pub group: GroupID,
}

#[derive(InputObject)]
#[cfg_attr(test, derive(Dummy))]
pub struct DeleteNotificationInput {
    pub id: NotificationID,
}

impl UseCase {
    pub async fn create_notification(
        &self,
        auth: &AuthState,
        input: CreateNotificationInput,
    ) -> Result<Notification, UseCaseError> {
        if self.get_group(auth, &input.group).await?.is_some() {
            let now = Utc::now();
            let notification = Notification {
                id: NotificationID::new(nanoid!()),
                created_at: now,
                updated_at: now,
                message: input.message,
                group: input.group,
            };
            let notification = self
                .repository
                .create_notification(notification)
                .await
                .or(Err(UseCaseError::InternalServerError))?;
            Ok(notification)
        } else {
            Err(UseCaseError::NotFound)?
        }
    }

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
        entities::{Claims, Group, UserID},
        repositories::MockRepository,
    };
    use fake::{Fake, Faker};
    use std::sync::Arc;

    // -------------------------------------------------------------------------
    //  get_notification
    // -------------------------------------------------------------------------

    #[tokio::test]
    async fn get_notification_200() {
        let claims: Claims = Faker.fake();
        let notification: Notification = Faker.fake();
        let mut group: Group = Faker.fake();
        group.id = notification.group.clone();
        group.participants.push(UserID::new(&claims.sub));
        let id = notification.id.clone();

        let mut mock = MockRepository::new();
        mock.expect_get_notification()
            .returning(move |_| Ok(Some(notification.clone())));
        mock.expect_get_group()
            .returning(move |_| Ok(Some(group.clone())));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let get = usecase.get_notification(&auth, &id).await.unwrap();
        assert_eq!(get.map(|g| g.id), Some(id));
    }

    #[tokio::test]
    async fn get_notification_500() {
        let claims: Claims = Faker.fake();
        let id: NotificationID = Faker.fake();

        let mut mock = MockRepository::new();
        mock.expect_get_notification()
            .returning(move |_| Err(Box::new(UseCaseError::InternalServerError)));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let get = usecase.get_notification(&auth, &id).await;
        assert_eq!(get, Err(UseCaseError::InternalServerError));
    }

    // -------------------------------------------------------------------------
    //  get_notifications_by_group
    // -------------------------------------------------------------------------

    #[tokio::test]
    async fn get_notifications_by_group_200() {
        let claims: Claims = Faker.fake();
        let mut group: Group = Faker.fake();
        group.participants.push(UserID::new(&claims.sub));
        let id = group.id.clone();

        let mut mock = MockRepository::new();
        mock.expect_get_group()
            .returning(move |_| Ok(Some(group.clone())));
        mock.expect_get_notifications_by_group()
            .returning(move |_| Ok(vec![]));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let get = usecase
            .get_notifications_by_group(&auth, &id)
            .await
            .unwrap();
        assert_eq!(get.len(), 0);
    }

    #[tokio::test]
    async fn get_notifications_by_group_404() {
        let claims: Claims = Faker.fake();
        let id: GroupID = Faker.fake();

        let mut mock = MockRepository::new();
        mock.expect_get_group().returning(move |_| Ok(None));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let get = usecase.get_notifications_by_group(&auth, &id).await;
        assert_eq!(get, Err(UseCaseError::NotFound));
    }

    #[tokio::test]
    async fn get_notifications_by_group_500() {
        let claims: Claims = Faker.fake();
        let mut group: Group = Faker.fake();
        group.participants.push(UserID::new(&claims.sub));
        let id = group.id.clone();

        let mut mock = MockRepository::new();
        mock.expect_get_group()
            .returning(move |_| Ok(Some(group.clone())));
        mock.expect_get_notifications_by_group()
            .returning(move |_| Err(Box::new(UseCaseError::InternalServerError)));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let get = usecase.get_notifications_by_group(&auth, &id).await;
        assert_eq!(get, Err(UseCaseError::InternalServerError));
    }

    // -------------------------------------------------------------------------
    //  create_notification
    // -------------------------------------------------------------------------

    #[tokio::test]
    async fn create_notification_200() {
        let claims: Claims = Faker.fake();
        let input: CreateNotificationInput = Faker.fake();
        let mut group: Group = Faker.fake();
        group.id = input.group.clone();
        group.participants.push(UserID::new(&claims.sub));
        let id = input.group.clone();

        let mut mock = MockRepository::new();
        mock.expect_get_group()
            .returning(move |_| Ok(Some(group.clone())));
        mock.expect_create_notification().returning(|n| Ok(n));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let create = usecase.create_notification(&auth, input).await.unwrap();
        assert_eq!(create.group, id);
    }

    #[tokio::test]
    async fn create_notification_500() {
        let claims: Claims = Faker.fake();
        let input: CreateNotificationInput = Faker.fake();
        let mut group: Group = Faker.fake();
        group.participants.push(UserID::new(&claims.sub));

        let mut mock = MockRepository::new();
        mock.expect_get_group()
            .returning(move |_| Ok(Some(group.clone())));
        mock.expect_create_notification()
            .returning(move |_| Err(Box::new(UseCaseError::InternalServerError)));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let create = usecase.create_notification(&auth, input).await;
        assert_eq!(create, Err(UseCaseError::InternalServerError));
    }

    // -------------------------------------------------------------------------
    //  delete_notification
    // -------------------------------------------------------------------------

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
