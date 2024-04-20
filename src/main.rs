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
    List,
    #[command(about = "edit note")]
    Edit {
        #[arg(help = "note id")]
        id: usize,
    },
}

fn main() {
    let cli = Cli::parse();
    let app = App::new().unwrap();
    match cli.command {
        Commands::New { title } => {
            let _ = app.add_note(&title).unwrap();
        }
        Commands::List => {
            let notes = app.list_notes().unwrap();
            println!("  id: title");
            notes
                .iter()
                .for_each(|note| println!("{:4}: {}", note.id, note.title))
        }
        Commands::Edit { id } => {
            let _ = app.edit_note(id);
        }
    }
}
