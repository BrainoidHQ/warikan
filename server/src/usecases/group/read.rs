use crate::{
    entities::{AuthState, Group, GroupID, UserID},
    usecases::{UseCase, UseCaseError},
};

impl UseCase {
    pub async fn get_group(
        &self,
        auth: &AuthState,
        id: &GroupID,
    ) -> Result<Option<Group>, UseCaseError> {
        if let AuthState::Authorized(claims) = auth {
            let user = UserID::new(&claims.sub);
            let group = self
                .repository
                .get_group(id)
                .await
                .or(Err(UseCaseError::InternalServerError))?
                .map(|group| {
                    group
                        .participants
                        .contains(&user)
                        .then_some(group)
                        .ok_or(UseCaseError::Forbidden)
                })
                .transpose()?;
            Ok(group)
        } else {
            Err(UseCaseError::Unauthorized)?
        }
    }

    pub async fn get_groups_by_user(
        &self,
        auth: &AuthState,
        id: &UserID,
    ) -> Result<Vec<Group>, UseCaseError> {
        if let AuthState::Authorized(claims) = auth {
            if id == &UserID::new(&claims.sub) {
                let groups = self
                    .repository
                    .get_groups_by_user(id)
                    .await
                    .or(Err(UseCaseError::InternalServerError))?;
                Ok(groups)
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
    async fn get_group_200() {
        let claims: Claims = Faker.fake();
        let mut group: Group = Faker.fake();
        group.participants.push(UserID::new(&claims.sub));
        let id = group.id.clone();

        let mut mock = MockRepository::new();
        mock.expect_get_group()
            .returning(move |_| Ok(Some(group.clone())));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let get = usecase.get_group(&auth, &id).await.unwrap();
        assert_eq!(get.map(|g| g.id), Some(id));
    }

    #[tokio::test]
    async fn get_group_401() {
        let group: GroupID = Faker.fake();

        let mock = MockRepository::new();

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Unauthorized;

        let get = usecase.get_group(&auth, &group).await;
        assert_eq!(get, Err(UseCaseError::Unauthorized));
    }

    #[tokio::test]
    async fn get_group_403() {
        let claims: Claims = Faker.fake();
        let mut group: Group = Faker.fake();
        group.participants.clear();
        let id = group.id.clone();

        let mut mock = MockRepository::new();
        mock.expect_get_group()
            .returning(move |_| Ok(Some(group.clone())));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let get = usecase.get_group(&auth, &id).await;
        assert_eq!(get, Err(UseCaseError::Forbidden));
    }

    #[tokio::test]
    async fn get_group_500() {
        let claims: Claims = Faker.fake();
        let group: GroupID = Faker.fake();

        let mut mock = MockRepository::new();
        mock.expect_get_group()
            .returning(move |_| Err(Box::new(UseCaseError::InternalServerError)));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let get = usecase.get_group(&auth, &group).await;
        assert_eq!(get, Err(UseCaseError::InternalServerError));
    }

    #[tokio::test]
    async fn get_groups_by_user_200() {
        let claims: Claims = Faker.fake();
        let user = UserID::new(&claims.sub);
        let groups: Vec<Group> = Faker.fake();
        let len = groups.len();

        let mut mock = MockRepository::new();
        mock.expect_get_groups_by_user()
            .returning(move |_| Ok(groups.clone()));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let get = usecase.get_groups_by_user(&auth, &user).await.unwrap();
        assert_eq!(get.len(), len);
    }

    #[tokio::test]
    async fn get_groups_by_user_401() {
        let user: UserID = Faker.fake();

        let mock = MockRepository::new();

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Unauthorized;

        let get = usecase.get_groups_by_user(&auth, &user).await;
        assert_eq!(get, Err(UseCaseError::Unauthorized));
    }

    #[tokio::test]
    async fn get_groups_by_user_403() {
        let claims: Claims = Faker.fake();
        let user: UserID = Faker.fake();

        let mock = MockRepository::new();

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let get = usecase.get_groups_by_user(&auth, &user).await;
        assert_eq!(get, Err(UseCaseError::Forbidden));
    }

    #[tokio::test]
    async fn get_groups_by_user_500() {
        let claims: Claims = Faker.fake();
        let user = UserID::new(&claims.sub);

        let mut mock = MockRepository::new();
        mock.expect_get_groups_by_user()
            .returning(move |_| Err(Box::new(UseCaseError::InternalServerError)));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let get = usecase.get_groups_by_user(&auth, &user).await;
        assert_eq!(get, Err(UseCaseError::InternalServerError));
    }
}
