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
pub struct UpdateUserInput {
    pub id: UserID,
    pub name: Option<String>,
}

impl UseCase {
    pub async fn update_user(
        &self,
        auth: &AuthState,
        input: UpdateUserInput,
    ) -> Result<User, UseCaseError> {
        if let AuthState::Authorized(claims) = auth {
            if input.id == UserID::new(&claims.sub) {
                if let Some(user) = self.get_user_opt(auth, &input.id).await? {
                    let user = User {
                        id: input.id,
                        created_at: user.created_at,
                        updated_at: Utc::now(),
                        name: input.name.unwrap_or(user.name),
                    };
                    let user = self
                        .repository
                        .update_user(user)
                        .await
                        .or(Err(UseCaseError::InternalServerError))?;
                    Ok(user)
                } else {
                    Err(UseCaseError::NotFound)?
                }
            } else {
                Err(UseCaseError::Forbidden)?
            }
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
    async fn update_user_200() {
        let claims: Claims = Faker.fake();
        let id = UserID::new(&claims.sub);
        let mut user: User = Faker.fake();
        user.id = id.clone();
        let mut input: UpdateUserInput = Faker.fake();
        input.id = id.clone();

        let mut mock = MockRepository::new();
        mock.expect_get_user()
            .returning(move |_| Ok(Some(user.clone())));
        mock.expect_update_user().returning(|user| Ok(user));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let update = usecase.update_user(&auth, input).await.unwrap();
        assert_eq!(update.id, id);
    }

    #[tokio::test]
    async fn update_user_401() {
        let input: UpdateUserInput = Faker.fake();

        let mock = MockRepository::new();

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Unauthorized;

        let update = usecase.update_user(&auth, input).await;
        assert_eq!(update, Err(UseCaseError::Unauthorized));
    }

    #[tokio::test]
    async fn update_user_403() {
        let claims: Claims = Faker.fake();
        let input: UpdateUserInput = Faker.fake();

        let mock = MockRepository::new();

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let update = usecase.update_user(&auth, input).await;
        assert_eq!(update, Err(UseCaseError::Forbidden));
    }

    #[tokio::test]
    async fn update_user_404() {
        let claims: Claims = Faker.fake();
        let id = UserID::new(&claims.sub);
        let mut input: UpdateUserInput = Faker.fake();
        input.id = id.clone();

        let mut mock = MockRepository::new();
        mock.expect_get_user().returning(move |_| Ok(None));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let update = usecase.update_user(&auth, input).await;
        assert_eq!(update, Err(UseCaseError::NotFound));
    }

    #[tokio::test]
    async fn update_user_500() {
        let claims: Claims = Faker.fake();
        let id = UserID::new(&claims.sub);
        let mut user: User = Faker.fake();
        user.id = id.clone();
        let mut input: UpdateUserInput = Faker.fake();
        input.id = id.clone();

        let mut mock = MockRepository::new();
        mock.expect_get_user()
            .returning(move |_| Ok(Some(user.clone())));
        mock.expect_update_user()
            .returning(|_| Err(Box::new(UseCaseError::InternalServerError)));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let update = usecase.update_user(&auth, input).await;
        assert_eq!(update, Err(UseCaseError::InternalServerError));
    }
}
