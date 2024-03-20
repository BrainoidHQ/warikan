mod auth;
mod group;
mod notification;
mod payment;
mod user;
mod warikan;

pub use auth::*;
pub use group::*;
pub use notification::*;
pub use payment::*;
pub use user::*;
pub use warikan::*;

use thiserror::Error;

#[derive(Debug, Error)]
pub struct UnreachableError;

impl std::fmt::Display for UnreachableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unreachable error")
    }
}
