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

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub struct UnreachableError;

impl std::fmt::Display for UnreachableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unreachable error")
    }
}

pub trait Node<'a>: Ord + Serialize + Deserialize<'a> {
    fn id(&self) -> &str;
    fn created_at(&self) -> &DateTime<Utc>;
    fn updated_at(&self) -> &DateTime<Utc>;
}
