use std::env;
use std::path::MAIN_SEPARATOR_STR;
use std::fs::metadata;
use std::fmt::Display;

use crate::core::error::DotyError;

const USSER_HOME_ENV: &str = "HOME";
const DOTFILES_DIR_NAME: &str = "dotfiles";


/// The main building block and execution unit.
/// It represents 'something' that could be executed. 
/// It basically follows the rules of a `Command Pattern`.
///
/// # Examples
///
/// ```
/// pub struct MyUseCase;
///
/// impl UseCase<String, String, String> for MyUseCase {
///    fn run(&self, params: ()) -> Result<String, String> {
///        // we do our fancy stuff here
///    }
/// }
pub trait UseCase<T, Params, E> where E: Display {

    ///
    /// 
    /// 
    fn run(&self, params: Params) -> Result<T, E>;

    /// Get and validate that the `dotfiles` directoy 
    /// exist in the user `$HOME` directory. 
    fn dotfiles_dir(&self) -> Result<String, DotyError> {
        let mut dotfiles_dir = env::var(USSER_HOME_ENV)
            .map_err(|_| DotyError::DotfilesInvalidDir)?;

        dotfiles_dir.push_str(MAIN_SEPARATOR_STR);
        dotfiles_dir.push_str(DOTFILES_DIR_NAME);

        metadata(&dotfiles_dir)
            .map_err(|_| DotyError::DotfilesInvalidDir)?;

        Ok(dotfiles_dir)
    }
}