use clap::{Parser, Subcommand};

use doty::domain::{
    run_feature_bootstrap, 
    run_feature_list_profiles, 
    run_feature_install_profile, 
    run_feature_sanity_check
};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Bootstrap => { 
            match run_feature_bootstrap() {
                Ok(ok) => print!("ok result --> {}", ok),
                Err(error) => print!("error result --> {}", error.to_string())
            }
        }
        Commands::ListProfiles => { run_feature_list_profiles() }
        Commands::InstallProfile { profile } => { run_feature_install_profile(profile) }
        Commands::SanityCheck => { run_feature_sanity_check() }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/**
 * Supported CLI commands and/or 
 * Features/Functionalities.
 */
#[derive(Subcommand)]
enum Commands {
    /// TODO: This is Bootstrap help
    Bootstrap,
    /// TODO:  This is ListProfiles help
    ListProfiles,
    /// TODO: This is InstallProfile help
    InstallProfile { profile: String },
    /// TODO: This is SanityCheck help
    SanityCheck,
}

mod ui {
    fn _print_and_color_stderr() {
        unimplemented!()
    }
    
    fn _print_and_color_stdout() {
        unimplemented!()
    }
}