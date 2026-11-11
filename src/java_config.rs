#[derive(serde::Deserialize, Debug)]
pub struct JavaConfig {
    #[serde(default = "default_source")]
    pub source: String,
    #[serde(default = "default_output")]
    pub output: String,
    #[serde(default = "default_main_class")]
    pub main_class: String,

    pub jars: Vec<String>,
}

const FONDANT_TOML_PATH: &str = if cfg!(debug_assertions) {
    "./Fondant.example.toml"
} else {
    "./Fondant.toml"
};

pub fn get_java_config() -> std::io::Result<JavaConfig> {
    let contents = std::fs::read_to_string(FONDANT_TOML_PATH)?;
    toml::from_str(&contents).map_err(std::io::Error::other)
}

impl JavaConfig {
    pub fn source_path(&self) -> &std::path::Path {
        std::path::Path::new(&self.source)
    }
    pub fn output_path(&self) -> &std::path::Path {
        std::path::Path::new(&self.output)
    }
    pub fn cache_path(&self) -> std::path::PathBuf {
        self.output_path().join("fondant-cache.json")
    }
}

fn default_source() -> String {
    "src/main/java".to_string()
}
fn default_output() -> String {
    "output/classes".to_string()
}
fn default_main_class() -> String {
    "Main".to_string()
}
