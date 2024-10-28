use std::vec;

use nannou::prelude::*;
mod structures;
use crate::structures::*;
use rand::seq::SliceRandom;  // Import the random selection method
use rand::thread_rng;

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
    let mut matching_tiles: Vec<Tile> = vec![];
    let pattern_to_replace = vec![WHITE];
    let replacement_pattern = vec![BLACK];
    // let t = app.time;
    println!("update");
    for row in model.grid.rows.iter_mut() {
        for tile in row.iter_mut() {
            if tile.col == *pattern_to_replace.first().unwrap() {
                matching_tiles.push(*tile);
            }
        }
    }
    // for tile in matching_tiles {
    //     println!("x: {}, y: {}, col: {},{},{}", tile.x, tile.y, tile.col.red, tile.col.green, tile.col.blue);
    // }
    
    if let Some(random_tile) = matching_tiles.as_slice().choose(&mut thread_rng()) {
        println!("Random tile selected: x: {}, y: {}, col: {},{},{}",
            random_tile.x, random_tile.y, 
            random_tile.col.red, random_tile.col.green, random_tile.col.blue
        );
        let mut new_tile = Tile::new(random_tile.x, random_tile.y);
        new_tile.col = replacement_pattern.first().unwrap().clone();
        model.grid.set(random_tile.x as usize, random_tile.y as usize,new_tile);
    } else {
        println!("No matching tiles found");
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
