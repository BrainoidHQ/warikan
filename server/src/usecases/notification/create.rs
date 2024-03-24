use crate::{
    entities::{AuthState, GroupID, Notification, NotificationID},
    usecases::{UseCase, UseCaseError},
};
use async_graphql::InputObject;
use chrono::Utc;
use nanoid::nanoid;

#[cfg(test)]
use fake::Dummy;

#[derive(InputObject)]
#[cfg_attr(test, derive(Dummy))]
pub struct CreateNotificationInput {
    pub message: String,
    pub group: GroupID,
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
}
