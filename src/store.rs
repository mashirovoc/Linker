use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub name: String,
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_with: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum EntryType {
    Path,
    Url,
    File,
    App,
}

impl Entry {
    pub fn target_type(&self) -> EntryType {
        let t = &self.target;
        if t.starts_with("http://") || t.starts_with("https://") || t.starts_with("ftp://") {
            return EntryType::Url;
        }
        // Strip Windows extended-length prefix if present
        #[cfg(windows)]
        let t = &t.strip_prefix(r"\\?\").unwrap_or(t).to_owned();
        let p = std::path::Path::new(t);
        if p.is_dir() {
            EntryType::Path
        } else if p.is_file() {
            #[cfg(windows)]
            {
                let lower = t.to_lowercase();
                if lower.ends_with(".exe") || lower.ends_with(".bat") || lower.ends_with(".cmd") {
                    return EntryType::App;
                }
            }
            EntryType::File
        } else {
            EntryType::Path
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Store {
    pub entries: Vec<Entry>,
}

impl Store {
    pub fn load() -> Self {
        let path = store_path();
        let Ok(content) = fs::read_to_string(&path) else {
            return Self::default();
        };
        serde_json::from_str(&content).unwrap_or_default()
    }

    pub fn save(&self) -> std::io::Result<()> {
        let path = store_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        fs::write(path, content)
    }

    pub fn add(&mut self, name: String, target: String) -> Result<(), String> {
        if self.entries.iter().any(|e| e.name == name) {
            return Err(format!(
                "'{}' is already registered. Use 'l edit {} <new_target>' to update it.",
                name, name
            ));
        }
        let target = resolve_target(target);
        self.entries.push(Entry {
            name,
            target,
            open_with: None,
        });
        Ok(())
    }

    pub fn remove(&mut self, name: &str) -> Result<(), String> {
        let before = self.entries.len();
        self.entries.retain(|e| e.name != name);
        if self.entries.len() == before {
            Err(format!("No entry named '{}'", name))
        } else {
            Ok(())
        }
    }

    pub fn edit(&mut self, name: &str, new_target: String) -> Result<(), String> {
        match self.entries.iter_mut().find(|e| e.name == name) {
            Some(entry) => {
                entry.target = resolve_target(new_target);
                Ok(())
            }
            None => Err(format!("No entry named '{}'", name)),
        }
    }
}

fn store_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("linker")
        .join("bookmarks.json")
}

fn resolve_target(target: String) -> String {
    if target.starts_with("http://")
        || target.starts_with("https://")
        || target.starts_with("ftp://")
    {
        return target;
    }
    let p = std::path::Path::new(&target);
    if p.is_absolute() {
        return target;
    }
    let base = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let joined = base.join(p);
    let resolved = joined.canonicalize().unwrap_or(joined);
    strip_extended_prefix(resolved.to_string_lossy().into_owned())
}

/// On Windows, canonicalize() returns \\?\ prefixed extended paths.
/// Strip the prefix so paths work with Set-Location and other tools.
#[cfg(windows)]
fn strip_extended_prefix(s: String) -> String {
    s.strip_prefix(r"\\?\").unwrap_or(&s).to_owned()
}

#[cfg(not(windows))]
fn strip_extended_prefix(s: String) -> String {
    s
}
