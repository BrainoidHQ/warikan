#[cfg(feature = "mongodb")]
mod mongo;
#[cfg(feature = "mongodb")]
pub use mongo::*;

use crate::entities::{
    Group, GroupID, Notification, NotificationID, Payment, PaymentID, User, UserID,
};
use async_trait::async_trait;

#[cfg(test)]
use fake::{Fake, Faker};
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

#[cfg(test)]
pub struct GroupRepositoryTester<R: GroupRepository> {
    pub repository: R,
}

#[cfg(test)]
impl<R: GroupRepository> GroupRepositoryTester<R> {
    pub async fn test(repository: R) {
        let tester = Self { repository };
        tester.create_group().await;
        tester.update_group().await;
        tester.delete_group().await;
        tester.get_groups_by_user().await;
    }

    async fn create_group(&self) {
        let group: Group = Faker.fake();

        let create = self.repository.create_group(group).await.unwrap();
        let get = self.repository.get_group(&create.id).await.unwrap();

        assert_eq!(Some(create), get);
    }

    async fn update_group(&self) {
        let group1: Group = Faker.fake();
        let mut group2: Group = Faker.fake();
        group2.id = group1.id.clone();

        let create = self.repository.create_group(group1).await.unwrap();
        let update = self.repository.update_group(group2).await.unwrap();
        let get = self.repository.get_group(&create.id).await.unwrap();

        assert_eq!(Some(update), get);
    }

    async fn delete_group(&self) {
        let group: Group = Faker.fake();

        let create = self.repository.create_group(group).await.unwrap();
        self.repository.delete_group(&create.id).await.unwrap();
        let delete = self.repository.get_group(&create.id).await.unwrap();

        assert_eq!(delete, None);
    }

    async fn get_groups_by_user(&self) {
        let user: UserID = Faker.fake();

        let mut group1: Group = Faker.fake();
        let mut group2: Group = Faker.fake();
        let group3: Group = Faker.fake();

        group1.participants.push(user.clone());
        group2.participants.push(user.clone());

        let _ = self.repository.create_group(group1.clone()).await.unwrap();
        let _ = self.repository.create_group(group2.clone()).await.unwrap();
        let _ = self.repository.create_group(group3).await.unwrap();

        let groups = self.repository.get_groups_by_user(&user).await.unwrap();

        assert_eq!(groups, vec![group1, group2]);
    }
}

#[cfg(test)]
pub struct NotificationRepositoryTester<R: NotificationRepository> {
    pub repository: R,
}

#[cfg(test)]
impl<R: NotificationRepository> NotificationRepositoryTester<R> {
    pub async fn test(repository: R) {
        let tester = Self { repository };
        tester.create_notification().await;
        tester.delete_notification().await;
        tester.get_notifications_by_group().await;
    }

    async fn create_notification(&self) {
        let notification: Notification = Faker.fake();

        let create = self
            .repository
            .create_notification(notification)
            .await
            .unwrap();
        let get = self.repository.get_notification(&create.id).await.unwrap();

        assert_eq!(Some(create), get);
    }

    async fn delete_notification(&self) {
        let notification: Notification = Faker.fake();

        let create = self
            .repository
            .create_notification(notification)
            .await
            .unwrap();
        self.repository
            .delete_notification(&create.id)
            .await
            .unwrap();
        let delete = self.repository.get_notification(&create.id).await.unwrap();

        assert_eq!(delete, None);
    }

    async fn get_notifications_by_group(&self) {
        let mut notification1: Notification = Faker.fake();
        let mut notification2: Notification = Faker.fake();
        let notification3: Notification = Faker.fake();

        let group: GroupID = Faker.fake();
        notification1.group = group.clone();
        notification2.group = group.clone();

        self.repository
            .create_notification(notification1.clone())
            .await
            .unwrap();
        self.repository
            .create_notification(notification2.clone())
            .await
            .unwrap();
        self.repository
            .create_notification(notification3.clone())
            .await
            .unwrap();

        let get = self
            .repository
            .get_notifications_by_group(&group)
            .await
            .unwrap();

        assert_eq!(vec![notification1, notification2], get);
    }
}

#[cfg(test)]
pub struct PaymentRepositoryTester<R: PaymentRepository> {
    pub repository: R,
}

#[cfg(test)]
impl<R: PaymentRepository> PaymentRepositoryTester<R> {
    pub async fn test(repository: R) {
        let tester = Self { repository };
        tester.create_payment().await;
        tester.update_payment().await;
        tester.delete_payment().await;
        tester.get_payments_by_group().await;
    }

    async fn create_payment(&self) {
        let payment: Payment = Faker.fake();

        let create = self.repository.create_payment(payment).await.unwrap();
        let get = self.repository.get_payment(&create.id).await.unwrap();

        assert_eq!(Some(create), get);
    }

    async fn update_payment(&self) {
        let payment1: Payment = Faker.fake();
        let mut payment2: Payment = Faker.fake();
        payment2.id = payment1.id.clone();

        let create = self.repository.create_payment(payment1).await.unwrap();
        let update = self.repository.update_payment(payment2).await.unwrap();
        let get = self.repository.get_payment(&create.id).await.unwrap();

        assert_eq!(Some(update), get);
    }

    async fn delete_payment(&self) {
        let payment: Payment = Faker.fake();

        let create = self.repository.create_payment(payment).await.unwrap();
        self.repository.delete_payment(&create.id).await.unwrap();
        let delete = self.repository.get_payment(&create.id).await.unwrap();

        assert_eq!(delete, None);
    }

    async fn get_payments_by_group(&self) {
        let mut payment1: Payment = Faker.fake();
        let mut payment2: Payment = Faker.fake();
        let payment3: Payment = Faker.fake();

        let group: GroupID = Faker.fake();
        payment1.group = group.clone();
        payment2.group = group.clone();

        self.repository
            .create_payment(payment1.clone())
            .await
            .unwrap();
        self.repository
            .create_payment(payment2.clone())
            .await
            .unwrap();
        self.repository
            .create_payment(payment3.clone())
            .await
            .unwrap();

        let get = self.repository.get_payments_by_group(&group).await.unwrap();

        assert_eq!(vec![payment1, payment2], get);
    }
}

#[cfg(test)]
pub struct UserRepositoryTester<R: UserRepository> {
    pub repository: R,
}

#[cfg(test)]
impl<R: UserRepository> UserRepositoryTester<R> {
    pub async fn test(repository: R) {
        let tester = Self { repository };
        tester.create_user().await;
        tester.update_user().await;
        tester.delete_user().await;
    }

    async fn create_user(&self) {
        let user: User = Faker.fake();

        let create = self.repository.create_user(user).await.unwrap();
        let get = self.repository.get_user(&create.id).await.unwrap();

        assert_eq!(Some(create), get);
    }

    async fn update_user(&self) {
        let user1: User = Faker.fake();
        let mut user2: User = Faker.fake();
        user2.id = user1.id.clone();

        let create = self.repository.create_user(user1).await.unwrap();
        let update = self.repository.update_user(user2).await.unwrap();
        let get = self.repository.get_user(&create.id).await.unwrap();

        assert_eq!(Some(update), get);
    }

    async fn delete_user(&self) {
        let user: User = Faker.fake();

        let create = self.repository.create_user(user).await.unwrap();
        self.repository.delete_user(&create.id).await.unwrap();
        let delete = self.repository.get_user(&create.id).await.unwrap();

        assert_eq!(delete, None);
    }
}
