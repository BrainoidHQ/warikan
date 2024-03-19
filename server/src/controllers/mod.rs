mod group;
mod notification;
mod payment;
mod user;

pub use group::*;
pub use notification::*;
pub use payment::*;
pub use user::*;

use crate::{
    app,
    entities::{AuthState, Claims},
};
use async_graphql::{http::GraphiQLSource, MergedObject};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    extract::State,
    http::request::Parts,
    response::{self, IntoResponse},
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

#[derive(Default, MergedObject)]
pub struct Query(GroupQuery, NotificationQuery, PaymentQuery, UserQuery);

#[derive(Default, MergedObject)]
pub struct Mutation(GroupMutation, PaymentMutation, UserMutation);

pub async fn graphql(
    State(state): State<app::State>,
    auth: AuthState,
    req: GraphQLRequest,
) -> GraphQLResponse {
    dbg!(&auth);
    let req = req.into_inner().data(auth);
    state.schema.execute(req).await.into()
}

pub async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

#[async_trait]
impl FromRequestParts<app::State> for AuthState {
    type Rejection = ();

    async fn from_request_parts(
        parts: &mut Parts,
        state: &app::State,
    ) -> Result<Self, Self::Rejection> {
        Ok(match validate(parts, state).await {
            Ok(claims) => AuthState::Authorized(claims),
            Err(err) => {
                dbg!(err);
                AuthState::Unauthorized
            }
        })
    }
}

async fn validate(
    parts: &mut Parts,
    state: &app::State,
) -> Result<Claims, Box<dyn std::error::Error>> {
    let TypedHeader(Authorization(bearer)) = parts
        .extract::<TypedHeader<Authorization<Bearer>>>()
        .await?;
    let token = bearer.token();
    let claims = state.validator.validate(token)?;
    Ok(claims)
}
