use std::fmt;

#[derive(Debug)]
pub enum DotyError {
  DotfilesDirNotFound,
  FeatureError,
}

impl std::error::Error for DotyError { }

impl fmt::Display for DotyError {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    match self {
      DotyError::DotfilesDirNotFound => write!(formatter, "Doty: Dotfiles Directory Not Found"),
      DotyError::FeatureError => write!(formatter, "Doty: Feature Error"),
    }
  }
}