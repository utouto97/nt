use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::config::Config;

pub struct App {
    config: Config,
}

#[derive(Debug, TypedBuilder)]
pub struct AddNoteInput<'a> {
    title: &'a str,
    #[builder(default = Vec::new())]
    labels: Vec<&'a str>,
}

#[derive(Debug)]
pub enum Filter<'a> {
    Is(&'a str),
    Not(&'a str),
}

impl<'a> TryFrom<&'a str> for Filter<'a> {
    type Error = anyhow::Error;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let trimmed = value.trim();
        if trimmed.starts_with("is:") {
            Ok(Filter::Is(&trimmed[3..].trim()))
        } else if trimmed.starts_with("not:") {
            Ok(Filter::Not(&trimmed[4..].trim()))
        } else {
            Err(anyhow!("filter parse error"))
        }
    }
}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        let config = Config::load()?;
        Ok(Self { config })
    }

    pub fn add_note<'a>(&'a self, input: &AddNoteInput<'a>) -> anyhow::Result<Note> {
        let mut state = State::load(self.config.nt_dir().as_str());
        let id = state.next_id;
        let note = Note {
            id,
            path: format!("{}.md", id),
            title: input.title.to_string(),
            labels: input.labels.iter().map(|label| label.to_string()).collect(),
        };
        state.next_id += 1;
        state.notes.push(note.clone());
        state.save(self.config.nt_dir().as_str())?;
        let filepath = std::path::Path::new(self.config.nt_dir().as_str())
            .join("notes")
            .join(&note.path);
        std::fs::write(&filepath, input.title)?;
        std::process::Command::new("nvim").arg(&filepath).status()?;
        Ok(note)
    }

    pub fn list_notes(&self, filters: Vec<Filter>) -> anyhow::Result<Vec<Note>> {
        let state = State::load(self.config.nt_dir().as_str());
        let filtered: Vec<Note> = state
            .notes
            .into_iter()
            .filter(|note| {
                let mut ok = true;
                for filter in filters.iter() {
                    match filter {
                        Filter::Is(label) => {
                            if !note.labels.iter().any(|l| l.as_str() == *label) {
                                ok = false
                            }
                        }
                        Filter::Not(label) => {
                            if !note.labels.iter().all(|l| l.as_str() != *label) {
                                ok = false
                            }
                        }
                    }
                }
                ok
            })
            .collect();
        Ok(filtered)
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
    pub labels: Vec<String>,
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
