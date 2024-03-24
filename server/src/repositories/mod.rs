#[cfg(feature = "mongodb")]
mod mongo;
#[cfg(feature = "mongodb")]
pub use mongo::*;

use crate::entities::{
    Group, GroupID, Notification, NotificationID, Payment, PaymentID, User, UserID,
};
use async_trait::async_trait;

#[cfg(test)]
use mockall::predicate::*;
#[cfg(test)]
use mockall::*;

#[async_trait]
pub trait Repository:
    GroupRepository + NotificationRepository + PaymentRepository + UserRepository + Send + Sync
{
}

impl<
        T: GroupRepository + NotificationRepository + PaymentRepository + UserRepository + Send + Sync,
    > Repository for T
{
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait GroupRepository: Send + Sync {
    async fn create_group(
        &self,
        group: Group,
    ) -> Result<Group, Box<dyn std::error::Error + Send + Sync>>;

    async fn update_group(
        &self,
        group: Group,
    ) -> Result<Group, Box<dyn std::error::Error + Send + Sync>>;

    async fn delete_group(
        &self,
        id: &GroupID,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    async fn get_group(
        &self,
        id: &GroupID,
    ) -> Result<Option<Group>, Box<dyn std::error::Error + Send + Sync>>;

    async fn get_groups_by_user(
        &self,
        id: &UserID,
    ) -> Result<Vec<Group>, Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait NotificationRepository: Send + Sync {
    async fn create_notification(
        &self,
        notification: Notification,
    ) -> Result<Notification, Box<dyn std::error::Error + Send + Sync>>;

    async fn delete_notification(
        &self,
        id: &NotificationID,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    async fn get_notification(
        &self,
        id: &NotificationID,
    ) -> Result<Option<Notification>, Box<dyn std::error::Error + Send + Sync>>;

    async fn get_notifications_by_group(
        &self,
        group: &GroupID,
    ) -> Result<Vec<Notification>, Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait PaymentRepository: Send + Sync {
    async fn create_payment(
        &self,
        payment: Payment,
    ) -> Result<Payment, Box<dyn std::error::Error + Send + Sync>>;

    async fn update_payment(
        &self,
        payment: Payment,
    ) -> Result<Payment, Box<dyn std::error::Error + Send + Sync>>;

    async fn delete_payment(
        &self,
        id: &PaymentID,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    async fn get_payment(
        &self,
        id: &PaymentID,
    ) -> Result<Option<Payment>, Box<dyn std::error::Error + Send + Sync>>;

    async fn get_payments_by_group(
        &self,
        group: &GroupID,
    ) -> Result<Vec<Payment>, Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait UserRepository: Send + Sync {
    async fn create_user(
        &self,
        user: User,
    ) -> Result<User, Box<dyn std::error::Error + Send + Sync>>;

    async fn update_user(
        &self,
        user: User,
    ) -> Result<User, Box<dyn std::error::Error + Send + Sync>>;

    async fn delete_user(
        &self,
        id: &UserID,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    async fn get_user(
        &self,
        id: &UserID,
    ) -> Result<Option<User>, Box<dyn std::error::Error + Send + Sync>>;
}

#[cfg(test)]
mock! {
    pub Repository {}

    #[async_trait]
    impl GroupRepository for Repository {
        async fn create_group(
            &self,
            group: Group,
        ) -> Result<Group, Box<dyn std::error::Error + Send + Sync>>;

        async fn update_group(
            &self,
            group: Group,
        ) -> Result<Group, Box<dyn std::error::Error + Send + Sync>>;

        async fn delete_group(
            &self,
            id: &GroupID,
        ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

        async fn get_group(
            &self,
            id: &GroupID,
        ) -> Result<Option<Group>, Box<dyn std::error::Error + Send + Sync>>;

        async fn get_groups_by_user(
            &self,
            id: &UserID,
        ) -> Result<Vec<Group>, Box<dyn std::error::Error + Send + Sync>>;
    }

    #[async_trait]
    impl NotificationRepository for Repository {
        async fn create_notification(
            &self,
            notification: Notification,
        ) -> Result<Notification, Box<dyn std::error::Error + Send + Sync>>;

        async fn delete_notification(
            &self,
            id: &NotificationID,
        ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

        async fn get_notification(
            &self,
            id: &NotificationID,
        ) -> Result<Option<Notification>, Box<dyn std::error::Error + Send + Sync>>;

        async fn get_notifications_by_group(
            &self,
            group: &GroupID,
        ) -> Result<Vec<Notification>, Box<dyn std::error::Error + Send + Sync>>;
    }

    #[async_trait]
    impl PaymentRepository for Repository {
        async fn create_payment(
            &self,
            payment: Payment,
        ) -> Result<Payment, Box<dyn std::error::Error + Send + Sync>>;

        async fn update_payment(
            &self,
            payment: Payment,
        ) -> Result<Payment, Box<dyn std::error::Error + Send + Sync>>;

        async fn delete_payment(
            &self,
            id: &PaymentID,
        ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

        async fn get_payment(
            &self,
            id: &PaymentID,
        ) -> Result<Option<Payment>, Box<dyn std::error::Error + Send + Sync>>;

        async fn get_payments_by_group(
            &self,
            group: &GroupID,
        ) -> Result<Vec<Payment>, Box<dyn std::error::Error + Send + Sync>>;
    }

    #[async_trait]
    impl UserRepository for Repository {
        async fn create_user(
            &self,
            user: User,
        ) -> Result<User, Box<dyn std::error::Error + Send + Sync>>;

        async fn update_user(
            &self,
            user: User,
        ) -> Result<User, Box<dyn std::error::Error + Send + Sync>>;

        async fn delete_user(
            &self,
            id: &UserID,
        ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

        async fn get_user(
            &self,
            id: &UserID,
        ) -> Result<Option<User>, Box<dyn std::error::Error + Send + Sync>>;
    }
}
