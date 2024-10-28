use core::time;

use nannou::prelude::*;
mod structures;
use crate::structures::*;
// inspired by markov jr

struct Model {
    grid: Grid,
}

fn model(app: &App) -> Model {
    // Create a window and set its size
    app.new_window()
        .size(800, 600)
        .view(view)   // Assign the view function to render the content
        .build()
        .unwrap();

    let grid_width: i32 = 100;
    let grid_height: i32 = 100;
    let mut grid: Grid = Grid::new(grid_width as usize, grid_height as usize);
    
    for i in 0..grid_width {
        for j in 0..grid_height {
            grid.set(i as usize, j as usize, Tile::new(i as f32, j as f32));
        }
    }
    Model {
        grid: grid,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let t = app.time;
    println!("update");
    let mut i = 0;
    for row in model.grid.rows.iter_mut() {
        i += 1;
        for tile in row.iter_mut() {
            i += 1;
            tile.set_color(if i % 2 == (t as i32 % 2) { WHITE } else { BLUE });
        }
    }
}


fn main() {
    nannou::app(model) // Initialize the app with the model
        .update(update) // Continuously update the app state
        .run();         // Start the app loop
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let win = app.window_rect();
    // let t = app.time;
    let draw = app.draw();
    draw.background().color(BLACK);

    model.grid.draw(&draw, &win);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
