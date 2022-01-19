use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ImageInfo {
    pub file_size_bytes: usize,
    pub mime_type: String,
    pub dimensions: (u32, u32),
    pub colour_space: String,
    pub frames: Option<usize>,
    pub frame_delays: Option<Vec<usize>>,
    pub repeat: Option<isize>,
    pub comments: Vec<String>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Preprocessed {
    pub image: Vec<u8>,
    pub delays: Vec<usize>,
    pub repeat: i32, // -1 for never
    pub audio: Option<Vec<u8>>,
}