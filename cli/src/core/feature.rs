use std::env;
use std::path::MAIN_SEPARATOR_STR;

use crate::core::error::DotyError;

pub trait Feature {

    fn run(&self);

    /**
     * Get and validate that the dotfiles directoy exist 
     * in the user $HOME directory. 
     */
    fn get_dotfiles_dir(&self) -> Result<String, DotyError> {    
        match env::var("HOME") {
            Ok(home_dir_path) => Ok(format!("{home_dir}{separator}{dotfiles_dir}{separator}", 
                                            home_dir = home_dir_path, 
                                            separator = MAIN_SEPARATOR_STR, 
                                            dotfiles_dir = "dotfiles")),
            Err(_) => Err(DotyError::DotfilesDirNotFound {}) 
        }
    }
}
