use crate::core::feature::Feature;

#[derive(Debug)]
pub struct FeatureSanity { }

impl Feature for FeatureSanity {
    fn run(&self) {
        match self.dotfiles_dir() {
            Ok(_) => println!("doty: sanity!!!"),
            Err(_) => println!("ERROR!")
        }
    }
}