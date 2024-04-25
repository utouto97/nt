use anyhow::anyhow;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use shellexpand;

#[derive(Debug, Serialize, Deserialize, Getters)]
#[serde(default)]
pub struct Config {
    #[getter(skip)]
    nt_dir: String,
    editor: String,
    #[getter(skip)]
    default_label: String,
    default_filter: String,
}

const NT_CONFIG: &str = "~/.nt.json";

impl Config {
    pub fn nt_dir(&self) -> String {
        format!("{}", shellexpand::tilde(self.nt_dir.as_str()))
    }

    pub fn default_label(&self) -> Vec<String> {
        self.default_label
            .split(" ")
            .map(|x| x.to_string())
            .filter(|x| x.len() > 0)
            .collect()
    }

    pub fn load() -> anyhow::Result<Self> {
        let config_path = format!("{}", shellexpand::tilde(NT_CONFIG));
        let config = match std::fs::read_to_string(&config_path) {
            Ok(content) => {
                let config: Config = serde_json::from_str(&content)?;
                Ok(config)
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    Ok(Config::default())
                } else {
                    Err(e)
                }
            }
        }?;
        Ok(config)
    }

    pub fn get(&self, key: &str) -> anyhow::Result<String> {
        let json = serde_json::to_value(self)?;
        json.get(key)
            .ok_or(anyhow!("key not found"))
            .and_then(|v| Ok(v.to_string()))
    }

    pub fn set(&self, key: &str, value: Option<&str>) -> anyhow::Result<()> {
        let value = match value {
            Some(value) => serde_json::Value::from(value),
            None => {
                let default_json = serde_json::to_value(self)?;
                let default_value = default_json.get(key).ok_or(anyhow!("key not found"))?;
                default_value.clone()
            }
        };

        let mut json = serde_json::to_value(self)?;
        if json.get(key).is_some() {
            json[key] = value;
            let config_path = format!("{}", shellexpand::tilde(NT_CONFIG));
            std::fs::write(&config_path, json.to_string())?;
            Ok(())
        } else {
            Err(anyhow!("key not found"))
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            nt_dir: "~/nt".to_string(),
            editor: "vim".to_string(),
            default_label: "".to_string(),
            default_filter: "not:archived".to_string(),
        }
    }
}
