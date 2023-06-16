use std::error::Error;

type Either<T> = Result<T, Box<dyn Error>>;