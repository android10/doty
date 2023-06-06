use clap::{Command, Arg};

fn main() {
    let matches = Command::new("doty")
        .version("0.1.0")
        .author("Fernando Cejas <android10@fernandocejas.com>")
        .about("Doty is a .dotfiles Manager built with love in Rust")
        .arg(
            Arg::new("list-profiles")
                .short('l')
                .long("list-profiles")
                .help("List profiles defines in 'doty.toml' file.")
                .num_args(0)
        )
        .arg(
            Arg::new("install-profile")
                .short('i')
                .long("install-profile")
                .help("Install a <profile> define in 'doty.<profile>.toml' file.")
                .num_args(1)
                .value_name("profile")
        )
        .arg(
            Arg::new("stats")
                .short('s')
                .long("stats")
                .help("Display general CLI stats.")
                .num_args(0)
        )
        .get_matches();

    println!("{:#?}", matches);
}
