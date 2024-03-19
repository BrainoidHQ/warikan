use crate::{
    entities::{Amount, AuthState, Payment, PaymentID, User},
    usecases::{CreatePaymentInput, DeletePaymentInput, UpdatePaymentInput, UseCase},
};
use async_graphql::{Context, Object};
use chrono::{DateTime, Utc};

#[Object]
impl Payment {
    async fn id(&self) -> PaymentID {
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

    async fn creditors(&self) -> Vec<Amount> {
        self.creditors.clone()
    }

    async fn debtors(&self) -> Vec<Amount> {
        self.debtors.clone()
    }
}

#[Object]
impl Amount {
    async fn user(&self, ctx: &Context<'_>) -> async_graphql::Result<User> {
        let usecase = ctx.data::<UseCase>()?;
        let auth = ctx.data::<AuthState>()?;
        let user = usecase.get_user(auth, &self.user).await?;
        Ok(user)
    }

    async fn amount(&self) -> i32 {
        self.amount
    }
}

#[derive(Default)]
pub struct PaymentQuery;

#[Object]
impl PaymentQuery {
    async fn payment(
        &self,
        ctx: &Context<'_>,
        id: PaymentID,
    ) -> async_graphql::Result<Option<Payment>> {
        let usecase = ctx.data::<UseCase>()?;
        let auth = ctx.data::<AuthState>()?;
        let payment = usecase.get_payment(auth, &id).await?;
        Ok(payment)
    }
}

#[derive(Default)]
pub struct PaymentMutation;

#[Object]
impl PaymentMutation {
    async fn create_payment(
        &self,
        ctx: &Context<'_>,
        input: CreatePaymentInput,
    ) -> async_graphql::Result<Payment> {
        let usecase = ctx.data::<UseCase>()?;
        let auth = ctx.data::<AuthState>()?;
        Ok(usecase.create_payment(auth, input).await?)
    }

    async fn update_payment(
        &self,
        ctx: &Context<'_>,
        input: UpdatePaymentInput,
    ) -> async_graphql::Result<Payment> {
        let usecase = ctx.data::<UseCase>()?;
        let auth = ctx.data::<AuthState>()?;
        Ok(usecase.update_payment(auth, input).await?)
    }

    async fn delete_payment(
        &self,
        ctx: &Context<'_>,
        input: DeletePaymentInput,
    ) -> async_graphql::Result<PaymentID> {
        let usecase = ctx.data::<UseCase>()?;
        let auth = ctx.data::<AuthState>()?;
        Ok(usecase.delete_payment(auth, input).await?)
    }
}
