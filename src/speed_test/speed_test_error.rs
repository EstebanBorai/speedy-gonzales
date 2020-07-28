use std::fmt;
use std::error::Error;
use reqwest::{Error as ReqwestError};

#[derive(Debug)]
pub struct SpeedTestError {
  pub message: String,
}

impl From<ReqwestError> for SpeedTestError {
  fn from(error: ReqwestError) -> Self {
    SpeedTestError {
      message: error.status().unwrap().to_string()
    }
  }
}

impl fmt::Display for SpeedTestError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", format!("The request returned {}", self.message))
  }
}

impl Error for SpeedTestError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    Some(self)
  }
}
