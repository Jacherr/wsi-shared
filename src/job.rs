use crate::errors::ProcessingError;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum JobResult {
    Image((usize, Vec<u8>)),
    Error((usize, ProcessingError))
}
impl JobResult {
    pub fn id(&self) -> usize {
        match self {
            JobResult::Image(id) => id.0,
            JobResult::Error(id) => id.0
        }
    }
}