use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("error serializing TOML {0}")]
    TomlSerializationError(#[from] toml::ser::Error),
    #[error("error deserializing TOML {0}")]
    TomlDeserializationError(#[from] toml::de::Error),
    #[error("I/O Error {0}")]
    IOCoreError(#[from] iocore::Error),
    #[error("I/O Error {0}")]
    IOError(#[from] std::io::Error),

}
pub type Result<T> = std::result::Result<T, color_eyre::Report>;
