pub struct Config {
    pub nt_dir: String,
}

impl Config {
    pub fn new(nt_dir: &str) -> Self {
        Self {
            nt_dir: nt_dir.to_string(),
        }
    }
}
