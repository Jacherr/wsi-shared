use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug)]
pub struct AnnmarieBody {
    pub route: String,
    pub query_params: Vec<(String, String)>,
    pub images: Vec<Vec<u8>>
}
#[derive(Deserialize, Serialize, Debug)]
pub struct AudioQueryParams {
    pub effect: String
}
#[derive(Deserialize, Serialize, Debug)]
pub struct BlurQueryParams {
    pub power: f32,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct CaptionQueryParams {
    pub text: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct CompressQueryParams {
    pub level: Option<usize>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct ConstructGifQueryParams {
    pub delays: Vec<usize>,
    pub repeat: i32, // -1 for never
    pub audio: Option<Vec<u8>>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct GhostQueryParams {
    pub depth: Option<usize>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct GifSpeedQueryParams {
    pub delay: Option<usize>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct HeartLocketTextQueryParams {
    pub text: String
}
#[derive(Deserialize, Serialize, Debug)]
pub struct ImageMagickEvalQueryParams {
    pub script: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct MemeQueryParams {
    pub top: String,
    pub bottom: Option<String>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct MotivateQueryParams {
    pub top: String,
    pub bottom: Option<String>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct OverlayQueryParams {
    pub overlay: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct PixelateQueryParams {
    pub downscaled_height: Option<usize>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct ResizeQueryParams {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub scale: Option<f32>,
    pub method: Option<ResizeMethod>,
}
#[derive(Deserialize, Serialize, Debug)]
pub enum ResizeMethod {
    Nearest,
    Gaussian,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct RotateQueryParams {
    pub degrees: usize,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct SetLoopQueryParams {
    pub r#loop: bool,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct NoneQuery {}
