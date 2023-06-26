mod core;
mod features;

use crate::{
    features::feature_bootstrap::FeatureBootstrap, 
    // core::feature::Feature
};

use std::collections::HashMap;

pub fn run_feature(_feature_name: &str) {
    // TODO: Extract this
    let _map = HashMap::from([
        ("one", FeatureBootstrap{}), 
        ("two", FeatureBootstrap{})
      ]);
}