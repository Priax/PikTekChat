//
// PROJECT, 2023
// PicTekChat
// File description:
// Save
//
use std::path::{Path, PathBuf};

use crate::my_draw::pik::Layer;

pub fn save_file(layers: &[Layer], width: u32, height: u32, filename: &str) -> Result<(), String> {
    let mut image = image::RgbImage::new(width, height);

    for layer in layers {
        if !layer.visible {
            continue; //Skip invisible layers
        }
        for &(x, y, color) in &layer.pixels {
            if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                let pixel = image::Rgb([color.r, color.g, color.b]);
                image.put_pixel(x as u32, y as u32, pixel);
            }
        }
    }

    let valid_extensions = ["jpg", "png", "bmp"];

    let file_path = if let Some(extension) = Path::new(filename).extension().and_then(|ext| ext.to_str()) {
        if !valid_extensions.contains(&extension) {
            return Err(format!("Invalid file extension. Only the following extensions are supported: {}\x1b[0m", valid_extensions.join(", ")));
        }
        filename.to_owned()
    } else {
        let mut path = PathBuf::from(filename);
        path.set_extension("jpg");
        path.to_string_lossy().clone().to_string()
    };

    if Path::new(&file_path).exists() {
        eprintln!("Warning: File already exists.");
    }

    match image.save(&file_path) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Error saving image: {}\x1b[0m", err)),
    }
}
