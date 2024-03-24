use crate::{
    entities::{AuthState, User, UserID},
    usecases::{UseCase, UseCaseError},
};
use async_graphql::InputObject;
use chrono::Utc;

#[cfg(test)]
use fake::Dummy;

#[derive(InputObject)]
#[cfg_attr(test, derive(Dummy))]
pub struct CreateUserInput {
    pub name: String,
}

impl UseCase {
    pub async fn create_user(
        &self,
        auth: &AuthState,
        input: CreateUserInput,
    ) -> Result<User, UseCaseError> {
        if let AuthState::Authorized(claims) = auth {
            let now = Utc::now();
            let user = User {
                id: UserID::new(&claims.sub),
                created_at: now,
                updated_at: now,
                name: input.name,
            };
            let user = self
                .repository
                .create_user(user)
                .await
                .or(Err(UseCaseError::InternalServerError))?;
            Ok(user)
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
    async fn create_user_200() {
        let claims: Claims = Faker.fake();
        let input: CreateUserInput = Faker.fake();
        let name = input.name.clone();

        let mut mock = MockRepository::new();
        mock.expect_create_user().returning(|user| Ok(user));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let create = usecase.create_user(&auth, input).await.unwrap();
        assert_eq!(create.name, name);
    }

    #[tokio::test]
    async fn create_user_401() {
        let input: CreateUserInput = Faker.fake();

        let mock = MockRepository::new();

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Unauthorized;

        let create = usecase.create_user(&auth, input).await;
        assert_eq!(create, Err(UseCaseError::Unauthorized));
    }

    #[tokio::test]
    async fn create_user_500() {
        let claims: Claims = Faker.fake();
        let input: CreateUserInput = Faker.fake();

        let mut mock = MockRepository::new();
        mock.expect_create_user()
            .returning(|_| Err(Box::new(UseCaseError::InternalServerError)));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let create = usecase.create_user(&auth, input).await;
        assert_eq!(create, Err(UseCaseError::InternalServerError));
    }
}
