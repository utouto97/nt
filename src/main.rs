use clap::{Parser, Subcommand};
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
    New {
        #[arg(help = "note title")]
        title: String,
    },
    #[command(about = "list notes")]
    List,
    #[command(about = "edit note")]
    Edit {
        #[arg(help = "note id")]
        id: u32,
    },
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
    current_serial_number: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct FileMetadata {
    serial_number: u32,
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
            Err(_) => FileList {
                files: vec![],
                current_serial_number: 0,
            },
        };
        Ok(metadata)
    }

    fn add_file(&self, id: &str, title: &str) -> anyhow::Result<()> {
        let mut metadata = self.get_filelist()?;
        metadata.files.push(FileMetadata {
            serial_number: metadata.current_serial_number,
            id: String::from(id),
            title: String::from(title),
        });
        metadata.current_serial_number += 1;
        let filename = std::path::Path::new(self.config.nt_dir.as_str()).join("filelist.json");
        std::fs::write(&filename, serde_json::to_string(&metadata)?)?;
        Ok(())
    }

    fn get_file(&self, serial_number: u32) -> anyhow::Result<String> {
        let metadata = self.get_filelist()?;
        let file = metadata
            .files
            .iter()
            .find(|file| file.serial_number == serial_number)
            .unwrap();
        let filename = std::path::Path::new(self.config.nt_dir.as_str())
            .join("notes")
            .join(format!("{}.md", file.id));
        Ok(filename.to_str().unwrap().to_string())
    }
}

fn main() {
    let cli = Cli::parse();
    let app = App::new(Config {
        nt_dir: String::from("./nt"),
    });
    match cli.command {
        Commands::New { title } => {
            let id = generate_id();
            println!("Creating new note with id: {}, title: {}", id, title);
            app.new_note(&id, &title).unwrap();
            let metadata = app.get_filelist().unwrap();
            println!("Metadata: {:?}", metadata);
        }
        Commands::List => {
            let metadata = app.get_filelist().unwrap();
            metadata.files.iter().for_each(|file| {
                println!("{}: {}, {}", file.serial_number, file.title, file.id);
            });
        }
        Commands::Edit { id } => {
            let filename = app.get_file(id).unwrap();
            println!("Editing note with id: {} {}", id, filename);
            std::process::Command::new("nvim")
                .arg(filename)
                .status()
                .unwrap();
        }
    }
}
