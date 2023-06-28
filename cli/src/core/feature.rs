use std::env;
use std::path::MAIN_SEPARATOR_STR;
use std::fs::metadata;

use crate::core::error::DotyError;

const USSER_HOME_ENV: &str = "HOME";
const DOTFILES_DIR_NAME: &str = "dotfiles";

pub trait Feature {
    // fn run(&self) -> FeatureResult<String>;
    fn run(&self);

    /**
     * Get and validate that the 'dotfiles' directoy 
     * exist in the user $HOME directory. 
     */
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

pub type FeatureResult<T> = Result<T, DotyError>; 