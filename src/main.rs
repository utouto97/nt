use clap::{Args, Parser, Subcommand};
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize)]
struct Metadata {
    files: Vec<String>,
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

    fn get_metadata(&self) -> anyhow::Result<Metadata> {
        let filename = std::path::Path::new(self.config.nt_dir.as_str()).join("metadata.json");
        let metadata = match std::fs::read_to_string(&filename) {
            Ok(metadata) => serde_json::from_str(&metadata)?,
            Err(_) => Metadata { files: vec![] },
        };
        Ok(metadata)
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
            let metadata = app.get_metadata().unwrap();
            println!("Metadata: {:?}", metadata);
        }
    }
}
