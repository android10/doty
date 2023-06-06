fn main() {
    let _matches = clap::Command::new("doty")
        .version("0.1.0")
        .author("Fernando Cejas <android10@fernandocejas.com>")
        .about("Doty is a .dotfiles Manager in Rust")
        .get_matches();
}
