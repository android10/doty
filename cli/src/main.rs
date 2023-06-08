use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Cli {
    #[arg(short, long)]
    install: Option<String>,
}

fn main() {
    let _cli = Cli::parse();

    // println!("two: {:?}", cli.two);
    // println!("one: {:?}", cli.one);
}
