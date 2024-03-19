use crate::{
    entities::{AuthState, User, UserID},
    usecases::{UseCase, UseCaseError},
};
use async_graphql::InputObject;
use chrono::Utc;
use futures::future::try_join_all;

#[cfg(test)]
use fake::Dummy;

impl UseCase {
    pub async fn get_user_opt(
        &self,
        auth: &AuthState,
        id: &UserID,
    ) -> Result<Option<User>, UseCaseError> {
        if let AuthState::Authorized(_) = auth {
            let user = self
                .repository
                .get_user(id)
                .await
                .or(Err(UseCaseError::InternalServerError))?;
            Ok(user)
        } else {
            Err(UseCaseError::Unauthorized)?
        }
    }

    pub async fn get_user(&self, auth: &AuthState, id: &UserID) -> Result<User, UseCaseError> {
        self.get_user_opt(auth, id)
            .await?
            .ok_or(UseCaseError::NotFound)
    }

    pub async fn get_user_vec(
        &self,
        auth: &AuthState,
        ids: &[UserID],
    ) -> Result<Vec<User>, UseCaseError> {
        if let AuthState::Authorized(_) = auth {
            let users = try_join_all(ids.iter().map(|id| self.repository.get_user(id)))
                .await
                .or(Err(UseCaseError::InternalServerError))?
                .into_iter()
                .flatten()
                .collect();
            Ok(users)
        } else {
            Err(UseCaseError::Unauthorized)?
        }
    }
}

#[derive(InputObject)]
#[cfg_attr(test, derive(Dummy))]
pub struct CreateUserInput {
    pub name: String,
}

#[derive(InputObject)]
#[cfg_attr(test, derive(Dummy))]
pub struct UpdateUserInput {
    pub id: UserID,
    pub name: Option<String>,
}

#[derive(InputObject)]
#[cfg_attr(test, derive(Dummy))]
pub struct DeleteUserInput {
    pub id: UserID,
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
    use crate::{entities::Claims, repositories::MockRepository};
    use fake::{Fake, Faker};
    use itertools::Itertools;
    use std::sync::Arc;

    // -------------------------------------------------------------------------
    //  get_user_opt
    // -------------------------------------------------------------------------

    #[tokio::test]
    async fn get_user_opt_200() {
        let claims: Claims = Faker.fake();
        let id = UserID::new(&claims.sub);
        let mut user: User = Faker.fake();
        user.id = id.clone();

        let mut mock = MockRepository::new();
        mock.expect_get_user()
            .returning(move |_| Ok(Some(user.clone())));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let get = usecase.get_user_opt(&auth, &id).await.unwrap();
        assert_eq!(get.map(|u| u.id), Some(id));
    }

    #[tokio::test]
    async fn get_user_opt_401() {
        let id: UserID = Faker.fake();

        let mock = MockRepository::new();

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Unauthorized;

        let get = usecase.get_user_opt(&auth, &id).await;
        assert_eq!(get, Err(UseCaseError::Unauthorized));
    }

    #[tokio::test]
    async fn get_user_opt_500() {
        let claims: Claims = Faker.fake();
        let id = UserID::new(&claims.sub);

        let mut mock = MockRepository::new();
        mock.expect_get_user()
            .returning(|_| Err(Box::new(UseCaseError::InternalServerError)));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let get = usecase.get_user_opt(&auth, &id).await;
        assert_eq!(get, Err(UseCaseError::InternalServerError));
    }

    // -------------------------------------------------------------------------
    //  get_group_vec
    // -------------------------------------------------------------------------

    #[tokio::test]
    async fn get_user_vec_200() {
        let claims: Claims = Faker.fake();
        let ids: Vec<UserID> = Faker.fake();
        let user: User = Faker.fake();

        let mut mock = MockRepository::new();
        mock.expect_get_user()
            .times(ids.len())
            .returning(move |id| {
                let mut user = user.clone();
                user.id = id.clone();
                Ok(Some(user))
            });

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let get = usecase.get_user_vec(&auth, &ids).await.unwrap();

        let actual: Vec<&UserID> = get.iter().map(|u| &u.id).sorted().collect();
        let expected: Vec<&UserID> = ids.iter().sorted().collect();
        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn get_user_vec_401() {
        let ids: Vec<UserID> = Faker.fake();

        let mock = MockRepository::new();

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Unauthorized;

        let get = usecase.get_user_vec(&auth, &ids).await;
        assert_eq!(get, Err(UseCaseError::Unauthorized));
    }

    #[tokio::test]
    async fn get_user_vec_500() {
        let claims: Claims = Faker.fake();
        let ids: Vec<UserID> = Faker.fake();

        let mut mock = MockRepository::new();
        mock.expect_get_user()
            .returning(|_| Err(Box::new(UseCaseError::InternalServerError)));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let get = usecase.get_user_vec(&auth, &ids).await;

        if ids.len() == 0 {
            assert_eq!(get, Ok(vec![]));
        } else {
            assert_eq!(get, Err(UseCaseError::InternalServerError));
        }
    }

    // -------------------------------------------------------------------------
    //  create_user
    // -------------------------------------------------------------------------

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

    // -------------------------------------------------------------------------
    //  update_user
    // -------------------------------------------------------------------------

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

    // -------------------------------------------------------------------------
    //  delete_user
    // -------------------------------------------------------------------------

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
