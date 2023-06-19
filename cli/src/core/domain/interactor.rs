mod functional;
use super::functional::Either;

pub trait UseCase<T> {
    fn run(&self) -> Either<T> {
        todo!("implement this")
    }
}