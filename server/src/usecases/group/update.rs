use crate::{
    entities::{AuthState, Group, GroupID, UserID},
    usecases::{UseCase, UseCaseError},
};
use async_graphql::InputObject;
use chrono::Utc;

#[cfg(test)]
use fake::Dummy;

#[derive(InputObject)]
#[cfg_attr(test, derive(Dummy))]
pub struct UpdateGroupInput {
    pub id: GroupID,

    pub title: Option<String>,
    pub participants: Option<Vec<UserID>>,
}

impl UseCase {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{entities::Claims, repositories::MockRepository};
    use fake::{Fake, Faker};
    use std::sync::Arc;

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
}
