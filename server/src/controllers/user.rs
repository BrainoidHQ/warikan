use crate::{
    entities::{AuthState, User, UserID},
    usecases::{CreateUserInput, DeleteUserInput, UpdateUserInput, UseCase},
};
use async_graphql::{Context, Object};
use chrono::{DateTime, Utc};

#[Object]
impl User {
    async fn id(&self) -> UserID {
        self.id.clone()
    }

    async fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    async fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    async fn name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn user(&self, ctx: &Context<'_>, id: UserID) -> async_graphql::Result<Option<User>> {
        let usecase = ctx.data::<UseCase>()?;
        let auth = ctx.data::<AuthState>()?;
        let user = usecase.get_user_opt(auth, &id).await?;
        Ok(user)
    }
}

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    async fn create_user(
        &self,
        ctx: &Context<'_>,
        input: CreateUserInput,
    ) -> async_graphql::Result<User> {
        let usecase = ctx.data::<UseCase>()?;
        let auth = ctx.data::<AuthState>()?;
        Ok(usecase.create_user(auth, input).await?)
    }

    async fn update_user(
        &self,
        ctx: &Context<'_>,
        input: UpdateUserInput,
    ) -> async_graphql::Result<User> {
        let usecase = ctx.data::<UseCase>()?;
        let auth = ctx.data::<AuthState>()?;
        Ok(usecase.update_user(auth, input).await?)
    }

    async fn delete_user(
        &self,
        ctx: &Context<'_>,
        input: DeleteUserInput,
    ) -> async_graphql::Result<UserID> {
        let usecase = ctx.data::<UseCase>()?;
        let auth = ctx.data::<AuthState>()?;
        Ok(usecase.delete_user(auth, input).await?)
    }
}
