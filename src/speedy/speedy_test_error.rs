use std::fmt;
use std::error::Error;
use reqwest::{Error as ReqwestError};

#[derive(Debug)]
pub struct SpeedyTestError {
  pub message: String,
}

impl From<ReqwestError> for SpeedyTestError {
  fn from(error: ReqwestError) -> Self {
    SpeedyTestError {
      message: error.status().unwrap().to_string()
    }
  }
}

impl fmt::Display for SpeedyTestError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", format!("The request returned {}", self.message))
  }
}

impl Error for SpeedyTestError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    Some(self)
  }
}
