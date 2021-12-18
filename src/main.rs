use clap::App;

fn main() {
    let matches = App::new("fumo").version("0.1.0").get_matches();

    println!("Hello, world!");
}
