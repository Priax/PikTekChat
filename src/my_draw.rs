//
// PROJECT, 2023
// PicTekChat
// File description:
// Draw
//
pub mod pik {
    use sdl2::pixels::Color;
    use sdl2::rect::Point;

    pub struct Layer {
        pub pixels: Vec<(i32, i32, Color)>,
        pub visible: bool,
    }

    fn factorial(n: usize) -> usize {
        (1..=n).product()
    }

    fn binomial_coefficient(n: usize, k: usize) -> usize {
        factorial(n) / (factorial(k) * factorial(n - k))
    }

    fn bezier_interpolation(t: f32, points: &[i32]) -> f32 {
        let n = points.len() - 1;
        (0..=n)
            .map(|k| {
                let binomial = binomial_coefficient(n, k);
                binomial as f32 * ((1.0 - t).powi((n - k) as i32) * t.powi(k as i32)) * points[k] as f32
            })
            .sum()
    }

    pub fn draw_bezier_curve(layer: &mut Layer, points: &[Point], color: Color, pencil_size: i32) {
        let num_segments = 100; //Baisser le nombre de segments améliore les performances (mais c'est moins beau)

        for i in 0..num_segments {
            let t = i as f32 / num_segments as f32;

            let x = bezier_interpolation(t, &points.iter().map(|p| p.x()).collect::<Vec<_>>());
            let y = bezier_interpolation(t, &points.iter().map(|p| p.y()).collect::<Vec<_>>());

            let num_pixels = pencil_size * 2 + 1;

            for i in 0..num_pixels {
                let offset_x = (i as f32 - num_pixels as f32 / 2.0) / num_pixels as f32 * pencil_size as f32;
                let offset_y = (i as f32 - num_pixels as f32 / 2.0) / num_pixels as f32 * pencil_size as f32;

                layer.pixels.push((x as i32 + offset_x as i32, y as i32 + offset_y as i32, color));
            }
        }
    }

    #[allow(dead_code)]
    pub fn draw_circle(layer: &mut Layer, center_x: i32, center_y: i32, radius: i32, color: Color) {
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

    fn draw_circle_points(layer: &mut Layer, center_x: i32, center_y: i32, x: i32, y: i32, color: Color) {
        draw_line(layer, center_x - x, center_y + y, center_x + x, center_y + y, color);
        draw_line(layer, center_x - x, center_y - y, center_x + x, center_y - y, color);
        draw_line(layer, center_x - y, center_y + x, center_x + y, center_y + x, color);
        draw_line(layer, center_x - y, center_y - x, center_x + y, center_y - x, color);
    }

    fn draw_line(layer: &mut Layer, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
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
}
