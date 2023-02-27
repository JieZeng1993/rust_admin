//! Errorand Result types.
use std::error::Error;
use std::fmt::{self, Debug, Display};
use std::io;

use serde::de::Visitor;
use serde::ser::{Serialize, Serializer};
use serde::{Deserialize, Deserializer};

pub type AppResult<T> = Result<T, AppError>;

/// A generic error that represents all the ways a method can fail inside of rexpr::core.
#[derive(Debug)]
#[non_exhaustive]
pub enum AppError {
    /// Default Error
    E(String),
}

impl Display for AppError {
    // IntellijRust does not understand that [non_exhaustive] applies only for downstream crates
    // noinspection RsMatchCheck
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::E(error) => write!(f, "{}", error),
        }
    }
}

impl Error for AppError {}

impl From<io::Error> for AppError {
    #[inline]
    fn from(err: io::Error) -> Self {
        AppError::from(err.to_string())
    }
}

impl From<&str> for AppError {
    fn from(arg: &str) -> Self {
        return AppError::E(arg.to_string());
    }
}

impl From<std::string::String> for AppError {
    fn from(arg: String) -> Self {
        return AppError::E(arg);
    }
}

impl From<&dyn std::error::Error> for AppError {
    fn from(arg: &dyn std::error::Error) -> Self {
        return AppError::E(arg.to_string());
    }
}

impl From<AppError> for std::io::Error {
    fn from(arg: AppError) -> Self {
        arg.into()
    }
}

// impl From<rbatis::Error> for AppError {
//     fn from(arg: rbatis::Error) -> Self {
//         AppError::E(arg.to_string())
//     }
// }

// impl From<actix_web::error::Error> for AppError {
//     fn from(arg: actix_web::error::Error) -> Self {
//         AppError::E(arg.to_string())
//     }
// }

impl Clone for AppError {
    fn clone(&self) -> Self {
        AppError::from(self.to_string())
    }

    fn clone_from(&mut self, source: &Self) {
        *self = Self::from(source.to_string());
    }
}

// This is what #[derive(Serialize)] would generate.
impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

struct ErrorVisitor;

impl<'de> Visitor<'de> for ErrorVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: std::error::Error,
    {
        Ok(v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: std::error::Error,
    {
        Ok(v.to_string())
    }
}

impl<'de> Deserialize<'de> for AppError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let r = deserializer.deserialize_string(ErrorVisitor)?;
        return Ok(AppError::from(r));
    }
}
