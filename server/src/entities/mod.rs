mod auth;
mod group;
mod notification;
mod payment;
mod user;

pub use auth::*;
pub use group::*;
pub use notification::*;
pub use payment::*;
pub use user::*;

use thiserror::Error;

#[derive(Debug, Error)]
pub struct UnreachableError;

impl std::fmt::Display for UnreachableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unreachable error")
    }
}
