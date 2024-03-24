use crate::{
    entities::{AuthState, Notification, NotificationID},
    usecases::UseCase,
};
use async_graphql::{Context, Object};
use chrono::{DateTime, Utc};

#[Object]
impl Notification {
    async fn id(&self) -> NotificationID {
        self.id.clone()
    }

    async fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    async fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    async fn message(&self) -> String {
        self.message.clone()
    }
}

#[derive(Default)]
pub struct NotificationQuery;

#[Object]
impl NotificationQuery {
    async fn notification(
        &self,
        ctx: &Context<'_>,
        id: NotificationID,
    ) -> async_graphql::Result<Option<Notification>> {
        let usecase = ctx.data::<UseCase>()?;
        let auth = ctx.data::<AuthState>()?;
        let notification = usecase.get_notification(auth, &id).await?;
        Ok(notification)
    }
}
