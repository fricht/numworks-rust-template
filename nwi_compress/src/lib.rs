use std::{fs::File, io::Write};

use image;

/// Compress an image into a nwi image
///
/// in development
/// currently not using the compression (lz4)
///
/// # Parameters
/// - `in_path` is the path to the input image (must be 55*56)
/// - `out_path` is the path where the output will be written
///
/// # Return
/// A `Result` containing the error message if any (otherwise an empty tuple)
pub fn compress_icon(in_path: &str, out_path: &str) -> Result<(), String> {
    // read image data
    let img = match image::open(in_path) {
        Ok(i) => i,
        Err(e) => return Err(e.to_string()),
    };
    let rgb_img = img.to_rgb8();
    // convert to rgb565
    let mut raw_data: Vec<u8> = vec![
        0xF0, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x19,
    ];
    for col in rgb_img.pixels() {
        let r5 = col.0[0] as u16 >> 3;
        let g6 = col.0[1] as u16 >> 2;
        let b5 = col.0[2] as u16 >> 3;
        let rgb565 = ((r5 << 11) | (g6 << 5) | b5).to_le_bytes();
        raw_data.push(rgb565[0]);
        raw_data.push(rgb565[1]);
    }
    // check vec length
    if raw_data.len() != 6186 {
        return Err("The provided image might not be 55*56".to_string());
    }
    // write file
    let mut file = match File::create(out_path) {
        Ok(f) => f,
        Err(e) => return Err(e.to_string()),
    };
    match file.write_all(&mut raw_data) {
        Err(e) => return Err(e.to_string()),
        _ => (),
    }
    Ok(())
}
