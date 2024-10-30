use nannou::prelude::*;
mod structures;
use crate::structures::*;
use rand::seq::SliceRandom;  // Import the random selection method
use rand::thread_rng;
use std::time::Instant;

// inspired by markov jr
// pick *tile* randomly rather than assemble all possible tiles and select
struct Model {
    grid: Grid,
    iterations: i32,
    previous_window_size: (f32, f32),
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
    grid.set(50, 50, Tile::new(50.0, 50.0, BLACK));
    // grid.set(51, 50, Tile::new(51.0, 50.0, BLACK));
    // grid.set(52, 50, Tile::new(52.0, 50.0, BLACK));
    Model {
        grid: grid,
        iterations: 0,
        previous_window_size: (0.0, 0.0),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let now = Instant::now();
    model.iterations += 1;
    model.grid.iterate();

    let win = app.window_rect();
    if model.previous_window_size != (win.w(), win.h()) {
        model.previous_window_size = (win.w(), win.h());
        model.grid.reset_iterations();
    }

    let mut all_patterns: Vec<Pattern> = vec![];
    all_patterns.append(
        &mut vec![
            // Rule: Replace WHITE space surrounded by BLACKs into a BLACK (to form cave structure)
            Pattern::new(vec![BLACK, WHITE, WHITE], vec![BLACK, GRAY, GRAY]), // random walk
            Pattern::new(vec![GRAY, WHITE], vec![BLUE, GRAY]), // expand
            Pattern::new(vec![GRAY], vec![BLUE]), // homogonize
            Pattern::new(vec![BLUE, WHITE], vec![RED, BLUE]), // expand again

            Pattern::new(vec![BLUE, BLACK, RED, WHITE], vec![BLACK, PINK, BLUE, BLUE]), // random walk
            Pattern::new(vec![BLACK, PINK, BLUE, BLUE], vec![BLACK, BLUE, PINK, BLUE]), // jump over doors if stuck, delete door to avoid jump
            Pattern::new(vec![BLACK, BLUE], vec![BLACK, BLUE]), // random walk
            Pattern::new(vec![BLACK, RED, BLUE], vec![BLACK, RED, BLUE]), // jump over wall if stuck
            Pattern::new(vec![PINK, RED], vec![BLUE, BLUE]), // Expand pink doorways

            Pattern::new(vec![BLUE], vec![BLACK]), // speed up cave filling
        ]
    );
    
    let mut rng = thread_rng();
    
    // Function to recursively check for pattern match
    fn check_pattern(grid: &Grid, x: usize, y: usize, pattern: &[rgb::Rgb<nannou::color::encoding::Srgb, u8>], mut searched_tiles: Vec<(usize, usize)>, mut direction: Option<(i32, i32)>, depth: usize) -> Option<Vec<(usize, usize)>> {
        if depth >= pattern.len() {
            return Some(vec![]);  // Found entire pattern
        }
        
        if let Some(tile) = grid.get(x, y) {
            if tile.col == pattern[depth] {
                let mut neighbors = vec![
                    (-1, 0), // Left
                    (1, 0),             // Right
                    (0, -1),  // Up
                    (0, 1)              // Down
                ];
                if !direction.is_some() {
                    neighbors.shuffle(&mut thread_rng());

                    for neighbor in neighbors {
                            if pattern.len() > depth + 1 {
                                if grid.get((x as i32 + neighbor.0) as usize, (y as i32 + neighbor.1) as usize).is_some() {
                                    if grid.get((x as i32 + neighbor.0) as usize, (y as i32 + neighbor.1) as usize).unwrap().col == pattern[depth + 1] {
                                        direction = Some(neighbor);
                                        break;
                                    }
                                }
                            }else { // len is 1, so dir doesn't matter
                                direction = Some(neighbor);
                                break;
                            }
                            // println!("rand dir {}", direction.unwrap().0);
                    }
                }
                if !direction.is_some() {
                    return None;
                }

                for tile in &searched_tiles {
                    if tile.0 as i32 == x as i32 + direction.unwrap().0 && tile.1 as i32 == y as i32 + direction.unwrap().1 {
                        return None;
                    } 
                }
                searched_tiles.push((x, y));
                if let Some(mut matching_tiles) = check_pattern(grid, 
                        (x as i32 + direction.unwrap().0) as usize, 
                        (y as i32 + direction.unwrap().1) as usize,
                            pattern, searched_tiles, direction,depth + 1) {

                    matching_tiles.push((x, y)); // Store matching coordinates
                    return Some(matching_tiles);
                }
            }
        }

        None
    }

    println!("update");
  
    for sequence in all_patterns {
        let mut all_matches: Vec<Vec<(usize, usize)>> = vec![];
        // Iterate through the grid to find matching patterns
        for x in 0..model.grid.sx {
            for y in 0..model.grid.sy {
                if let Some(matching_tiles) = check_pattern(&model.grid, x as usize, y as usize, &sequence.pattern_to_replace, vec![], None, 0) {
                    // println!("Pattern found starting at x: {}, y: {}", x, y);
                    all_matches.push(matching_tiles);
                }
            }
        }
        if let Some(random_match) = all_matches.as_slice().choose(&mut thread_rng()) {

            for (i, &(tx, ty)) in random_match.iter().enumerate() {
                let new_tile = Tile::new(tx as f32, ty as f32, sequence.replacement_pattern[i].clone());
                model.grid.set(tx, ty, new_tile);
                // println!("set {} {} {}", tx, ty, i);
            }

            let elapsed = now.elapsed();
            println!("Elapsed: {:.2?}", elapsed);
            return;
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
    // if model.iterations % 10 != 0 {
    //     return;
    // }
    // Begin drawing
    let win = app.window_rect();

    // let t = app.time;
    let draw = app.draw();
    // draw.background().color(BLACK);

    model.grid.draw(&draw, &win);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
