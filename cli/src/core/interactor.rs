use std::fmt::Display;

pub trait UseCase<T, Params, E> where T: Display, E: Display {
    fn run(&self, params: Params) -> Result<T, E>;
}