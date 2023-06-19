use std::error::Error;

pub type Either<T> = Result<T, Box<dyn Error>>;