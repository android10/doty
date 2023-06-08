use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Cli {
    #[arg(short = 'b', long = "bootstrap", help = "Initialize 'dotfiles' directory")]
    bootstrap: bool,

    #[arg(short = 'l', long = "list-profiles", help = "List existing 'doty' profiles")]
    list_profiles: bool,

    #[arg(name= "PROFILE", short = 'i', long = "install-profile", help = "Install from 'doty.<PROFILE>.toml' file")]
    install_profile: Option<String>,

    #[arg(short = 's', long = "sanity-check", help = "Performs 'dotfiles' dir sanity check")]
    sanity_check: bool,
}

fn main() {
    let _cli = Cli::parse();

    // println!("two: {:?}", cli.two);
    // println!("one: {:?}", cli.one);
}
