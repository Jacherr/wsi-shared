use serde::{Serialize, Deserialize};

use crate::query_params::{AudioQueryParams, BlurQueryParams, CaptionQueryParams, ConstructGifQueryParams, GhostQueryParams, GifSpeedQueryParams, HeartLocketTextQueryParams, ImageMagickEvalQueryParams, MemeQueryParams, MotivateQueryParams, NoneQuery, OverlayQueryParams, PixelateQueryParams, ResizeQueryParams, RotateQueryParams, SetLoopQueryParams};

#[derive(Debug, Clone, Copy)]
pub struct FifoPaths(u32, i32, i32);
impl FifoPaths {
    pub fn new(id: u32) -> Self {
        FifoPaths(id, -1, -1)
    }
    pub fn set_fds(&mut self, fd_tx: i32, fd_rx: i32) {
        self.1 = fd_tx;
        self.2 = fd_rx;
    }
    pub fn fds(&self) -> (i32, i32) {
        (self.1, self.2)
    }
    pub fn incoming(&self) -> String {
        format!("/tmp/wsi-fifo-{}-incoming", self.0)
    }
    pub fn outgoing(&self) -> String {
        format!("/tmp/wsi-fifo-{}-outgoing", self.0)
    }
    pub fn pipes(&self) -> (String, String) {
        (self.outgoing(), self.incoming())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FifoData<T: Serialize> {
    pub image: Vec<u8>,
    pub params: T,
}
impl<T: Serialize> FifoData<T> {
    pub fn new(image: Vec<u8>, params: T) -> Self {
        Self {
            image,
            params
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FifoSend {
    _3dRotate(FifoData<NoneQuery>),
    AnnFrames(FifoData<NoneQuery>),
    AhShit(FifoData<NoneQuery>),
    Audio(FifoData<AudioQueryParams>),
    AprilFools(FifoData<NoneQuery>),
    Blur(FifoData<BlurQueryParams>),
    Caption(FifoData<CaptionQueryParams>),
    ConvertPng(FifoData<NoneQuery>),
    ConstructGif(FifoData<ConstructGifQueryParams>),
    FixTransparency(FifoData<NoneQuery>),
    Flash(FifoData<NoneQuery>),
    Flip(FifoData<NoneQuery>),
    Flop(FifoData<NoneQuery>),
    Frames(FifoData<NoneQuery>),
    Ghost(FifoData<GhostQueryParams>),
    GifLoop(FifoData<NoneQuery>),
    GifMagik(FifoData<NoneQuery>),
    GifScramble(FifoData<NoneQuery>),
    GifSpeed(FifoData<GifSpeedQueryParams>),
    Grayscale(FifoData<NoneQuery>),
    HeartLocketText(FifoData<HeartLocketTextQueryParams>),
    ImageInfo(FifoData<NoneQuery>),
    ImageMagickEval(FifoData<ImageMagickEvalQueryParams>),
    Invert(FifoData<NoneQuery>),
    Jpeg(FifoData<NoneQuery>),
    Magik(FifoData<NoneQuery>),
    Meme(FifoData<MemeQueryParams>),
    Motivate(FifoData<MotivateQueryParams>),
    Overlay(FifoData<OverlayQueryParams>),
    Pixelate(FifoData<PixelateQueryParams>),
    Preprocess(FifoData<NoneQuery>),
    Printer(FifoData<NoneQuery>),
    Rainbow(FifoData<NoneQuery>),
    Resize(FifoData<ResizeQueryParams>),
    Reverse(FifoData<NoneQuery>),
    Rotate(FifoData<RotateQueryParams>),
    SetLoop(FifoData<SetLoopQueryParams>),
    Spin(FifoData<NoneQuery>),
    Spread(FifoData<NoneQuery>),
    Swirl(FifoData<NoneQuery>),
    Tehi(FifoData<NoneQuery>),
    Wall(FifoData<NoneQuery>),
    Wave(FifoData<NoneQuery>),
    Wormhole(FifoData<NoneQuery>),
    Zoom(FifoData<NoneQuery>)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WsiRequest {
    pub id: usize,
    pub premium_level: usize,
    pub data: FifoSend
}
impl WsiRequest {
    pub fn new(id: usize, premium_level: usize, data: FifoSend) -> Self {
        Self {
            id,
            premium_level,
            data
        }
    }
}