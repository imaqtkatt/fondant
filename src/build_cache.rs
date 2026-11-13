use std::io::Read;

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct BuildCache {
    sources: std::collections::BTreeMap<String, blake3::Hash>,
    main: String,
}

impl BuildCache {
    pub fn new(sources: std::collections::BTreeMap<String, blake3::Hash>, main: String) -> Self {
        Self { sources, main }
    }

    pub fn load(path: &std::path::Path) -> std::io::Result<Option<Self>> {
        if !path.exists() {
            return Ok(None);
        }
        let contents = std::fs::read_to_string(path)?;
        let cache = toml::from_str(&contents).map_err(std::io::Error::other)?;
        Ok(Some(cache))
    }

    pub fn save(&self, path: &std::path::Path) -> std::io::Result<()> {
        use std::io::Write;

        let mut file = std::fs::File::options()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path)?;

        let toml_string = toml::to_string(self).map_err(std::io::Error::other)?;
        file.write_all(toml_string.as_bytes())?;
        Ok(())
    }
}

pub fn fingerprint_files(
    files: &[std::path::PathBuf],
) -> std::io::Result<std::collections::BTreeMap<String, blake3::Hash>> {
    let mut fingerprints = std::collections::BTreeMap::<String, blake3::Hash>::new();

    for path in files {
        let h = hash_file(path)?;
        fingerprints.insert(path.to_string_lossy().into_owned(), h);
    }

    Ok(fingerprints)
}

fn hash_file(path: &std::path::Path) -> std::io::Result<blake3::Hash> {
    let mut file = std::fs::File::open(path)?;
    let mut hasher = blake3::Hasher::new();

    let mut buf = [0u8; 8192];

    loop {
        let read = file.read(&mut buf)?;

        if read == 0 {
            break;
        }

        hasher.update(&buf[..read]);
    }

    Ok(hasher.finalize())
}
