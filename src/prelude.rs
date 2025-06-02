pub use crate::error::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub struct W<T>(pub T);

#[cfg(test)]
pub(crate) use mockall::{automock, predicate::*};
