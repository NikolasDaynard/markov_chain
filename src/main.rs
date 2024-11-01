use nannou::prelude::*;
mod structures;
use crate::structures::*;
mod lexer;
use crate::lexer::*;
use rand::seq::SliceRandom;  // Import the random selection method
use rand::thread_rng;
use std::collections::HashMap;
use std::env;
use std::time::Instant;

// inspired by markov jr
// pick *tile* randomly rather than assemble all possible tiles and select
struct Model {
    prev_mouse: (f32, f32),
    grid: Grid,
    iterations: i32,
    prev_window_size: (f32, f32),
    tilemap: HashMap<Color, Vec<Tile>>,
    rewrite_rules: Vec<Pattern>,
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

    let mut tilemap: HashMap<Color, Vec<Tile>> = HashMap::new();
    for x in 0..grid.sx {
        for y in 0..grid.sy {
            let tile = grid.get(x as usize, y as usize).unwrap();
            let mut tile_vector: Vec<Tile>;
            if tilemap.contains_key(&Color::new(tile.col)) {
                tile_vector = tilemap[&Color::new(tile.col)].clone();
            }else {
                tile_vector = vec![];
            }
            tile_vector.push(tile);
            tilemap.insert(Color::new(tile.col), tile_vector);
        }
    }

    let args: Vec<String> = env::args().collect();
    
    let generated_rules = parse_file(args[1].clone());

    Model {
        prev_mouse: (0.0, 0.0),
        grid: grid,
        iterations: 0,
        prev_window_size: (0.0, 0.0),
        tilemap: tilemap,
        rewrite_rules: generated_rules,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // make a map of tile : color, and then just iterate over the map
    // next thing, make map store what checks tile passes, if it is not updated and fails the check, remove it from the map
    // if you use it as starting tile, and it fails the check, add num to failing checks, if a tile near it updates reset it
    // only need to notify in straight line <- ^ V ->

    // start sequence fail = dead and then needs update to relive

    // multithreading this is *super* possible, one per rule
    // tile update fails | - +
    let now = Instant::now();
    if app.mouse.x != model.prev_mouse.0 || app.mouse.y != model.prev_mouse.1 {
        if model.prev_mouse.0 == -1.0 {
            model.grid.reset_iterations(); // redraw all
            model.prev_mouse.0 = app.mouse.x;
            model.prev_mouse.1 = app.mouse.y;
        }else {
            model.prev_mouse.0 = -1.0;
            model.prev_mouse.1 = -1.0;
        }
    }

    model.iterations += 1;
    model.grid.iterate();

    let win = app.window_rect();
    if model.prev_window_size != (win.w(), win.h()) {
        model.prev_window_size = (win.w(), win.h());
        model.grid.reset_iterations();
    }
    
    // Function to recursively check for pattern match
    fn check_pattern(grid: &Grid, x: usize, y: usize, pattern: &[rgb::Rgb<nannou::color::encoding::Srgb, u8>], mut searched_tiles: Vec<(usize, usize)>, direction: Option<(i32, i32)>, depth: usize) -> Option<Vec<(usize, usize)>> {
        if depth >= pattern.len() {
            return Some(vec![]);  // Found entire pattern
        }
        
        if let Some(tile) = grid.get(x, y) {
            if tile.col == pattern[depth] {
                let mut neighbors = vec![
                    (-1, 0),  // Left
                    (1, 0),   // Right
                    (0, -1),  // Up
                    (0, 1)    // Down
                ];
                neighbors.shuffle(&mut thread_rng());
                for neighbor in neighbors {
                    let mut dirtemp = direction;
                    if !direction.is_some() {
                        dirtemp = Some(neighbor);
                    }

                    for tile in &searched_tiles {
                        if tile.0 as i32 == x as i32 + dirtemp.unwrap().0 && tile.1 as i32 == y as i32 + dirtemp.unwrap().1 {
                            continue;
                        } 
                    }
                    searched_tiles.push((x, y));
                    if let Some(mut matching_tiles) = check_pattern(grid, 
                            (x as i32 + dirtemp.unwrap().0) as usize, 
                            (y as i32 + dirtemp.unwrap().1) as usize,
                                pattern, searched_tiles.clone(), dirtemp,depth + 1) {

                        matching_tiles.push((x, y)); // Store matching coordinates
                        return Some(matching_tiles);
                    }
                }
            }
        }

        None
    }

    for (i, sequence) in model.rewrite_rules.iter().enumerate() {
        let mut all_matches: Vec<Vec<(usize, usize)>> = vec![];
        // Iterate through the grid to find matching patterns
        if !model.tilemap.contains_key(&Color::new(*sequence.pattern_to_replace.first().unwrap())) {
            continue;
        }
        if let Some(pattern_color) = sequence.pattern_to_replace.first() {
            if let Some(tiles) = model.tilemap.get_mut(&Color::new(*pattern_color)) {
                for tile in tiles.iter_mut() {
                    if !tile.is_sequence_live(i) {
                        // tile.print();
                        continue;
                    }

                    if let Some(matching_tiles) = check_pattern(&model.grid, tile.x as usize, tile.y as usize, &sequence.pattern_to_replace, vec![], None, 0) {
                        all_matches.push(matching_tiles);
                        // break; // greedy take (optimal)
                    } else {
                        // TODO: this stupid check is needed, 
                        // only tiles directly next to the new tiles are updates, 
                        // if one of the tiles in a really long sequence is flipped, 
                        // it has no idea this is potentially still open, so limit to just one tile away
                        if !check_pattern(&model.grid, tile.x as usize, tile.y as usize, &[*sequence.pattern_to_replace.first().unwrap()], vec![], None, 0).is_some() {
                            tile.kill(i); 
                        }
                    }
                }
            } else {
                println!("Pattern color not found in tilemap.");
            }
        } else {
            println!("Pattern to replace is empty.");
        }
        
        
        if let Some(random_match) = all_matches.as_slice().choose(&mut thread_rng()) {
            let tilmapelapsed = now.elapsed();

            for (i, &(tx, ty)) in random_match.iter().enumerate() {

                let mut replace_tile = |prev_tile: Tile, new_tile: Tile| {
                    let mut tile_vector: Vec<Tile>;
                    if model.tilemap.contains_key(&Color::new(new_tile.col)) {
                        tile_vector = model.tilemap[&Color::new(new_tile.col)].clone();
                    }else {
                        tile_vector = vec![];
                    }
                    tile_vector.push(new_tile);
                    model.tilemap.insert(Color::new(new_tile.col), tile_vector);

                    // collect garbage
                    if model.tilemap.contains_key(&Color::new(prev_tile.col)) {
                        tile_vector = model.tilemap[&Color::new(prev_tile.col)].clone();
                    }else {
                        tile_vector = vec![];
                    }

                    let mut index = 0;
                    for tile in tile_vector.iter() {
                        if tile.x as usize == tx && tile.y as usize == ty {
                            tile_vector.remove(index);
                            break;
                        }
                        index += 1;
                    }
                };

                let new_tile = Tile::new(tx as f32, ty as f32, sequence.replacement_pattern[i].clone());
                let prev_tile = model.grid.get(tx, ty).unwrap();
                replace_tile(prev_tile, new_tile);

                model.grid.set(tx, ty, new_tile);

                // update tiles live
                let default = Tile::new(-1.0, -1.0, BLACK); 
                model.grid.get(tx + 1, ty).unwrap_or(default).set_live();
                model.grid.get(ty.checked_sub(1).unwrap_or(usize::MAX), ty).unwrap_or(default).set_live();
                model.grid.get(tx, ty + 1).unwrap_or(default).set_live();
                model.grid.get(tx, ty.checked_sub(1).unwrap_or(usize::MAX)).unwrap_or(default).set_live();
                // println!("set {} {} {}", tx, ty, i);

            }

            let elapsed = now.elapsed();
            println!("Elapsed: {:.2?}", elapsed);
            println!("Tile Elapsed: {:.2?}", tilmapelapsed);
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
    // let now = Instant::now();
    // TODO: debug window showing live of hovered tile
    let win = app.window_rect();

    let draw = app.draw();

    model.grid.draw(&draw, &win);
    draw_debug(&win, &draw, model,
    model.grid.get_tile_at_x_y(&win, app.mouse.x, app.mouse.y));

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();

    // println!("Rendering: {:.2?}", now.elapsed());
}

fn draw_debug(win: &Rect, draw: &Draw, model: &Model, tile: Option<Tile>) {
    if tile.is_none() { return; }
    let tile = tile.unwrap();

    // Get the center position of the tile
    let (tile_x, tile_y) = tile.get_position(win, model.grid.sx, model.grid.sy);

    // Set the width and height of the rectangle
    let rect_width = 100.0;
    let rect_height = 50.0;

    // Adjust the position so (x, y) refers to the bottom-left corner of the rectangle
    let adjusted_x = tile_x + rect_width / 2.0;
    let adjusted_y = tile_y + rect_height / 2.0;

    draw.rect()
        .x(adjusted_x)
        .y(adjusted_y)
        .w(rect_width)
        .h(rect_height)
        .color(BLACK);

        // x y
    draw.text((tile.x.to_string() + ", " + &tile.y.to_string()).as_str())
        .x(adjusted_x - rect_height / 2.2)
        .y(adjusted_y + rect_height / 2.2)
        .w(rect_width)
        .h(rect_height)
        .color(PINK);

    // color
    draw.text((tile.col.red.to_string() + ", " + &tile.col.green.to_string() + ", " + &tile.col.blue.to_string()).as_str())
        .x(adjusted_x - rect_height / 2.2)
        .y(adjusted_y + rect_height / 3.8)
        .w(rect_width)
        .h(rect_height)
        .color(PINK);

    // live sequences
    draw.text(structures::Tile::format_u32_as_bits(tile.live_sequences).as_str())
        .x(adjusted_x)
        .y(adjusted_y)
        .w(rect_width)
        .h(rect_height)
        .color(PINK).font_size(7);
}

