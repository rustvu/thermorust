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
    let mut model = Model {
        source: Box::new([[0.0; SIZE_Y]; SIZE_X]),
        grid: Box::new([[0.0; SIZE_Y]; SIZE_X]),
    };

    let source_img = image::open("rustvu.png").unwrap().to_luma16();
    let x_offset = (SIZE_X - source_img.width() as usize) / 2;
    let y_offset = (SIZE_Y - source_img.height() as usize) / 2;
    source_img.enumerate_pixels().for_each(|(x, y, pixel)| {
        let x = x as usize;
        let y = (source_img.height() - y) as usize;
        model.source[x + x_offset][y + y_offset] = pixel[0] as f64 / u16::MAX as f64;
    });

    model
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let source = &model.source;
    let grid = &model.grid;
    let mut new_grid = Box::new([[0.0; SIZE_Y]; SIZE_X]);

    for x in 1..SIZE_X - 1 {
        for y in 1..SIZE_Y - 1 {
            if source[x][y] > 0.0 {
                new_grid[x][y] = source[x][y];
            } else {
                let laplacian = -4.0 * grid[x][y]
                    + grid[x - 1][y]
                    + grid[x + 1][y]
                    + grid[x][y - 1]
                    + grid[x][y + 1];
                new_grid[x][y] = ALPHA * laplacian + model.grid[x][y];
            }
        }
    }

    model.grid = new_grid;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let boundary = app.window_rect();

    let draw = app
        .draw()
        .xy(boundary.bottom_left())
        .scale_x(boundary.w() / SIZE_X as f32)
        .scale_y(boundary.h() / SIZE_Y as f32);

    draw.background().color(BLACK);

    for x in 0..SIZE_X {
        for y in 0..SIZE_Y {
            draw.rect().x_y(x as f32, y as f32).w_h(1.0, 1.0).rgb(
                model.grid[x][y] as f32,
                0.0,
                0.0,
            );
        }
    }
    
    draw.to_frame(app, &frame).unwrap();

    println!("FPS: {}", 1000 / app.duration.since_prev_update.as_millis());
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}
