use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use shellexpand;

#[derive(Debug, Serialize, Deserialize, Getters)]
#[serde(default)]
pub struct Config {
    #[getter(skip)]
    nt_dir: String,
}

const NT_CONFIG: &str = "~/.nt.json";

impl Config {
    pub fn nt_dir(&self) -> String {
        format!("{}", shellexpand::tilde(self.nt_dir.as_str()))
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
}

impl Default for Config {
    fn default() -> Self {
        Self {
            nt_dir: "~/nt".to_string(),
        }
    }
}
