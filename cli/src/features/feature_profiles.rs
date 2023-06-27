use crate::core::feature::Feature;
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

