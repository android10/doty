mod core;
mod features;

use crate::{
    features::feature_bootstrap::FeatureBootstrap, 
    features::feature_profiles::FeatureListProfiles, 
};

pub mod domain {
    pub use crate::core::error::DotyError;
    pub use crate::core::feature::FeatureResult;

    pub fn run_feature_bootstrap() -> Result<String, String> { 
        // println!("doty: bootstrap!!!") 
        // unimplemented!()
        // Ok("this is an ok".to_string())
        Err("this is an error".to_string())
    }

    pub fn run_feature_list_profiles() { print!("doty: profiles!!!") }
    
    pub fn run_feature_install_profile(profile: &str) { print!("doty: install profile: {profile:?}") }
    
    pub fn run_feature_sanity_check() { print!("doty: sanity!!!") }
}