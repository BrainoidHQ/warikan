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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        entities::{Claims, Group, UserID},
        repositories::MockRepository,
    };
    use async_graphql::{EmptyMutation, EmptySubscription, Request, Schema, Value, Variables};
    use chrono::SecondsFormat;
    use fake::{Fake, Faker};
    use indoc::indoc;
    use serde_json::json;
    use std::sync::Arc;

    impl TryFrom<Notification> for Value {
        type Error = serde_json::Error;

        fn try_from(notification: Notification) -> Result<Self, Self::Error> {
            Value::from_json(json!({
                "id": notification.id,
                "createdAt": notification.created_at.to_rfc3339_opts(SecondsFormat::Nanos, false),
                "updatedAt": notification.updated_at.to_rfc3339_opts(SecondsFormat::Nanos, false),
                "message": notification.message,
            }))
        }
    }

    #[tokio::test]
    async fn query_notification() {
        let notification: Notification = Faker.fake();
        let mut group: Group = Faker.fake();
        let claims: Claims = Faker.fake();

        group.participants.push(UserID::new(&claims.sub));
        let expected = Value::try_from(notification.clone()).unwrap();
        let id = notification.id.clone();

        let mut mock = MockRepository::new();
        mock.expect_get_group()
            .returning(move |_| Ok(Some(group.clone())));
        mock.expect_get_notification()
            .returning(move |_| Ok(Some(notification.clone())));

        let auth = AuthState::Authorized(claims);
        let usecase = UseCase::new(Arc::new(mock));
        let schema = Schema::build(
            NotificationQuery::default(),
            EmptyMutation,
            EmptySubscription,
        )
        .data(auth)
        .data(usecase)
        .finish();

        let query = indoc! {r#"
            query GetNotification($id: ID!) {
                notification(id: $id) {
                    id
                    createdAt
                    updatedAt
                    message
                }
            }
        "#};
        let vars = Variables::from_json(json!({
            "id": id,
        }));

        let req = Request::new(query).variables(vars);
        let res = schema.execute(req).await;

        if let Value::Object(data) = res.data {
            assert_eq!(data["notification"], expected);
        } else {
            unreachable!();
        }
    }
}
