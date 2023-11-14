//
// PROJECT, 2023
// PicTekChat
// File description:
// Draw
//

pub mod pik {
    use sdl2::pixels::Color;
    pub struct Layer {
        pub pixels: Vec<(i32, i32, Color)>,
        pub visible: bool,
    }
}

pub fn draw_circle(layer: &mut pik::Layer, center_x: i32, center_y: i32, radius: i32, color: sdl2::pixels::Color) {
    let mut x = 0;
    let mut y = radius;
    let mut d = 3 - 2 * radius;

    while x <= y {
        draw_circle_points(layer, center_x, center_y, x, y, color);

        x += 1;

        if d > 0 {
            y -= 1;
            d += 4 * (x - y) + 10;
        } else {
            d += 4 * x + 6;
        }

        draw_circle_points(layer, center_x, center_y, x, y, color);
    }
}

fn draw_circle_points(layer: &mut pik::Layer, center_x: i32, center_y: i32, x: i32, y: i32, color: sdl2::pixels::Color) {
    draw_line(layer, center_x - x, center_y + y, center_x + x, center_y + y, color);
    draw_line(layer, center_x - x, center_y - y, center_x + x, center_y - y, color);
    draw_line(layer, center_x - y, center_y + x, center_x + y, center_y + x, color);
    draw_line(layer, center_x - y, center_y - x, center_x + y, center_y - x, color);
}

fn draw_line(layer: &mut pik::Layer, x0: i32, y0: i32, x1: i32, y1: i32, color: sdl2::pixels::Color) {
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    let mut x = x0;
    let mut y = y0;

    loop {
        layer.pixels.push((x, y, color));

        if x == x1 && y == y1 {
            break;
        }

        let e2 = 2 * err;

        if e2 > -dy {
            err -= dy;
            x += sx;
        }

        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
}
