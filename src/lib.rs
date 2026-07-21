#![allow(unused)]
#[macro_use]
extern crate pest_derive;

pub(crate) mod prettify;
pub use prettify::prettify_file;

pub mod errors;
pub use errors::{Error, Result};

pub(crate) mod parser;
pub use parser::{parse_source, toml::TomlParser};

pub(crate) mod source;
pub use source::{Source, Span, SpanPosition};

pub use pest_grammars::toml::Rule;
