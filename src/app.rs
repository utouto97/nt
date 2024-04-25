use std::collections::HashSet;

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
        let path = std::path::Path::new(config.nt_dir().as_str()).join("notes");
        if !path.exists() {
            std::fs::create_dir_all(path)?
        }
        Ok(Self { config })
    }

    pub fn add_note<'a>(&'a self, input: &AddNoteInput<'a>) -> anyhow::Result<Note> {
        let mut state = State::load(self.config.nt_dir().as_str());
        let id = state.next_id;
        let mut labels: HashSet<String> =
            input.labels.iter().map(|label| label.to_string()).collect();
        labels.extend(self.config.default_label().clone());
        let note = Note {
            id,
            path: format!("{}.md", id),
            title: input.title.to_string(),
            labels,
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
        let mut default_filters: Vec<Filter> = self
            .config
            .default_filter()
            .split(" ")
            .map(|f| f.try_into().unwrap())
            .collect();
        default_filters.extend(filters);
        let state = State::load(self.config.nt_dir().as_str());
        let filtered: Vec<Note> = state
            .notes
            .into_iter()
            .filter(|note| {
                let mut ok = true;
                for filter in default_filters.iter() {
                    match filter {
                        Filter::Is(label) => {
                            if !note.labels.contains(*label) {
                                ok = false
                            }
                        }
                        Filter::Not(label) => {
                            if note.labels.contains(*label) {
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
        std::process::Command::new(self.config.editor())
            .arg(filepath)
            .status()?;
        Ok(())
    }

    pub fn add_labels<'a>(&'a self, id: usize, labels: Vec<&'a str>) -> anyhow::Result<()> {
        let mut state = State::load(self.config.nt_dir().as_str());
        let note = state
            .notes
            .iter_mut()
            .find(|note| note.id == id)
            .ok_or(anyhow!("note not found"))?;
        for label in labels {
            note.labels.insert(label.to_string());
        }
        state.save(self.config.nt_dir().as_str())?;
        Ok(())
    }

    pub fn remove_labels<'a>(&'a self, id: usize, labels: Vec<&'a str>) -> anyhow::Result<()> {
        let mut state = State::load(self.config.nt_dir().as_str());
        let note = state
            .notes
            .iter_mut()
            .find(|note| note.id == id)
            .ok_or(anyhow!("note not found"))?;
        for label in labels {
            note.labels.remove(label);
        }
        state.save(self.config.nt_dir().as_str())?;
        Ok(())
    }

    pub fn search_notes(&self, keyword: &str, filters: Vec<Filter>) -> anyhow::Result<Vec<Note>> {
        let mut default_filters: Vec<Filter> = self
            .config
            .default_filter()
            .split(" ")
            .map(|f| f.try_into().unwrap())
            .collect();
        default_filters.extend(filters);
        let state = State::load(self.config.nt_dir().as_str());
        let filtered: Vec<Note> = state
            .notes
            .into_iter()
            .filter(|note| {
                let mut ok = true;
                for filter in default_filters.iter() {
                    match filter {
                        Filter::Is(label) => {
                            if !note.labels.contains(*label) {
                                ok = false
                            }
                        }
                        Filter::Not(label) => {
                            if note.labels.contains(*label) {
                                ok = false
                            }
                        }
                    }
                }

                let filepath = std::path::Path::new(self.config.nt_dir().as_str())
                    .join("notes")
                    .join(&note.path);
                if !(grep(filepath.to_str().unwrap_or(""), keyword).unwrap_or(false)) {
                    ok = false
                }

                ok
            })
            .collect();
        Ok(filtered)
    }

    pub fn get_config(&self, key: &str) -> anyhow::Result<String> {
        self.config.get(key)
    }

    pub fn set_config(&self, key: &str, value: Option<&str>) -> anyhow::Result<()> {
        self.config.set(key, value)
    }
}

fn grep(file_path: &str, keyword: &str) -> anyhow::Result<bool> {
    if file_path.len() == 0 {
        return Ok(false);
    }
    let contents = std::fs::read_to_string(file_path)?;
    Ok(contents.contains(keyword))
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
    pub labels: HashSet<String>,
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
