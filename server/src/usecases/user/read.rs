use crate::{
    entities::{AuthState, User, UserID},
    usecases::{UseCase, UseCaseError},
};
use futures::future::try_join_all;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{entities::Claims, repositories::MockRepository};
    use fake::{Fake, Faker};
    use itertools::Itertools;
    use std::sync::Arc;

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
}
