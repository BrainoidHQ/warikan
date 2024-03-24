use crate::{
    entities::{AuthState, UserID},
    usecases::{UseCase, UseCaseError},
};
use async_graphql::InputObject;

#[cfg(test)]
use fake::Dummy;

#[derive(InputObject)]
#[cfg_attr(test, derive(Dummy))]
pub struct DeleteUserInput {
    pub id: UserID,
}

impl UseCase {
    pub async fn delete_user(
        &self,
        auth: &AuthState,
        input: DeleteUserInput,
    ) -> Result<UserID, UseCaseError> {
        if let AuthState::Authorized(claims) = auth {
            if input.id == UserID::new(&claims.sub) {
                if self.get_user_opt(auth, &input.id).await?.is_some() {
                    self.repository
                        .delete_user(&input.id)
                        .await
                        .or(Err(UseCaseError::InternalServerError))?;
                    Ok(input.id.clone())
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
    use crate::{
        entities::{Claims, User},
        repositories::MockRepository,
    };
    use fake::{Fake, Faker};
    use std::sync::Arc;

    #[tokio::test]
    async fn delete_user_200() {
        let claims: Claims = Faker.fake();
        let id = UserID::new(&claims.sub);
        let mut user: User = Faker.fake();
        user.id = id.clone();
        let input: DeleteUserInput = DeleteUserInput { id: id.clone() };

        let mut mock = MockRepository::new();
        mock.expect_get_user()
            .returning(move |_| Ok(Some(user.clone())));
        mock.expect_delete_user().returning(|_| Ok(()));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let delete = usecase.delete_user(&auth, input).await.unwrap();
        assert_eq!(delete, id);
    }

    #[tokio::test]
    async fn delete_user_401() {
        let input: DeleteUserInput = Faker.fake();

        let mock = MockRepository::new();

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Unauthorized;

        let delete = usecase.delete_user(&auth, input).await;
        assert_eq!(delete, Err(UseCaseError::Unauthorized));
    }

    #[tokio::test]
    async fn delete_user_403() {
        let claims: Claims = Faker.fake();
        let input: DeleteUserInput = Faker.fake();

        let mock = MockRepository::new();

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let delete = usecase.delete_user(&auth, input).await;
        assert_eq!(delete, Err(UseCaseError::Forbidden));
    }

    #[tokio::test]
    async fn delete_user_404() {
        let claims: Claims = Faker.fake();
        let id = UserID::new(&claims.sub);
        let input: DeleteUserInput = DeleteUserInput { id: id.clone() };

        let mut mock = MockRepository::new();
        mock.expect_get_user().returning(move |_| Ok(None));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let delete = usecase.delete_user(&auth, input).await;
        assert_eq!(delete, Err(UseCaseError::NotFound));
    }

    #[tokio::test]
    async fn delete_user_500() {
        let claims: Claims = Faker.fake();
        let id = UserID::new(&claims.sub);
        let mut user: User = Faker.fake();
        user.id = id.clone();
        let input: DeleteUserInput = DeleteUserInput { id: id.clone() };

        let mut mock = MockRepository::new();
        mock.expect_get_user()
            .returning(move |_| Ok(Some(user.clone())));
        mock.expect_delete_user()
            .returning(|_| Err(Box::new(UseCaseError::InternalServerError)));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let delete = usecase.delete_user(&auth, input).await;
        assert_eq!(delete, Err(UseCaseError::InternalServerError));
    }
}
