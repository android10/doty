use clap::{Parser, Subcommand};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Bootstrap {  } => {
            // Ok("doty: bootstrap!!!".to_string())
            println!("doty: bootstrap!!!");
            // FeatureBootstrap { }.run()
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

    doty::run_feature("bootstrap");
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
    Bootstrap,                          // --bootstrap
    ListProfiles,                       // --list-profiles
    InstallProfile { profile: String }, // --install-profile
    SanityCheck,                        // --sanity-check
}

mod ui {
    fn _print_and_color_stderr() {
        unimplemented!()
    }
    
    fn _print_and_color_stdout() {
        unimplemented!()
    }
}