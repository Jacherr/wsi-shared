use std::time::Duration;
use lazy_static::lazy_static;

#[allow(dead_code)]
pub struct LimitData {
    pub time: Duration,
    pub size: usize,
    pub frames: usize
}

lazy_static! {
    pub static ref LIMITS: Vec<LimitData> = {
        let mut limits: Vec<LimitData> = vec![];
        limits.push(LimitData {
            time: Duration::from_secs(40),
            size: 384,
            frames: 150
        });
        limits.push(LimitData {
            time: Duration::from_secs(60),
            size: 512,
            frames: 200
        });
        limits.push(LimitData {
            time: Duration::from_secs(80),
            size: 1024,
            frames: 225
        });
        limits.push(LimitData {
            time: Duration::from_secs(120),
            size: 2048,
            frames: 250
        });
        limits
    };
}