use std::fmt;

#[derive(Debug)]
pub enum DotyError {
  DotfilesInvalidDir,
  FeatureError(String),
}

impl std::error::Error for DotyError { }

impl fmt::Display for DotyError {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    match self {
      DotyError::DotfilesInvalidDir => write!(formatter, "Doty: Dotfiles Directory Not Found"),
      DotyError::FeatureError(message) => write!(formatter, "Doty Feature Error {}", message),
    }
  }
}