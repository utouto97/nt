use clap::{Args, Parser, Subcommand};
use rand::distributions::{Alphanumeric, DistString};

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "new note")]
    New(NewArgs),
}

#[derive(Debug, Args)]
struct NewArgs {
    #[arg(help = "note title")]
    title: String,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::New(args) => {
            let id = generate_id();
            println!("Creating new note with id: {}, title: {}", id, args.title);
        }
    }
}

fn generate_id() -> String {
    let mut rng = rand::thread_rng();
    Alphanumeric.sample_string(&mut rng, 32)
}
