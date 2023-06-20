use crate::core::feature::Feature;

#[derive(Debug)]
pub struct FeatureBootstrap { }

impl Feature for FeatureBootstrap {
    fn run(&self) {
        println!("dotfiles dir: {}", self.get_dotfiles_dir().unwrap());
    }
}
