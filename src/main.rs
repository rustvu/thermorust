//! A simple heat diffusion simulation.

use nannou::{image, prelude::*};

// Size of the grid
const SIZE_X: usize = 320;
const SIZE_Y: usize = 240;

// Diffusion constant
const ALPHA: f64 = 0.2;

type Grid = [[f64; SIZE_Y]; SIZE_X];
struct Model {
    source: Box<Grid>, // heat source
    grid: Box<Grid>,
}

fn model(_app: &App) -> Model {
    let source_img = image::open("rustvu.png").unwrap().to_luma16();
    let x_offset = (SIZE_X - source_img.width() as usize) / 2;
    let y_offset = (SIZE_Y - source_img.height() as usize) / 2;

    let mut source = Box::new([[0.0; SIZE_Y]; SIZE_X]);
    for (x, y, pixel) in source_img.enumerate_pixels() {
        let img_x = x as usize + x_offset;
        let img_y = (source_img.height() - y - 1) as usize + y_offset;
        source[img_x][img_y] = pixel[0] as f64 / u16::MAX as f64;
    }

    Model {
        source,
        grid: Box::new([[0.0; SIZE_Y]; SIZE_X]),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let mut new_grid = Box::new([[0.0; SIZE_Y]; SIZE_X]);

    for x in 1..SIZE_X - 1 {
        for y in 1..SIZE_Y - 1 {
            new_grid[x][y] = if model.source[x][y] > 0.0 {
                model.source[x][y]
            } else {
                let laplacian = model.grid[x - 1][y]
                    + model.grid[x + 1][y]
                    + model.grid[x][y - 1]
                    + model.grid[x][y + 1]
                    - 4.0 * model.grid[x][y];
                model.grid[x][y] + ALPHA * laplacian
            };
        }
    }

    model.grid = new_grid;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let rect = app.window_rect();
    let draw = app.draw();
    draw.background().color(BLACK);

    let sx = rect.w() / SIZE_X as f32;
    let sy = rect.h() / SIZE_Y as f32;

    for (x, col) in model.grid.iter().enumerate() {
        for (y, &v) in col.iter().enumerate() {
            draw.rect()
                .x_y(
                    rect.left() + sx * (x as f32 + 0.5),
                    rect.bottom() + sy * (y as f32 + 0.5),
                )
                .w_h(sx, sy)
                .rgb(v as f32, 0.0, 0.0);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}
