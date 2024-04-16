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
struct FileList {
    files: Vec<FileMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FileMetadata {
    id: String,
    title: String,
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
        self.add_file(id, title)?;
        Ok(())
    }

    fn get_filelist(&self) -> anyhow::Result<FileList> {
        let filename = std::path::Path::new(self.config.nt_dir.as_str()).join("filelist.json");
        let metadata = match std::fs::read_to_string(&filename) {
            Ok(metadata) => serde_json::from_str(&metadata)?,
            Err(_) => FileList { files: vec![] },
        };
        Ok(metadata)
    }

    fn add_file(&self, id: &str, title: &str) -> anyhow::Result<()> {
        let mut metadata = self.get_filelist()?;
        metadata.files.push(FileMetadata {
            id: String::from(id),
            title: String::from(title),
        });
        let filename = std::path::Path::new(self.config.nt_dir.as_str()).join("filelist.json");
        std::fs::write(&filename, serde_json::to_string(&metadata)?)?;
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
            let metadata = app.get_filelist().unwrap();
            println!("Metadata: {:?}", metadata);
        }
    }
}
