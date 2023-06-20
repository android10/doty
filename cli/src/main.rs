mod features;
mod core;

use crate::{features::feature_bootstrap::FeatureBootstrap, core::feature::Feature};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}


#[derive(Subcommand)]
enum Commands {
    /// Initialize 'dotfiles' directory.
    Bootstrap { },

    /// List existing 'doty' profiles.
    ListProfiles { },

    /// Install profile from 'doty.<PROFILE>.toml' file.
    InstallProfile { profile: String },

    /// Performs 'dotfiles' dir sanity check.
    SanityCheck { },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Bootstrap {  } => {
            let feature_bootstrap = FeatureBootstrap { };
            feature_bootstrap.run()
        }
        Commands::ListProfiles {  } => {
            println!("doty: profiles!!!")
        }
        Commands::InstallProfile { profile } => {
            println!("doty: install profile: {profile:?}")
        }
        Commands::SanityCheck {  } => {
            println!("doty: sanity!!!")
        }
    }
}
