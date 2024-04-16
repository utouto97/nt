use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    #[arg(help = "name")]
    name: String,
}

fn main() {
    let args = Args::parse();
    println!("Hello, {}!", args.name);
}
