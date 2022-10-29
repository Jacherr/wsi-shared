use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug)]
pub struct AudioQueryParams {
    pub effect: String,
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
pub struct GhostQueryParams {
    pub depth: Option<usize>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct GifSpeedQueryParams {
    pub delay: Option<f64>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct HeartLocketTextQueryParams {
    pub text: String,
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
pub struct PreprocessQueryParams {
    pub frame0: bool
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
impl ResizeMethod {
    pub fn from_str(s: &str) -> Option<ResizeMethod> {
        match s {
            "nearest" => Some(ResizeMethod::Nearest),
            "gaussian" => Some(ResizeMethod::Gaussian),
            _ => None,
        }
    }
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
#[derive(Deserialize, Serialize, Debug)]
pub struct NeonQueryParams {
    pub radius: usize,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct BloomQueryParams {
    pub radius: usize,
    pub brightness: usize,
    pub sharpness: usize,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct ZoomBlurQueryParams {
    pub factor: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MakesweetQueryParams {
    pub template: String,
    pub images: Vec<Vec<u8>>,
}
