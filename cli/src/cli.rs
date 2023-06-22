use clap::{Parser, Subcommand};

use crate::{
    features::feature_bootstrap::FeatureBootstrap, 
    core::feature::Feature
};

// use crate::core::error::DotyError;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/**
 * Supported CLI commands
 */
#[derive(Subcommand)]
enum Commands {
    // --bootstrap
    Bootstrap,

    // --list-profiles
    ListProfiles,

    // --install-profile
    InstallProfile { profile: String },

    // --sanity-check
    SanityCheck,
}

pub fn run_command() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Bootstrap {  } => {
            // Ok("doty: bootstrap!!!".to_string())
            FeatureBootstrap { }.run()
        }
        Commands::ListProfiles {  } => {
            // Ok("doty: profiles!!!".to_string())
            print!("doty: profiles!!!")
        }
        Commands::InstallProfile { profile } => {
            // Ok(format!("doty: install profile: {}", profile))
            print!("doty: install profile: {profile:?}")
        }
        Commands::SanityCheck {  } => {
            // Ok("doty: sanity!!!".to_string())
            print!("doty: sanity!!!")
        }
    }
}

fn print_and_color_stderr() {
    todo!()
}

fn print_and_color_stdout() {
    todo!()
}

