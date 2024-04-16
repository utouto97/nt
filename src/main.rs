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

struct Config {
    nt_dir: String,
}

struct App {
    config: Config,
}

fn generate_id() -> String {
    let mut rng = rand::thread_rng();
    Alphanumeric.sample_string(&mut rng, 32)
}

impl App {
    fn new(config: Config) -> Self {
        App { config }
    }

    fn new_note(&self, id: &str, title: &str) -> anyhow::Result<()> {
        let filename = std::path::Path::new(self.config.nt_dir.as_str())
            .join("notes")
            .join(format!("{}.md", id));
        std::fs::write(&filename, title)?;
        Ok(())
    }
}

fn main() {
    let cli = Cli::parse();
    let app = App::new(Config {
        nt_dir: String::from("./nt"),
    });
    match cli.command {
        Commands::New(args) => {
            let id = generate_id();
            println!("Creating new note with id: {}, title: {}", id, args.title);
            app.new_note(&id, &args.title).unwrap();
        }
    }
}
