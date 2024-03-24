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
