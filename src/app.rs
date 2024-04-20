use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileList {
    pub files: Vec<FileMetadata>,
    pub current_serial_number: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileMetadata {
    pub serial_number: u32,
    pub id: String,
    pub title: String,
}

pub struct App {
    config: Config,
}

impl App {
    pub fn new(config: Config) -> Self {
        App { config }
    }

    pub fn new_note(&self, id: &str, title: &str) -> anyhow::Result<()> {
        let filename = std::path::Path::new(self.config.nt_dir().as_str())
            .join("notes")
            .join(format!("{}.md", id));
        std::fs::write(&filename, title)?;
        self.add_file(id, title)?;
        Ok(())
    }

    pub fn get_filelist(&self) -> anyhow::Result<FileList> {
        let filename = std::path::Path::new(self.config.nt_dir().as_str()).join("filelist.json");
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
        let filename = std::path::Path::new(self.config.nt_dir().as_str()).join("filelist.json");
        std::fs::write(&filename, serde_json::to_string(&metadata)?)?;
        Ok(())
    }

    pub fn get_file(&self, serial_number: u32) -> anyhow::Result<String> {
        let metadata = self.get_filelist()?;
        let file = metadata
            .files
            .iter()
            .find(|file| file.serial_number == serial_number)
            .unwrap();
        let filename = std::path::Path::new(self.config.nt_dir().as_str())
            .join("notes")
            .join(format!("{}.md", file.id));
        Ok(filename.to_str().unwrap().to_string())
    }

    pub fn init(&self) -> anyhow::Result<()> {
        let nt_dir = self.config.nt_dir();
        let nt_dir = std::path::Path::new(nt_dir.as_str());
        if !nt_dir.exists() {
            let metadata = FileList {
                files: vec![],
                current_serial_number: 1,
            };
            let json = serde_json::to_string(&metadata)?;
            std::fs::create_dir_all(nt_dir.join("notes"))?;
            std::fs::write(nt_dir.join("filelist.json"), json)?;
        }
        Ok(())
    }
}
