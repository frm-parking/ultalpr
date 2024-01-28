use std::env;
use std::ops::Deref;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use serde::Deserialize;
use tokio::fs;
use url::Url;
use engine::Config as EngineConfig;

#[derive(Debug, Clone)]
pub struct ConfigPath(PathBuf);

impl Default for ConfigPath {
	fn default() -> Self {
		env::var("CONFIG").unwrap_or("config.toml".into()).into()
	}
}

impl AsRef<Path> for ConfigPath {
	fn as_ref(&self) -> &Path {
		&self.0
	}
}

impl Deref for ConfigPath {
	type Target = Path;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl From<PathBuf> for ConfigPath {
	fn from(value: PathBuf) -> Self {
		Self(value)
	}
}

impl From<&Path> for ConfigPath {
	fn from(value: &Path) -> Self {
		Self(value.to_owned())
	}
}

impl From<&str> for ConfigPath {
	fn from(value: &str) -> Self {
		Self(value.into())
	}
}

impl From<String> for ConfigPath {
	fn from(value: String) -> Self {
		Self(value.into())
	}
}

#[derive(Debug, Deserialize)]
pub struct ServiceConfig {
	pub host: String,
	pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct SourceConfig {
	pub id: String,
	pub uri: Url,
	pub user: Option<String>,
	pub pwd: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
	pub service: ServiceConfig,
	pub source: Vec<SourceConfig>,
	pub alpr: EngineConfig,
}

impl Config {
	pub async fn read_from_file<P>(path: P) -> Result<Arc<Self>>
	where
		P: AsRef<Path>,
	{
		let content = fs::read_to_string(path).await?;
		Ok(Arc::new(toml::from_str(&content)?))
	}
}
