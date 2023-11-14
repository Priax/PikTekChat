//
// PROJECT, 2023
// PicTekChat
// File description:
// Rust :D
//
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Point;

mod my_draw;
mod save;
mod text;
mod open;
mod layer;

use crate::my_draw::pik::Layer;

fn main() {
    let mut window_width: u32 = 800;
    let mut window_height: u32 = 600;
    let mut layer_number: u32 = 1;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("PicTekChat", window_width, window_height)
        .position_centered()
        .resizable()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();

    let mut layers: Vec<Layer> = vec![Layer {
        pixels: Vec::new(),
        visible: true,
    }];
    let mut current_layer_index = 0;

    let ttf_context = sdl2::ttf::init().unwrap();
    let font_size = 30;
    let font = ttf_context.load_font("./ThaleahFat/ThaleahFat.ttf", font_size).unwrap();
    let color = Color::WHITE;

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut is_drawing = false;
    let colors = [
        Color::MAGENTA,
        Color::BLUE,
        Color::RED,
        Color::GREEN,
        Color::CYAN,
        Color::WHITE,
        Color::BLACK,
    ];
    let mut current_color_index = 0;
    let mut pencil_size = 3;

    'gameloop: loop {
        for evt in event_pump.poll_iter() {
            match evt {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'gameloop;
                },
                Event::Window {win_event: sdl2::event::WindowEvent::Resized(width, height), ..} => {
                    window_width = width as u32;
                    window_height = height as u32;
                },
                Event::MouseButtonDown {mouse_btn, x, y, ..} => {
                    if mouse_btn == MouseButton::Left && layers[current_layer_index].visible {
                            is_drawing = true;
                            my_draw::draw_circle(&mut layers[current_layer_index], x, y, pencil_size, colors[current_color_index]);
                    }
                },
                Event::MouseButtonUp {mouse_btn, ..} => {
                    if mouse_btn == MouseButton::Left {
                        is_drawing = false;
                    }
                },
                Event::MouseMotion { x, y, .. } => {
                    if is_drawing {
                        my_draw::draw_circle(&mut layers[current_layer_index], x, y, pencil_size, colors[current_color_index]);
                    }
                },
                Event::KeyDown {keycode: Some(Keycode::C), ..} => {
                    current_color_index = (current_color_index + 1) % colors.len();
                },
                Event::KeyDown {keycode: Some(Keycode::I), ..} => {
                    if pencil_size < 20 {
                        pencil_size += 1;
                    }
                },
                Event::KeyDown {keycode: Some(Keycode::D), ..} => {
                    if pencil_size > 1 {
                        pencil_size -= 1;
                    }
                },
                Event::KeyDown {keycode: Some(Keycode::O), ..} => {
                    let filename = get_user_input();
                    match open::open_image(&filename) {
                        Ok(pixels) => {
                            layers[current_layer_index].pixels = pixels;
                        }
                        Err(err) => {
                            eprintln!("Error loading image: {}", err);
                        }
                    }
                },
                Event::KeyDown {keycode: Some(Keycode::F), ..} => {
                    let filename = get_user_input();
                    match save::save_file(&layers, window_width, window_height, &filename) {
                        Ok(()) => println!("\x1b[32mImage saved successfully.\x1b[0m"),
                        Err(err) => eprintln!("\x1b[31mError saving image: {}", err),
                    }
                },
                Event::KeyDown {keycode: Some(Keycode::N), ..} => {
                    current_layer_index = (current_layer_index + 1) % layers.len();
                }
                Event::KeyDown {keycode: Some(Keycode::P), ..} => {
                    current_layer_index = (current_layer_index + layers.len() - 1) % layers.len();
                }
                Event::KeyDown {keycode: Some(Keycode::V), ..} => {
                    layers[current_layer_index].visible = !layers[current_layer_index].visible;
                }
                Event::KeyDown {keycode: Some(Keycode::X), ..} => {
                    layers[current_layer_index].pixels.clear();
                },
                Event::KeyDown { keycode: Some(Keycode::A), ..} => {
                    layers.push(layer::create_new_layer());
                    layer_number += 1;
                }
                Event::KeyDown { keycode: Some(Keycode::R), ..} => {
                    if layer_number > 1 {
                        layer::delete_layer(&mut layers, current_layer_index);
                        layer_number -= 1;
                        current_layer_index = (current_layer_index + layers.len() - 1) % layers.len();
                    }
                }
                _ => (),
            }
        }

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        for layer in &layers {
            if layer.visible {
                for &(x, y, color) in &layer.pixels {
                    canvas.set_draw_color(color);
                    canvas.draw_point(Point::new(x, y)).unwrap();
                }
            }
        }

        text::render_text(&mut canvas, &font, &format!("Pencil Size: {}", pencil_size), Point::new(10, 10), color);
        text::render_text(&mut canvas, &font, &format!("Current layer: {}", current_layer_index), Point::new(10, 30), color);
        text::render_text(&mut canvas, &font, &format!("Number of layers: {}", layer_number), Point::new(250, 30), color);
        let color_name = match colors[current_color_index] {
            Color::MAGENTA => "Magenta",
            Color::BLUE => "Bleu",
            Color::RED => "Rouge",
            Color::GREEN => "Vert",
            Color::CYAN => "Cyan",
            Color::WHITE => "Blanc",
            Color::BLACK => "Noir",
            _ => "Unknown",
        };
        text::render_text(&mut canvas, &font, &format!("Couleur: {}", color_name), Point::new(250, 10), color);

        canvas.present();
    }
}

fn get_user_input() -> String {
    use std::io::{stdin,stdout,Write};
    let mut s=String::new();
    print!("Please enter filename: ");
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }

    s
}
