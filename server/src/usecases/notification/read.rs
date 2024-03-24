use crate::{
    entities::{AuthState, GroupID, Notification, NotificationID},
    usecases::{UseCase, UseCaseError},
};

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
}
