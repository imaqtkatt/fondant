#[derive(serde::Deserialize, Debug)]
pub struct JavaConfig {
    project: Project,
    pub java: Java,
}

#[derive(serde::Deserialize, Debug)]
struct Project {
    pub name: String,
    pub artifact: Option<String>,
    pub group_id: Option<String>,
    pub version: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct Java {
    #[serde(default = "default_source")]
    pub source: String,
    #[serde(default = "default_target")]
    pub target: String,
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
        std::path::Path::new(&self.java.source)
    }
    pub fn output_path(&self) -> &std::path::Path {
        std::path::Path::new(&self.java.target)
    }
    pub fn cache_path(&self) -> std::path::PathBuf {
        self.output_path().join("fondant-cache.toml")
    }
    pub fn jar_name(&self) -> String {
        let artifact = self.project.artifact.as_ref().unwrap_or(&self.project.name);
        let version = &self.project.version;
        format!("{artifact}-{version}.jar")
    }
}

fn default_source() -> String {
    "src/main/java".to_string()
}
fn default_target() -> String {
    "target/classes".to_string()
}
fn default_main_class() -> String {
    "Main".to_string()
}
