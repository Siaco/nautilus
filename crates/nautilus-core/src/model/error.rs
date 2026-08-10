use thiserror::Error;

#[derive(Error, Debug)]
pub enum PipelineParseError {
    #[error("Failed to parse YAML: {0}")]
    YamlError(#[from] serde_yaml::Error),
    #[error("Failed to parse TOML: {0}")]
    TomlError(#[from] toml::de::Error),
    #[error("Validation Error: {0}")]
    ValidationError(String),
}
