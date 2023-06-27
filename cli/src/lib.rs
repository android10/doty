mod core;
mod features;

use crate::{
    features::feature_bootstrap::FeatureBootstrap, 
    // core::feature::Feature
};

pub mod domain {
    pub fn run_bootstrap() { println!("doty: bootstrap!!!") }    
    pub fn run_list_profiles() { print!("doty: profiles!!!") }    
    pub fn run_install_profile(profile: &str) { print!("doty: install profile: {profile:?}") }    
    pub fn run_sanity_check() { print!("doty: sanity!!!") }    
}