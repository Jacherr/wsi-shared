use crate::errors::ProcessingError;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum JobResult {
    Image((usize, Vec<u8>)),
    Error((usize, ProcessingError))
}