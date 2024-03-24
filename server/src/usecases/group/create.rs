use crate::{
    entities::{AuthState, Group, GroupID, UserID},
    usecases::{UseCase, UseCaseError},
};
use async_graphql::InputObject;
use chrono::Utc;
use nanoid::nanoid;

#[cfg(test)]
use fake::Dummy;

#[derive(InputObject)]
#[cfg_attr(test, derive(Dummy))]
pub struct CreateGroupInput {
    pub title: String,
}

impl UseCase {
    pub async fn create_group(
        &self,
        auth: &AuthState,
        input: CreateGroupInput,
    ) -> Result<Group, UseCaseError> {
        if let AuthState::Authorized(claims) = auth {
            let now = Utc::now();
            let group = Group {
                id: GroupID::new(nanoid!()),
                created_at: now,
                updated_at: now,
                title: input.title,
                participants: vec![UserID::new(&claims.sub)],
            };
            let group = self
                .repository
                .create_group(group)
                .await
                .or(Err(UseCaseError::InternalServerError))?;
            Ok(group)
        } else {
            Err(UseCaseError::Unauthorized)?
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{entities::Claims, repositories::MockRepository};
    use fake::{Fake, Faker};
    use std::sync::Arc;

    #[tokio::test]
    async fn create_group_200() {
        let claims: Claims = Faker.fake();
        let input: CreateGroupInput = Faker.fake();
        let group: Group = Faker.fake();
        let id = group.id.clone();

        let mut mock = MockRepository::new();
        mock.expect_create_group()
            .returning(move |_| Ok(group.clone()));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let create = usecase.create_group(&auth, input).await.unwrap();
        assert_eq!(create.id, id);
    }

    #[tokio::test]
    async fn create_group_401() {
        let input: CreateGroupInput = Faker.fake();

        let mock = MockRepository::new();

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Unauthorized;

        let create = usecase.create_group(&auth, input).await;
        assert_eq!(create, Err(UseCaseError::Unauthorized));
    }

    #[tokio::test]
    async fn create_group_500() {
        let claims: Claims = Faker.fake();
        let input: CreateGroupInput = Faker.fake();

        let mut mock = MockRepository::new();
        mock.expect_create_group()
            .returning(move |_| Err(Box::new(UseCaseError::InternalServerError)));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let create = usecase.create_group(&auth, input).await;
        assert_eq!(create, Err(UseCaseError::InternalServerError));
    }
}
