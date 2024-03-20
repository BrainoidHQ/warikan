mod group;
mod notification;
mod payment;
mod user;

pub use group::*;
pub use notification::*;
pub use payment::*;
pub use user::*;

use crate::repositories::Repository;
use std::sync::Arc;
use thiserror::Error;

pub struct UseCase {
    pub repository: Arc<dyn Repository>,
}

impl UseCase {
    pub fn new(repository: Arc<dyn Repository>) -> Self {
        Self { repository }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum UseCaseError {
    #[error("400 bad request")]
    BadRequest,

    #[error("401 unauthorized")]
    Unauthorized,

    #[error("403 forbidden")]
    Forbidden,

    #[error("404 not found")]
    NotFound,

    #[error("500 internal server error")]
    InternalServerError,
}
