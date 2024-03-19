use crate::{
    entities::{AuthState, Group, GroupID, UserID},
    usecases::{UseCase, UseCaseError},
};
use async_graphql::InputObject;
use chrono::Utc;
use futures::future::try_join_all;
use nanoid::nanoid;

#[cfg(test)]
use fake::Dummy;

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

#[derive(InputObject)]
#[cfg_attr(test, derive(Dummy))]
pub struct CreateGroupInput {
    pub title: String,
}

#[derive(InputObject)]
#[cfg_attr(test, derive(Dummy))]
pub struct UpdateGroupInput {
    pub id: GroupID,

    pub title: Option<String>,
    pub participants: Option<Vec<UserID>>,
}

#[derive(InputObject)]
#[cfg_attr(test, derive(Dummy))]
pub struct DeleteGroupInput {
    pub id: GroupID,
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

    pub async fn update_group(
        &self,
        auth: &AuthState,
        input: UpdateGroupInput,
    ) -> Result<Group, UseCaseError> {
        if let Some(group) = self.get_group(auth, &input.id).await? {
            let group = Group {
                id: input.id,
                created_at: group.created_at,
                updated_at: Utc::now(),
                title: input.title.unwrap_or(group.title),
                participants: input.participants.unwrap_or(group.participants),
            };
            let group = self
                .repository
                .update_group(group)
                .await
                .or(Err(UseCaseError::InternalServerError))?;
            Ok(group)
        } else {
            Err(UseCaseError::NotFound)?
        }
    }

    pub async fn delete_group(
        &self,
        auth: &AuthState,
        input: DeleteGroupInput,
    ) -> Result<GroupID, UseCaseError> {
        if self.get_group(auth, &input.id).await?.is_some() {
            self.repository
                .delete_group(&input.id)
                .await
                .or(Err(UseCaseError::InternalServerError))?;
            try_join_all(
                self.repository
                    .get_payments_by_group(&input.id)
                    .await
                    .or(Err(UseCaseError::InternalServerError))?
                    .iter()
                    .map(|payment| async {
                        self.repository
                            .delete_payment(&payment.id)
                            .await
                            .or(Err(UseCaseError::InternalServerError))
                    }),
            )
            .await?;
            try_join_all(
                self.repository
                    .get_notifications_by_group(&input.id)
                    .await
                    .or(Err(UseCaseError::InternalServerError))?
                    .iter()
                    .map(|payment| async {
                        self.repository
                            .delete_notification(&payment.id)
                            .await
                            .or(Err(UseCaseError::InternalServerError))
                    }),
            )
            .await?;
            Ok(input.id)
        } else {
            Err(UseCaseError::NotFound)?
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        entities::{Claims, Notification, Payment},
        repositories::MockRepository,
    };
    use fake::{Fake, Faker};
    use std::sync::Arc;

    // -------------------------------------------------------------------------
    //  get_group
    // -------------------------------------------------------------------------

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

    // -------------------------------------------------------------------------
    //  get_groups_by_user
    // -------------------------------------------------------------------------

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

    // -------------------------------------------------------------------------
    //  create_group
    // -------------------------------------------------------------------------

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

    // -------------------------------------------------------------------------
    //  update_group
    // -------------------------------------------------------------------------

    #[tokio::test]
    async fn update_group_200() {
        let claims: Claims = Faker.fake();
        let input: UpdateGroupInput = Faker.fake();
        let mut group1: Group = Faker.fake();
        group1.participants.push(UserID::new(&claims.sub));
        let group2 = group1.clone();
        let id = group1.id.clone();

        let mut mock = MockRepository::new();
        mock.expect_get_group()
            .returning(move |_| Ok(Some(group1.clone())));
        mock.expect_update_group()
            .returning(move |_| Ok(group2.clone()));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let update = usecase.update_group(&auth, input).await.unwrap();
        assert_eq!(update.id, id);
    }

    #[tokio::test]
    async fn update_group_404() {
        let claims: Claims = Faker.fake();
        let input: UpdateGroupInput = Faker.fake();

        let mut mock = MockRepository::new();
        mock.expect_get_group().returning(move |_| Ok(None));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let update = usecase.update_group(&auth, input).await;
        assert_eq!(update, Err(UseCaseError::NotFound));
    }

    #[tokio::test]
    async fn update_group_500() {
        let claims: Claims = Faker.fake();
        let input: UpdateGroupInput = Faker.fake();
        let mut group: Group = Faker.fake();
        group.participants.push(UserID::new(&claims.sub));

        let mut mock = MockRepository::new();
        mock.expect_get_group()
            .returning(move |_| Ok(Some(group.clone())));
        mock.expect_update_group()
            .returning(move |_| Err(Box::new(UseCaseError::InternalServerError)));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let update = usecase.update_group(&auth, input).await;
        assert_eq!(update, Err(UseCaseError::InternalServerError));
    }

    // -------------------------------------------------------------------------
    //  update_group
    // -------------------------------------------------------------------------

    #[tokio::test]
    async fn delete_group_200() {
        let claims: Claims = Faker.fake();
        let mut group: Group = Faker.fake();
        group.participants.push(UserID::new(&claims.sub));
        let input = DeleteGroupInput {
            id: group.id.clone(),
        };
        let id = group.id.clone();
        let payments: Vec<Payment> = Faker.fake();

        let notifications: Vec<Notification> = Faker.fake();

        let mut mock = MockRepository::new();
        mock.expect_get_group()
            .returning(move |_| Ok(Some(group.clone())));
        mock.expect_delete_group().returning(move |_| Ok(()));
        mock.expect_delete_payment()
            .times(payments.len())
            .returning(move |_| Ok(()));
        mock.expect_delete_notification()
            .times(notifications.len())
            .returning(move |_| Ok(()));
        mock.expect_get_payments_by_group()
            .returning(move |_| Ok(payments.clone()));
        mock.expect_get_notifications_by_group()
            .returning(move |_| Ok(notifications.clone()));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let delete = usecase.delete_group(&auth, input).await.unwrap();
        assert_eq!(delete, id);
    }

    #[tokio::test]
    async fn delete_group_404() {
        let claims: Claims = Faker.fake();
        let input: DeleteGroupInput = Faker.fake();

        let mut mock = MockRepository::new();
        mock.expect_get_group().returning(move |_| Ok(None));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let delete = usecase.delete_group(&auth, input).await;
        assert_eq!(delete, Err(UseCaseError::NotFound));
    }

    #[tokio::test]
    async fn delete_group_500() {
        let claims: Claims = Faker.fake();
        let input: DeleteGroupInput = Faker.fake();
        let mut group: Group = Faker.fake();
        group.participants.push(UserID::new(&claims.sub));

        let mut mock = MockRepository::new();
        mock.expect_get_group()
            .returning(move |_| Ok(Some(group.clone())));
        mock.expect_delete_group()
            .returning(move |_| Err(Box::new(UseCaseError::InternalServerError)));

        let usecase = UseCase::new(Arc::new(mock));
        let auth = AuthState::Authorized(claims);

        let delete = usecase.delete_group(&auth, input).await;
        assert_eq!(delete, Err(UseCaseError::InternalServerError));
    }
}
