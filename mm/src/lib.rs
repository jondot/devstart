#![allow(clippy::module_name_repetitions)]

use tasks::Task;
pub mod alias;
pub mod clipboard;
pub mod exec;
pub mod prompt;
pub mod shortcuts;
pub mod table;
pub mod tasks;

use snafu::prelude::*;
pub const MMFILE: &str = ".makeme.yaml";

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("cannot serialize"))]
    SerializationYaml { source: serde_yaml::Error },
    #[snafu(display("cannot serialize"))]
    SerializationJson { source: serde_json::Error },
    #[snafu(display("IO/process failure"))]
    IO { source: std::io::Error },
    #[snafu(whatever, display("{message}"))]
    Whatever {
        message: String,
        // https://github.com/shepmaster/snafu/issues/322#issuecomment-1013744387
        #[snafu(source(from(Box<dyn std::error::Error + Send + Sync>, Some)))]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}
pub type Result<T, E = Error> = std::result::Result<T, E>;
