use image::ImageError as CrateImageError;
use serde::{Deserialize, Serialize};
use std::{string::ToString, num::ParseFloatError, num::ParseIntError};
use zip::result::ZipError;


#[derive(Debug)]
pub enum CmdError {
    IoError(std::io::Error),
    StdError(String),
    Timeout,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub enum ProcessingError {
    CorruptImage(String),
    Timeout,
    ImagePropertyExtractionError,
    Restarting,
    UnsupportedFiletype,
    WorkerDied(Option<String>),
    InputImageError(String),
    ScriptError(String),
    ParameterError(String),
    RequiresPatronTier(usize),
    Other(String),
}
impl std::fmt::Display for ProcessingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
         match self {
            ProcessingError::CorruptImage(e) => write!(f, "The input image is corrupt: {}", e),
            ProcessingError::Timeout => write!(f, "The operation timed out"),
            ProcessingError::ImagePropertyExtractionError => {
                write!(f, "Failed to extract image properties")
            }
            ProcessingError::Restarting => {
                write!(f, "The image service is restarting, please try again in a couple of minutes")
            }
            ProcessingError::UnsupportedFiletype => write!(f, "Unsupported file type"),
            ProcessingError::WorkerDied(e) => {
                write!(f, "The worker handling your request crashed. {}", {
                    if let Some(e) = e {
                        format!("Reason: {}", e)
                    } else {
                        format!("Reason: Unknown")
                    }
                })
            }
            ProcessingError::InputImageError(e) => write!(f, "Invalid image: {}", e),
            ProcessingError::ScriptError(e) => write!(
                f,
                "Internal script error: {}\nThis is a bug. We would appreciate a report.",
                e
            ),
            ProcessingError::ParameterError(e) => write!(f, "Parameter error: {}", e),
            ProcessingError::RequiresPatronTier(tier) => {
                write!(
                    f,
                    "This operation requires you to be a tier {} patron or higher.",
                    tier
                )
            }
            ProcessingError::Other(e) => write!(f, "Unknown error: {}", e),
        }
    }
}
impl From<CmdError> for ProcessingError {
    fn from(input: CmdError) -> ProcessingError {
        match input {
            CmdError::Timeout => ProcessingError::Timeout,
            CmdError::IoError(e) => ProcessingError::Other(e.to_string()),
            CmdError::StdError(e) => ProcessingError::ScriptError(e),
        }
    }
}
impl From<CrateImageError> for ProcessingError {
    fn from(input: CrateImageError) -> ProcessingError {
        match input {
            CrateImageError::Decoding(e) => ProcessingError::CorruptImage(e.to_string()),
            CrateImageError::Encoding(e) => ProcessingError::ScriptError(e.to_string()),
            CrateImageError::Parameter(e) => ProcessingError::ScriptError(e.to_string()),
            CrateImageError::IoError(e) => ProcessingError::ScriptError(e.to_string()),
            CrateImageError::Unsupported(e) => ProcessingError::ScriptError(e.to_string()),
            CrateImageError::Limits(e) => ProcessingError::ScriptError(e.to_string()),
        }
    }
}
impl From<&'static str> for ProcessingError {
    fn from(input: &'static str) -> ProcessingError {
        ProcessingError::ScriptError(input.to_owned())
    }
}
impl From<ZipError> for ProcessingError {
    fn from(input: ZipError) -> ProcessingError {
        ProcessingError::ScriptError(input.to_string())
    }
}
impl From<std::io::Error> for ProcessingError {
    fn from(input: std::io::Error) -> ProcessingError {
        ProcessingError::ScriptError(input.to_string())
    }
}
impl From<reqwest::Error> for ProcessingError {
    fn from(input: reqwest::Error) -> ProcessingError {
        ProcessingError::Other(input.to_string())
    }
}
impl From<ParseIntError> for ProcessingError {
    fn from(input: ParseIntError) -> ProcessingError {
        ProcessingError::ParameterError(input.to_string())
    }
}
impl From<ParseFloatError> for ProcessingError {
    fn from(input: ParseFloatError) -> ProcessingError {
        ProcessingError::ParameterError(input.to_string())
    }
}
impl std::error::Error for ProcessingError {}
