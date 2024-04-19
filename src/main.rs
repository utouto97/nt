use clap::{Parser, Subcommand};
use rand::distributions::{Alphanumeric, DistString};

mod app;
mod config;
use app::App;
use config::Config;

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

fn generate_id() -> String {
    let mut rng = rand::thread_rng();
    Alphanumeric.sample_string(&mut rng, 32)
}

fn main() {
    let cli = Cli::parse();
    let home_dir = dirs::home_dir().unwrap();
    let dir = home_dir.join("nt");
    let app = App::new(Config::new(dir.to_str().unwrap()));
    app.init().unwrap();
    match cli.command {
        Commands::New { title } => {
            let id = generate_id();
            println!("Creating new note with id: {}, title: {}", id, title);
            app.new_note(&id, &title).unwrap();
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
