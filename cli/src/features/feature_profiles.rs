use crate::core::feature::Feature;
use crate::core::interactor::UseCase;
// #[derive(Debug)]
pub struct FeatureListProfiles { }

impl Feature for FeatureListProfiles {
    
    fn run(&self) {
        match self.dotfiles_dir() {
            Ok(dotfiles_dir) => println!("This is the DIR: {}", dotfiles_dir),
            Err(_) => println!("INVALID dotfiles dir.")
        }
    }
}

pub struct ListProfilesUseCase;

impl UseCase<Vec<String>, (), String> for ListProfilesUseCase {
    fn run(&self, _params: ()) -> Result<Vec<String>, String> {
        todo!()
    }
}