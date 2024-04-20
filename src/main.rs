use clap::{Parser, Subcommand};

mod app;
mod config;
use app::App;

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
    List {
        #[arg(short, long, help = "Include archived notes")]
        archived: bool,
    },
    #[command(about = "edit note")]
    Edit {
        #[arg(help = "note id")]
        id: usize,
    },
    #[command(about = "archive note")]
    Archive {
        #[arg(help = "note id")]
        id: usize,
    },
}

fn main() {
    let cli = Cli::parse();
    let app = App::new().unwrap();
    match cli.command {
        Commands::New { title } => {
            app.add_note(&title).unwrap();
        }
        Commands::List { archived } => {
            let notes = app.list_notes(archived).unwrap();
            println!("archived   id: title");
            notes.iter().for_each(|note| {
                println!(
                    "{:>8} {:4}: {}",
                    if note.archived { "o" } else { "" },
                    note.id,
                    note.title
                )
            })
        }
        Commands::Edit { id } => {
            app.edit_note(id).unwrap();
        }
        Commands::Archive { id } => {
            app.archive_note(id).unwrap();
        }
    }
}
