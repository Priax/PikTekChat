//
// PROJECT, 2023
// PicTekChat
// File description:
// Open
//
use std::path::Path;
use image::GenericImageView;

use sdl2::pixels::Color;

pub fn open_image(file_path: &str) -> Result<Vec<(i32, i32, Color)>, String> {
    let img = image::open(&Path::new(file_path)).map_err(|e| format!("Error opening image: {}", e))?;

    let mut pixels = Vec::new();
    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y);
            let color = Color::RGBA(pixel.0[0], pixel.0[1], pixel.0[2], pixel.0[3]);
            pixels.push((x as i32, y as i32, color));
        }
    }

    Ok(pixels)
}
