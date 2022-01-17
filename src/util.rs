pub fn extract_frames_from_encoded(result: Vec<u8>) -> Vec<Vec<u8>> {
    let mut head = 6;
    let mut images: Vec<Vec<u8>> = Vec::new();

    let image_count = u16::from_le_bytes([result[4], result[5]]);
    for _ in 0..image_count {
        let image_length = u32::from_le_bytes([result[head], result[head + 1], result[head + 2], result[head + 3]]) as usize;
        head += 4;
        images.push(result[head..head + image_length].to_vec());
        head += image_length;
    }

    images
}

pub fn encode_frames(images: Vec<Vec<u8>>) -> Vec<u8> {
    let mut ann_buffer = "ASS0".as_bytes().to_vec();
    ann_buffer.extend_from_slice(&u16::to_le_bytes(images.len() as u16));

    for img in images {
        ann_buffer = [
            ann_buffer,
            u32::to_le_bytes(img.len() as u32).to_vec(),
            img
        ]
        .concat();
    };

    ann_buffer
}