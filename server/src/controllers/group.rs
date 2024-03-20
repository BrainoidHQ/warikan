use crate::{
    entities::{AuthState, Group, GroupID, Notification, Payment, User, UserID, Warikan},
    usecases::{CreateGroupInput, DeleteGroupInput, UpdateGroupInput, UseCase},
};
use async_graphql::{Context, Object};
use chrono::{DateTime, Utc};

#[Object]
impl Group {
    async fn id(&self) -> GroupID {
        self.id.clone()
    }

    async fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    async fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    async fn title(&self) -> String {
        self.title.clone()
    }

    async fn participants(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<User>> {
        let usecase = ctx.data::<UseCase>()?;
        let auth = ctx.data::<AuthState>()?;
        Ok(usecase.get_user_vec(auth, &self.participants).await?)
    }

    async fn payments(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Payment>> {
        let usecase = ctx.data::<UseCase>()?;
        let auth = ctx.data::<AuthState>()?;
        Ok(usecase.get_payments_by_group(auth, &self.id).await?)
    }

    async fn notifications(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Notification>> {
        let usecase = ctx.data::<UseCase>()?;
        let auth = ctx.data::<AuthState>()?;
        Ok(usecase.get_notifications_by_group(auth, &self.id).await?)
    }

    async fn warikan(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Warikan>> {
        let usecase = ctx.data::<UseCase>()?;
        let auth = ctx.data::<AuthState>()?;
        Ok(usecase.warikan_by_group(auth, &self.id).await?)
    }
}

#[Object]
impl Warikan {
    async fn from(&self, ctx: &Context<'_>) -> async_graphql::Result<User> {
        let usecase = ctx.data::<UseCase>()?;
        let auth = ctx.data::<AuthState>()?;
        let user = usecase.get_user(auth, &self.from).await?;
        Ok(user)
    }

    async fn to(&self, ctx: &Context<'_>) -> async_graphql::Result<User> {
        let usecase = ctx.data::<UseCase>()?;
        let auth = ctx.data::<AuthState>()?;
        let user = usecase.get_user(auth, &self.to).await?;
        Ok(user)
    }

    async fn amount(&self) -> i32 {
        self.amount
    }
}

#[derive(Default)]
pub struct GroupQuery;

#[Object]
impl GroupQuery {
    async fn group(&self, ctx: &Context<'_>, id: GroupID) -> async_graphql::Result<Option<Group>> {
        let usecase = ctx.data::<UseCase>()?;
        let auth = ctx.data::<AuthState>()?;
        Ok(usecase.get_group(auth, &id).await?)
    }

    async fn groups(&self, ctx: &Context<'_>, id: UserID) -> async_graphql::Result<Vec<Group>> {
        let usecase = ctx.data::<UseCase>()?;
        let auth = ctx.data::<AuthState>()?;
        Ok(usecase.get_groups_by_user(auth, &id).await?)
    }
}

#[derive(Default)]
pub struct GroupMutation;

#[Object]
impl GroupMutation {
    async fn create_group(
        &self,
        ctx: &Context<'_>,
        input: CreateGroupInput,
    ) -> async_graphql::Result<Group> {
        let usecase = ctx.data::<UseCase>()?;
        let auth = ctx.data::<AuthState>()?;
        Ok(usecase.create_group(auth, input).await?)
    }

    async fn update_group(
        &self,
        ctx: &Context<'_>,
        input: UpdateGroupInput,
    ) -> async_graphql::Result<Group> {
        let usecase = ctx.data::<UseCase>()?;
        let auth = ctx.data::<AuthState>()?;
        Ok(usecase.update_group(auth, input).await?)
    }

    async fn delete_group(
        &self,
        ctx: &Context<'_>,
        input: DeleteGroupInput,
    ) -> async_graphql::Result<GroupID> {
        let usecase = ctx.data::<UseCase>()?;
        let auth = ctx.data::<AuthState>()?;
        Ok(usecase.delete_group(auth, input).await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{entities::Claims, repositories::MockRepository};
    use async_graphql::{EmptySubscription, Request, Schema, Value, Variables};
    use chrono::SecondsFormat;
    use fake::{Fake, Faker};
    use indoc::indoc;
    use serde_json::json;
    use std::{collections::HashMap, sync::Arc, vec};

    impl TryFrom<Group> for Value {
        type Error = serde_json::Error;

        fn try_from(group: Group) -> Result<Self, Self::Error> {
            let participants = group
                .participants
                .iter()
                .map(|id| json!({ "id": id }))
                .collect::<Vec<_>>();
            Value::from_json(json!({
                "id": group.id,
                "createdAt": group.created_at.to_rfc3339_opts(SecondsFormat::Nanos, false),
                "updatedAt": group.updated_at.to_rfc3339_opts(SecondsFormat::Nanos, false),
                "title": group.title,
                "participants": participants,
            }))
        }
    }

    #[tokio::test]
    async fn query_group() {
        let users = fake::vec![User; 1..10];
        let mut group: Group = Faker.fake();
        let mut claims: Claims = Faker.fake();

        group.participants = users.iter().map(|u| u.id.clone()).collect();
        claims.sub = users[0].id.to_string();

        let expected = Value::try_from(group.clone()).unwrap();
        let users: HashMap<_, _> = users.into_iter().map(|u| (u.id.clone(), u)).collect();
        let id = group.id.clone();

        let mut mock = MockRepository::new();
        mock.expect_get_group()
            .returning(move |_| Ok(Some(group.clone())));
        mock.expect_get_user()
            .returning(move |id| Ok(Some(users[id].clone())));

        let auth = AuthState::Authorized(claims);
        let usecase = UseCase::new(Arc::new(mock));
        let schema = Schema::build(
            GroupQuery::default(),
            GroupMutation::default(),
            EmptySubscription,
        )
        .data(auth)
        .data(usecase)
        .finish();

        let query = indoc! {r#"
            query GetGroup($id: ID!) {
                group(id: $id) {
                    id
                    createdAt
                    updatedAt
                    title
                    participants {
                        id
                    }
                }
            }
        "#};
        let vars = Variables::from_json(json!({
            "id": id
        }));

        let req = Request::new(query).variables(vars);
        let res = schema.execute(req).await;

        if let Value::Object(data) = res.data {
            assert_eq!(data["group"], expected);
        } else {
            unreachable!();
        }
    }

    #[tokio::test]
    async fn query_groups() {
        let user: User = Faker.fake();
        let mut groups = fake::vec![Group; 1..10];
        let mut claims: Claims = Faker.fake();

        groups[0].participants = vec![user.id.clone()];
        claims.sub = user.id.to_string();

        let group = groups[0].clone();
        let expected = Value::List(vec![Value::try_from(group.clone()).unwrap()]);
        let id = user.id.clone();

        let mut mock = MockRepository::new();
        mock.expect_get_groups_by_user()
            .returning(move |_| Ok(vec![group.clone()]));
        mock.expect_get_user()
            .returning(move |_| Ok(Some(user.clone())));

        let auth = AuthState::Authorized(claims);
        let usecase = UseCase::new(Arc::new(mock));
        let schema = Schema::build(
            GroupQuery::default(),
            GroupMutation::default(),
            EmptySubscription,
        )
        .data(auth)
        .data(usecase)
        .finish();

        let query = indoc! {r#"
            query GetGroups($id: ID!) {
                groups(id: $id) {
                    id
                    createdAt
                    updatedAt
                    title
                    participants {
                        id
                    }
                }
            }
        "#};
        let vars = Variables::from_json(json!({
            "id": id
        }));

        let req = Request::new(query).variables(vars);
        let res = schema.execute(req).await;

        if let Value::Object(data) = res.data {
            assert_eq!(data["groups"], expected);
        } else {
            unreachable!();
        }
    }

    #[tokio::test]
    async fn mutation_create_group() {
        let user: User = Faker.fake();
        let title: String = Faker.fake();
        let mut claims: Claims = Faker.fake();

        claims.sub = user.id.to_string();

        let expected_title = Value::String(title.clone());
        let expected_participant = Value::from_json(json!([{"id": user.id}])).unwrap();

        let mut mock = MockRepository::new();
        mock.expect_create_group().returning(|group| Ok(group));
        mock.expect_get_user()
            .returning(move |_| Ok(Some(user.clone())));

        let auth = AuthState::Authorized(claims);
        let usecase = UseCase::new(Arc::new(mock));
        let schema = Schema::build(
            GroupQuery::default(),
            GroupMutation::default(),
            EmptySubscription,
        )
        .data(auth)
        .data(usecase)
        .finish();

        let query = indoc! {r#"
            mutation CreateGroup($input: CreateGroupInput!) {
                createGroup(input: $input) {
                    id
                    createdAt
                    updatedAt
                    title
                    participants {
                        id
                    }
                }
            }
        "#};
        let vars = Variables::from_json(json!({
            "input": {
                "title": title
            }
        }));

        let req = Request::new(query).variables(vars);
        let res = schema.execute(req).await;

        if let Value::Object(data) = res.data {
            if let Value::Object(group) = &data["createGroup"] {
                assert_eq!(group["title"], expected_title);
                assert_eq!(group["participants"], expected_participant);
            } else {
                unreachable!();
            }
        } else {
            unreachable!();
        }
    }

    #[tokio::test]
    async fn mutation_update_group() {
        let mut group: Group = Faker.fake();
        let user: User = Faker.fake();
        let title: String = Faker.fake();
        let mut claims: Claims = Faker.fake();

        group.participants.push(user.id.clone());
        claims.sub = user.id.to_string();

        let id = group.id.clone();
        let expected_title = Value::String(title.clone());
        let expected_participant = Value::from_json(json!(group
            .participants
            .iter()
            .map(|id| json!({"id": id}))
            .collect::<Vec<_>>()))
        .unwrap();

        let mut mock = MockRepository::new();
        mock.expect_get_group()
            .returning(move |_| Ok(Some(group.clone())));
        mock.expect_update_group().returning(|group| Ok(group));
        mock.expect_get_user().returning(move |id| {
            let mut user = user.clone();
            user.id = id.clone();
            Ok(Some(user.clone()))
        });

        let auth = AuthState::Authorized(claims);
        let usecase = UseCase::new(Arc::new(mock));
        let schema = Schema::build(
            GroupQuery::default(),
            GroupMutation::default(),
            EmptySubscription,
        )
        .data(auth)
        .data(usecase)
        .finish();

        let query = indoc! {r#"
            mutation UpdateGroup($input: UpdateGroupInput!) {
                updateGroup(input: $input) {
                    id
                    createdAt
                    updatedAt
                    title
                    participants {
                        id
                    }
                }
            }
        "#};
        let vars = Variables::from_json(json!({
            "input": {
                "id": id,
                "title": title,
            }
        }));

        let req = Request::new(query).variables(vars);
        let res = schema.execute(req).await;

        if let Value::Object(data) = res.data {
            if let Value::Object(group) = &data["updateGroup"] {
                assert_eq!(group["title"], expected_title);
                assert_eq!(group["participants"], expected_participant);
            } else {
                unreachable!();
            }
        } else {
            unreachable!();
        }
    }

    #[tokio::test]
    async fn mutation_delete_group() {
        let user: User = Faker.fake();
        let mut group: Group = Faker.fake();
        let mut claims: Claims = Faker.fake();

        group.participants.push(user.id.clone());
        claims.sub = user.id.to_string();

        let id = group.id.clone();
        let expected = Value::String(group.id.to_string());

        let mut mock: MockRepository = MockRepository::new();
        mock.expect_get_group()
            .returning(move |_| Ok(Some(group.clone())));
        mock.expect_delete_group().returning(|_| Ok(()));
        mock.expect_get_payments_by_group()
            .returning(|_| Ok(vec![]));
        mock.expect_get_notifications_by_group()
            .returning(|_| Ok(vec![]));

        let auth = AuthState::Authorized(claims);
        let usecase = UseCase::new(Arc::new(mock));
        let schema = Schema::build(
            GroupQuery::default(),
            GroupMutation::default(),
            EmptySubscription,
        )
        .data(auth)
        .data(usecase)
        .finish();

        let query = indoc! {r#"
            mutation DeleteGroup($input: DeleteGroupInput!) {
                deleteGroup(input: $input)
            }
        "#};
        let vars = Variables::from_json(json!({
            "input": {
                "id": id.to_string()
            }
        }));

        let req = Request::new(query).variables(vars);
        let res = schema.execute(req).await;

        if let Value::Object(data) = res.data {
            assert_eq!(data["deleteGroup"], expected);
        } else {
            unreachable!();
        }
    }
}
