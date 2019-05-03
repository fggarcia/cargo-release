use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::FatalError;

pub trait ConfigSource {
    fn sign_commit(&self) -> Option<bool> {
        None
    }

    fn upload_doc(&self) -> Option<bool> {
        None
    }

    fn push_remote(&self) -> Option<&str> {
        None
    }

    fn doc_branch(&self) -> Option<&str> {
        None
    }

    fn disable_publish(&self) -> Option<bool> {
        None
    }

    fn disable_push(&self) -> Option<bool> {
        None
    }

    fn dev_version_ext(&self) -> Option<&str> {
        None
    }

    fn no_dev_version(&self) -> Option<bool> {
        None
    }

    fn pre_release_commit_message(&self) -> Option<&str> {
        None
    }

    fn pro_release_commit_message(&self) -> Option<&str> {
        None
    }

    fn pre_release_replacements(&self) -> Option<&[Replace]> {
        None
    }

    fn pre_release_hook(&self) -> Option<&Command> {
        None
    }

    fn tag_message(&self) -> Option<&str> {
        None
    }

    fn tag_prefix(&self) -> Option<&str> {
        None
    }

    fn doc_commit_message(&self) -> Option<&str> {
        None
    }

    fn disable_tag(&self) -> Option<bool> {
        None
    }

    fn enable_features(&self) -> Option<&[String]> {
        None
    }

    fn enable_all_features(&self) -> Option<bool> {
        None
    }

    fn dependent_version(&self) -> Option<DependentVersion> {
        None
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub sign_commit: Option<bool>,
    pub upload_doc: Option<bool>,
    pub push_remote: Option<String>,
    pub doc_branch: Option<String>,
    pub disable_publish: Option<bool>,
    pub disable_push: Option<bool>,
    pub dev_version_ext: Option<String>,
    pub no_dev_version: Option<bool>,
    pub pre_release_commit_message: Option<String>,
    pub pro_release_commit_message: Option<String>,
    pub pre_release_replacements: Option<Vec<Replace>>,
    pub pre_release_hook: Option<Command>,
    pub tag_message: Option<String>,
    pub tag_prefix: Option<String>,
    pub doc_commit_message: Option<String>,
    pub disable_tag: Option<bool>,
    pub enable_features: Option<Vec<String>>,
    pub enable_all_features: Option<bool>,
    pub dependent_version: Option<DependentVersion>,
}

impl Config {
    pub fn sign_commit(&self) -> bool {
        self.sign_commit.unwrap_or(false)
    }

    pub fn upload_doc(&self) -> bool {
        self.upload_doc.unwrap_or(false)
    }

    pub fn push_remote(&self) -> &str {
        self.push_remote
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("origin")
    }

    pub fn doc_branch(&self) -> &str {
        self.doc_branch
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("gh-pages")
    }

    pub fn disable_publish(&self) -> bool {
        self.disable_publish.unwrap_or(false)
    }

    pub fn disable_push(&self) -> bool {
        self.disable_push.unwrap_or(false)
    }

    pub fn dev_version_ext(&self) -> &str {
        self.dev_version_ext
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("alpha.0")
    }

    pub fn no_dev_version(&self) -> bool {
        self.no_dev_version.unwrap_or(false)
    }

    pub fn pre_release_commit_message(&self) -> &str {
        self.pre_release_commit_message
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("(cargo-release) version {{version}}")
    }

    pub fn pro_release_commit_message(&self) -> &str {
        self.pro_release_commit_message
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("(cargo-release) start next development iteration {{version}}")
    }

    pub fn pre_release_replacements(&self) -> &[Replace] {
        self.pre_release_replacements
            .as_ref()
            .map(|v| v.as_ref())
            .unwrap_or(&[])
    }

    pub fn pre_release_hook(&self) -> Option<&Command> {
        self.pre_release_hook.as_ref()
    }

    pub fn tag_message(&self) -> &str {
        self.tag_message
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("(cargo-release) {{crate_name}} version {{version}}")
    }

    pub fn tag_prefix(&self) -> Option<&str> {
        self.tag_prefix.as_ref().map(|s| s.as_str())
    }

    pub fn doc_commit_message(&self) -> &str {
        self.doc_commit_message
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("(cargo-release) generate docs")
    }

    pub fn disable_tag(&self) -> bool {
        self.disable_tag.unwrap_or(false)
    }

    pub fn enable_features(&self) -> &[String] {
        self.enable_features
            .as_ref()
            .map(|v| v.as_ref())
            .unwrap_or(&[])
    }

    pub fn enable_all_features(&self) -> bool {
        self.enable_all_features.unwrap_or(false)
    }

    pub fn dependent_version(&self) -> DependentVersion {
        self.dependent_version.unwrap_or_default()
    }
}

impl ConfigSource for Config {
    fn sign_commit(&self) -> Option<bool> {
        self.sign_commit
    }

    fn upload_doc(&self) -> Option<bool> {
        self.upload_doc
    }

    fn push_remote(&self) -> Option<&str> {
        self.push_remote.as_ref().map(|s| s.as_str())
    }

    fn doc_branch(&self) -> Option<&str> {
        self.doc_branch.as_ref().map(|s| s.as_str())
    }

    fn disable_publish(&self) -> Option<bool> {
        self.disable_publish
    }

    fn disable_push(&self) -> Option<bool> {
        self.disable_push
    }

    fn dev_version_ext(&self) -> Option<&str> {
        self.dev_version_ext.as_ref().map(|s| s.as_str())
    }

    fn no_dev_version(&self) -> Option<bool> {
        self.no_dev_version
    }

    fn pre_release_commit_message(&self) -> Option<&str> {
        self.pre_release_commit_message.as_ref().map(|s| s.as_str())
    }

    fn pro_release_commit_message(&self) -> Option<&str> {
        self.pro_release_commit_message.as_ref().map(|s| s.as_str())
    }

    fn pre_release_replacements(&self) -> Option<&[Replace]> {
        self.pre_release_replacements.as_ref().map(|v| v.as_ref())
    }

    fn pre_release_hook(&self) -> Option<&Command> {
        self.pre_release_hook.as_ref()
    }

    fn tag_message(&self) -> Option<&str> {
        self.tag_message.as_ref().map(|s| s.as_str())
    }

    fn tag_prefix(&self) -> Option<&str> {
        self.tag_prefix.as_ref().map(|s| s.as_str())
    }

    fn doc_commit_message(&self) -> Option<&str> {
        self.doc_commit_message.as_ref().map(|s| s.as_str())
    }

    fn disable_tag(&self) -> Option<bool> {
        self.disable_tag
    }

    fn enable_features(&self) -> Option<&[String]> {
        self.enable_features.as_ref().map(|v| v.as_ref())
    }

    fn enable_all_features(&self) -> Option<bool> {
        self.enable_all_features
    }

    fn dependent_version(&self) -> Option<DependentVersion> {
        self.dependent_version
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Replace {
    pub file: PathBuf,
    pub search: String,
    pub replace: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Command {
    Line(String),
    Args(Vec<String>),
}

impl Command {
    pub fn args(&self) -> Vec<&str> {
        match self {
            Command::Line(ref s) => vec![s.as_str()],
            Command::Args(ref a) => a.iter().map(|s| s.as_str()).collect(),
        }
    }
}

arg_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum DependentVersion {
        Upgrade,
        Fix,
        Error,
        Warn,
        Ignore,
    }
}

impl Default for DependentVersion {
    fn default() -> Self {
        DependentVersion::Fix
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
struct CargoManifest {
    package: Option<CargoPackage>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
struct CargoPackage {
    metadata: Option<CargoMetadata>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
struct CargoMetadata {
    release: Option<Config>,
}

fn load_from_file(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

fn get_config_from_manifest(manifest_path: &Path) -> Result<Option<Config>, FatalError> {
    if manifest_path.exists() {
        let m = load_from_file(manifest_path).map_err(FatalError::from)?;
        let c: CargoManifest = toml::from_str(&m).map_err(FatalError::from)?;
        Ok(c.package.and_then(|p| p.metadata).and_then(|m| m.release))
    } else {
        Ok(None)
    }
}

fn get_config_from_file(file_path: &Path) -> Result<Option<Config>, FatalError> {
    if file_path.exists() {
        let c = load_from_file(file_path).map_err(FatalError::from)?;
        let config = toml::from_str(&c).map_err(FatalError::from)?;
        Ok(Some(config))
    } else {
        Ok(None)
    }
}

pub fn resolve_custom_config(file_path: &Path) -> Result<Option<Config>, FatalError> {
    get_config_from_file(file_path)
}

/// Try to resolve configuration source.
///
/// This tries the following sources in order, short-circuiting on the first one found:
/// 1. $(pwd)/release.toml
/// 2. $(pwd)/Cargo.toml `package.metadata.release` (with deprecation warning)
/// 3. $HOME/.release.toml
///
pub fn resolve_config(manifest_path: &Path) -> Result<Option<Config>, FatalError> {
    // Project release file.
    let default_config = manifest_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("release.toml");
    let current_dir_config = get_config_from_file(&default_config)?;
    if let Some(cfg) = current_dir_config {
        let mut config = Config::default();
        config.update(&cfg);
        return Ok(Some(config));
    };

    // Crate manifest.
    let current_dir_config = get_config_from_manifest(manifest_path)?;
    if let Some(cfg) = current_dir_config {
        let mut config = Config::default();
        config.update(&cfg);
        return Ok(Some(config));
    };

    // User-local configuration from home directory.
    let home_dir = dirs::home_dir();
    if let Some(mut home) = home_dir {
        home.push(".release.toml");
        return resolve_custom_config(home.as_path());
    };

    // No project-wide configuration.
    Ok(None)
}

#[cfg(test)]
mod test {
    use super::*;

    mod resolve_config {
        use super::*;

        #[test]
        fn doesnt_panic() {
            let release_config = resolve_config(Path::new("Cargo.toml")).unwrap().unwrap();
            assert!(release_config.sign_commit());
        }
    }
}
