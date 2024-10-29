use std::collections::VecDeque;

use nannou::prelude::*;
mod structures;
use crate::structures::*;
use rand::seq::SliceRandom;  // Import the random selection method
use rand::thread_rng;

// inspired by markov jr

struct Model {
    grid: Grid,
    iterations: i32,
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
            grid.set(i as usize, j as usize, Tile::new(i as f32, j as f32, WHITE));
        }
    }
    Model {
        grid: grid,
        iterations: 0,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.iterations += 1;

    let pattern_to_replace = vec![WHITE, WHITE, WHITE];  // Sequence to find
    let replacement_pattern = vec![RED, GREEN, BLUE];  // Replacement sequence
    let mut rng = thread_rng();
    
    // Function to recursively check for pattern match
    fn check_pattern(grid: &Grid, x: usize, y: usize, pattern: &[rgb::Rgb<nannou::color::encoding::Srgb, u8>], mut searched_tiles: Vec<(usize, usize)>, depth: usize) -> Option<Vec<(usize, usize)>> {
        if depth >= pattern.len() {
            return Some(vec![]);  // Found entire pattern
        }
        
        if let Some(tile) = grid.get(x, y) {
            if tile.col == pattern[depth] {
                let mut neighbors = vec![
                    (x.wrapping_sub(1), y), // Left
                    (x + 1, y),             // Right
                    (x, y.wrapping_sub(1)),  // Up
                    (x, y + 1)              // Down
                ];
                neighbors.shuffle(&mut thread_rng());

                for (nx, ny) in neighbors {
                    for tile in &searched_tiles {
                        if tile.0 == nx && tile.1 == ny {
                            return None;
                        } 
                    }
                    searched_tiles.push((x, y));
                    println!("searched {} {}", x, y);
                    if let Some(mut matching_tiles) = check_pattern(grid, nx, ny, pattern, searched_tiles.clone(), depth + 1) {
                        matching_tiles.push((x, y)); // Store matching coordinates
                        return Some(matching_tiles);
                    }
                }
            }
        }
        
        None
    }

    println!("update");
    
    let mut all_matches: Vec<Vec<(usize, usize)>> = vec![];
    // Iterate through the grid to find matching patterns
    for x in 0..model.grid.sx {
        for y in 0..model.grid.sy {
            if let Some(matching_tiles) = check_pattern(&model.grid, x as usize, y as usize, &pattern_to_replace, vec![], 0) {
                println!("Pattern found starting at x: {}, y: {}", x, y);
                all_matches.push(matching_tiles);
            }
        }
    }
    if let Some(random_match) = all_matches.as_slice().choose(&mut thread_rng()) {
        for (i, &(tx, ty)) in random_match.iter().enumerate() {
            let new_tile = Tile::new(tx as f32, ty as f32, replacement_pattern[i].clone());
            model.grid.set(tx, ty, new_tile);
            println!("sey {} {}", tx, ty);
        }
    }
    
    println!("No matching patterns found");
}



fn main() {
    nannou::app(model) // Initialize the app with the model
        .update(update) // Continuously update the app state
        .run();         // Start the app loop
}

fn view(app: &App, model: &Model, frame: Frame) {
    if model.iterations % 1 != 0 {
        return;
    }
    // Begin drawing
    let win = app.window_rect();
    // let t = app.time;
    let draw = app.draw();
    draw.background().color(BLACK);

    model.grid.draw(&draw, &win);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
