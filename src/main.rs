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
        #[arg(short = 'l', long = "label", help = "set label")]
        labels: Vec<String>,
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
        Commands::New { title, labels } => {
            let input = app::AddNoteInput::builder()
                .title(title.as_str())
                .labels(labels.iter().map(|label| label.as_str()).collect())
                .build();
            app.add_note(&input).unwrap();
        }
        Commands::List => {
            let notes = app.list_notes().unwrap();
            println!("archived   id: title");
            notes
                .iter()
                .for_each(|note| println!("{:>8} {:4}: {}", "", note.id, note.title))
        }
        Commands::Edit { id } => {
            app.edit_note(id).unwrap();
        }
    }
}
