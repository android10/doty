use clap::{Parser, Subcommand};

use doty::domain::{run_bootstrap, run_list_profiles, run_install_profile, run_sanity_check};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Bootstrap => { run_bootstrap() }
        Commands::ListProfiles => { run_list_profiles() }
        Commands::InstallProfile { profile } => { run_install_profile(profile) }
        Commands::SanityCheck => { run_sanity_check() }
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
 * Supported CLI commands
 */
#[derive(Subcommand)]
enum Commands {
    /// This is Bootstrap help
    Bootstrap,
    /// This is ListProfiles help
    ListProfiles,
    /// This is InstallProfile help
    InstallProfile { profile: String },
    /// This is SanityCheck help
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