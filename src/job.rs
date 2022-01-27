use crate::errors::ProcessingError;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobResult {
    id: usize,
    pub result: Result<Vec<u8>, ProcessingError>
}
impl JobResult {
    pub fn new(id: usize, result: Result<Vec<u8>, ProcessingError>) -> Self {
        Self {
            id,
            result
        }
    }
    pub fn new_ok(id: usize, result: Vec<u8>) -> Self {
        Self {
            id,
            result: Ok(result)
        }
    }
    pub fn new_err(id: usize, result: ProcessingError) -> Self {
        Self {
            id,
            result: Err(result)
        }
    }
    pub fn id(&self) -> usize {
        self.id
    }
}