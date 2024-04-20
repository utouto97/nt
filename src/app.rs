use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::config::Config;

pub struct App {
    config: Config,
}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        let config = Config::load()?;
        Ok(Self { config })
    }

    pub fn add_note(&self, title: &str) -> anyhow::Result<Note> {
        let mut state = State::load(self.config.nt_dir().as_str());
        let id = state.next_id;
        let note = Note {
            id,
            title: title.to_string(),
            path: format!("{}.md", id),
            archived: false,
        };
        state.next_id += 1;
        state.notes.push(note.clone());
        state.save(self.config.nt_dir().as_str())?;
        let filepath = std::path::Path::new(self.config.nt_dir().as_str())
            .join("notes")
            .join(&note.path);
        std::fs::write(&filepath, title)?;
        std::process::Command::new("nvim").arg(&filepath).status()?;
        Ok(note)
    }

    pub fn list_notes(&self, archived: bool) -> anyhow::Result<Vec<Note>> {
        let state = State::load(self.config.nt_dir().as_str());
        let notes = if archived {
            state.notes
        } else {
            state
                .notes
                .into_iter()
                .filter(|note| !note.archived)
                .collect()
        };
        Ok(notes)
    }

    pub fn edit_note(&self, id: usize) -> anyhow::Result<()> {
        let state = State::load(self.config.nt_dir().as_str());
        let note = state
            .notes
            .iter()
            .find(|note| note.id == id)
            .ok_or(anyhow!("note not found"))?;
        let filepath = std::path::Path::new(self.config.nt_dir().as_str())
            .join("notes")
            .join(&note.path);
        std::process::Command::new("nvim").arg(filepath).status()?;
        Ok(())
    }

    pub fn archive_note(&self, id: usize) -> anyhow::Result<()> {
        let mut state = State::load(self.config.nt_dir().as_str());
        state
            .notes
            .iter_mut()
            .find(|note| note.id == id)
            .map(|note| note.archived = true)
            .ok_or(anyhow!("note not found"))?;
        state.save(self.config.nt_dir().as_str())?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct State {
    pub next_id: usize,
    pub notes: Vec<Note>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Note {
    pub id: usize,
    pub path: String,
    pub title: String,
    #[serde(default)]
    pub archived: bool,
}

impl State {
    pub fn load(nt_dir: &str) -> State {
        let filepath = std::path::Path::new(nt_dir).join("nt_state.json");
        let content = match std::fs::read_to_string(&filepath) {
            Ok(content) => content,
            Err(_) => return State::default(),
        };
        let state: State = serde_json::from_str(content.as_str()).unwrap_or_default();
        state
    }

    pub fn save(&self, nt_dir: &str) -> anyhow::Result<()> {
        let filepath = std::path::Path::new(nt_dir).join("nt_state.json");
        std::fs::write(filepath, serde_json::to_string(&self)?)?;
        Ok(())
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            next_id: 1,
            notes: Vec::new(),
        }
    }
}
