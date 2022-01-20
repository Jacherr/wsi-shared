use std::string::ToString;
use serde::{Serialize, Deserialize};
use image::ImageError as CrateImageError;
use zip::result::ZipError;

#[cfg(target_os = "linux")]
use magick_rust::MagickError;

#[derive(Debug)]
pub enum CmdError {
    IoError(std::io::Error),
    StdError(String),
    Timeout,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub enum ProcessingError {
    CorruptImage,
    Timeout,
    ImagePropertyExtractionError,
    Restarting,
    UnsupportedFiletype,
    WorkerDied,
    InputImageError(String),
    ScriptError(String),
    ParameterError(String),
    Other(String)
}
impl ToString for ProcessingError {
    fn to_string(&self) -> String {
        match self {
            ProcessingError::CorruptImage => format!("The input image is corrupt"),
            ProcessingError::Timeout => format!("The operation timed out"),
            ProcessingError::ImagePropertyExtractionError => format!("Failed to extract image properties"),
            ProcessingError::Restarting => format!("The image service is restarting, please try again in a couple of minutes"),
            ProcessingError::UnsupportedFiletype => format!("Unsupported file type"),
            ProcessingError::WorkerDied => format!("The worker handling your request crashed. Check the dimensions of your image. Otherwise, this is probably a bug and we would appreciate a report."),
            ProcessingError::InputImageError(e) => format!("Invalid image: {}", e),
            ProcessingError::ScriptError(e) => format!("Internal script error: {}\nThis is a bug. We would appreciate a report.", e),
            ProcessingError::ParameterError(e) => format!("Parameter error: {}", e),
            ProcessingError::Other(e) => format!("Unknown error: {}", e)
        }
    }
}
impl From<CmdError> for ProcessingError {
    fn from(input: CmdError) -> ProcessingError {
        match input {
            CmdError::Timeout => ProcessingError::Timeout,
            CmdError::IoError(e) => ProcessingError::Other(e.to_string()),
            CmdError::StdError(e) => ProcessingError::ScriptError(e)
        }
    }
}
impl From<CrateImageError> for ProcessingError {
    fn from(input: CrateImageError) -> ProcessingError {
        match input {
            CrateImageError::Decoding(_) => ProcessingError::CorruptImage,
            CrateImageError::Encoding(e) => ProcessingError::ScriptError(e.to_string()),
            CrateImageError::Parameter(e) => ProcessingError::ScriptError(e.to_string()),
            CrateImageError::IoError(e) => ProcessingError::ScriptError(e.to_string()),
            CrateImageError::Unsupported(e) => ProcessingError::ScriptError(e.to_string()),
            CrateImageError::Limits(e) => ProcessingError::ScriptError(e.to_string())
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
#[cfg(target_os = "linux")]
impl From<MagickError> for ProcessingError {
    fn from(input: MagickError) -> ProcessingError {
        ProcessingError::ScriptError(input.to_string())
    }
}