use async_graphql::{EmptySubscription, Request, Schema, Value, Variables};
use fake::{Fake, Faker};
use indoc::indoc;
use serde_json::json;
use server::{
    controllers::{Mutation, Query},
    entities::{AuthState, Claims},
    repositories::{MongoRepository, MongoRepositoryConfig},
    usecases::UseCase,
};
use std::sync::Arc;

#[tokio::test]
async fn scenario_simple() {
    // -------------------------------------------------------------------------
    // 0. setup
    // -------------------------------------------------------------------------

    let sub: String = Faker.fake();
    let claims = Claims {
        iss: Faker.fake(),
        sub: sub.clone(),
        aud: Faker.fake(),
        iat: Faker.fake(),
        exp: Faker.fake(),
        azp: Faker.fake(),
        scope: Faker.fake(),
    };
    let auth = AuthState::Authorized(claims);

    let mongo = MongoRepository::new(MongoRepositoryConfig {
        uri: "mongodb://localhost:27017",
        database: "warikan",
    })
    .await
    .unwrap();
    let usecase = UseCase::new(Arc::new(mongo));

    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(auth)
        .data(usecase)
        .finish();

    // -------------------------------------------------------------------------
    // 1. create user
    // -------------------------------------------------------------------------

    let query = indoc! {r#"
        mutation CreateUser($input: CreateUserInput!) {
            createUser(input: $input) {
                id
            }
        }
    "#};
    let vars = Variables::from_json(json!({
        "input": {
            "name": "user",
        },
    }));

    let req = Request::new(query).variables(vars);
    let res = schema.execute(req).await;

    if let Value::Object(data) = res.data {
        if let Value::Object(ref user) = data["createUser"] {
            assert_eq!(user["id"], Value::String(sub.clone()));
        } else {
            unreachable!();
        }
    } else {
        unreachable!();
    }

    // -------------------------------------------------------------------------
    // 2. read user
    // -------------------------------------------------------------------------

    let query = indoc! {r#"
        query GetUser($id: ID!) {
            user(id: $id) {
                name
            }
        }
    "#};
    let vars = Variables::from_json(json!({
        "id": &sub,
    }));

    let req = Request::new(query).variables(vars);
    let res = schema.execute(req).await;

    if let Value::Object(data) = res.data {
        if let Value::Object(ref user) = data["user"] {
            assert_eq!(user["name"], Value::String("user".to_string()));
        } else {
            unreachable!();
        }
    } else {
        unreachable!();
    }

    // -------------------------------------------------------------------------
    // 3. update user
    // -------------------------------------------------------------------------

    let query = indoc! {r#"
        mutation UpdateUser($input: UpdateUserInput!) {
            updateUser(input: $input) {
                name
            }
        }
    "#};
    let vars = Variables::from_json(json!({
        "input": {
            "id": &sub,
            "name": "user1",
        },
    }));

    let req = Request::new(query).variables(vars);
    let res = schema.execute(req).await;

    if let Value::Object(data) = res.data {
        if let Value::Object(ref user) = data["updateUser"] {
            assert_eq!(user["name"], Value::String("user1".to_string()));
        } else {
            unreachable!();
        }
    } else {
        unreachable!();
    }

    // -------------------------------------------------------------------------
    // 4. create group
    // -------------------------------------------------------------------------

    let query = indoc! {r#"
        mutation CreateGroup($input: CreateGroupInput!) {
            createGroup(input: $input) {
                id
                title
            }
        }
    "#};
    let vars = Variables::from_json(json!({
        "input": {
            "title": "group",
        },
    }));

    let req = Request::new(query).variables(vars);
    let res = schema.execute(req).await;

    let group_id = if let Value::Object(data) = res.data {
        if let Value::Object(ref group) = data["createGroup"] {
            assert_eq!(group["title"], Value::String("group".to_string()));
            if let Value::String(ref id) = group["id"] {
                id.to_string()
            } else {
                unreachable!();
            }
        } else {
            unreachable!();
        }
    } else {
        unreachable!();
    };

    // -------------------------------------------------------------------------
    // 5. read group
    // -------------------------------------------------------------------------

    let query = indoc! {r#"
        query GetGroup($id: ID!) {
            group(id: $id) {
                title
            }
        }
    "#};
    let vars = Variables::from_json(json!({
        "id": &group_id,
    }));

    let req = Request::new(query).variables(vars);
    let res = schema.execute(req).await;

    if let Value::Object(data) = res.data {
        if let Value::Object(ref group) = data["group"] {
            assert_eq!(group["title"], Value::String("group".to_string()));
        } else {
            unreachable!();
        }
    } else {
        unreachable!();
    }

    // -------------------------------------------------------------------------
    // 6. update group
    // -------------------------------------------------------------------------

    let query = indoc! {r#"
        mutation UpdateGroup($input: UpdateGroupInput!) {
            updateGroup(input: $input) {
                title
            }
        }
    "#};
    let vars = Variables::from_json(json!({
        "input": {
            "id": &group_id,
            "title": "group1",
        },
    }));

    let req = Request::new(query).variables(vars);
    let res = schema.execute(req).await;

    if let Value::Object(data) = res.data {
        if let Value::Object(ref group) = data["updateGroup"] {
            assert_eq!(group["title"], Value::String("group1".to_string()));
        } else {
            unreachable!();
        }
    } else {
        unreachable!();
    }

    // -------------------------------------------------------------------------
    // 7. create payment
    // -------------------------------------------------------------------------

    let query = indoc! {r#"
        mutation CreatePayment($input: CreatePaymentInput!) {
            createPayment(input: $input) {
                id
                title
            }
        }
    "#};
    let vars = Variables::from_json(json!({
        "input": {
            "title": "payment",
            "group": &group_id,
        },
    }));

    let req = Request::new(query).variables(vars);
    let res = schema.execute(req).await;

    let payment_id = if let Value::Object(data) = res.data {
        if let Value::Object(ref payment) = data["createPayment"] {
            assert_eq!(payment["title"], Value::String("payment".to_string()));
            if let Value::String(ref id) = payment["id"] {
                id.to_string()
            } else {
                unreachable!();
            }
        } else {
            unreachable!();
        }
    } else {
        unreachable!();
    };

    // -------------------------------------------------------------------------
    // 8. read payment
    // -------------------------------------------------------------------------

    let query = indoc! {r#"
        query GetPayment($id: ID!) {
            payment(id: $id) {
                title
            }
        }
    "#};
    let vars = Variables::from_json(json!({
        "id": &payment_id,
    }));

    let req = Request::new(query).variables(vars);
    let res = schema.execute(req).await;

    if let Value::Object(data) = res.data {
        if let Value::Object(ref payment) = data["payment"] {
            assert_eq!(payment["title"], Value::String("payment".to_string()));
        } else {
            unreachable!();
        }
    } else {
        unreachable!();
    }

    // -------------------------------------------------------------------------
    // 9. update payment
    // -------------------------------------------------------------------------

    let query = indoc! {r#"
        mutation UpdatePayment($input: UpdatePaymentInput!) {
            updatePayment(input: $input) {
                title
            }
        }
    "#};
    let vars = Variables::from_json(json!({
        "input": {
            "id": &payment_id,
            "title": "payment1",
        },
    }));

    let req = Request::new(query).variables(vars);
    let res = schema.execute(req).await;

    if let Value::Object(data) = res.data {
        if let Value::Object(ref payment) = data["updatePayment"] {
            assert_eq!(payment["title"], Value::String("payment1".to_string()));
        } else {
            unreachable!();
        }
    } else {
        unreachable!();
    }

    // -------------------------------------------------------------------------
    // 10. delete payment
    // -------------------------------------------------------------------------

    let query = indoc! {r#"
        mutation DeletePayment($input: DeletePaymentInput!) {
            deletePayment(input: $input)
        }
    "#};
    let vars = Variables::from_json(json!({
        "input": {
            "id": &payment_id,
        },
    }));

    let req = Request::new(query).variables(vars);
    let res = schema.execute(req).await;

    if let Value::Object(data) = res.data {
        assert_eq!(data["deletePayment"], Value::String(payment_id));
    } else {
        unreachable!();
    }

    // -------------------------------------------------------------------------
    // 11. delete group
    // -------------------------------------------------------------------------

    let query = indoc! {r#"
        mutation DeleteGroup($input: DeleteGroupInput!) {
            deleteGroup(input: $input)
        }
    "#};
    let vars = Variables::from_json(json!({
        "input": {
            "id": &group_id,
        },
    }));

    let req = Request::new(query).variables(vars);
    let res = schema.execute(req).await;

    if let Value::Object(data) = res.data {
        assert_eq!(data["deleteGroup"], Value::String(group_id));
    } else {
        unreachable!();
    }

    // -------------------------------------------------------------------------
    // 12. delete user
    // -------------------------------------------------------------------------

    let query = indoc! {r#"
        mutation DeleteUser($input: DeleteUserInput!) {
            deleteUser(input: $input)
        }
    "#};
    let vars = Variables::from_json(json!({
        "input": {
            "id": &sub,
        },
    }));

    let req = Request::new(query).variables(vars);
    let res = schema.execute(req).await;

    if let Value::Object(data) = res.data {
        assert_eq!(data["deleteUser"], Value::String(sub));
    } else {
        unreachable!();
    }
}
