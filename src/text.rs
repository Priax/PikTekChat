//
// PROJECT, 2023
// PicTekChat
// File description:
// Write text
//
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::ttf::Font;

pub fn render_text(canvas: &mut Canvas<sdl2::video::Window>, font: &Font<'_, '_>, text: &str, position: Point, color: sdl2::pixels::Color) {
    let surface = font
        .render(text)
        .blended(color)
        .map_err(|e| e.to_string())
        .unwrap();

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();

    let (width, height) = surface.size();
    let dest_rect = sdl2::rect::Rect::new(position.x, position.y, width, height);

    canvas.copy(&texture, None, dest_rect).unwrap();
}
