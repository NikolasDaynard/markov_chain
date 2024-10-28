use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run()
}

// Define a Tile struct with position and size
struct Tile {
    x: f32,
    y: f32,
}
pub struct Grid {
    rows: [[u8; 3]; 3],
}


impl Tile {
    fn new(x: f32, y: f32) -> Self {
        Tile { x, y }
    }

    fn draw(&self, draw: &Draw, win: &Rect, grid_width: i32, grid_height: i32) {
        let tile_width = (win.w() / grid_width as f32) * 0.5;
        let tile_height = (win.h() / grid_height as f32) * 0.5;

        draw.rect()
            .x(self.x / grid_width as f32 * win.w())
            .y(self.y / grid_height as f32 * win.h())
            .w(tile_width)
            .h(tile_height)
            .color(WHITE);
    }
}

impl Grid {
    pub fn get(&self, row: usize, col: usize) -> Option<u8> {
        Some(*self.rows.get(row)?.get(col)?)
    }
}

fn view(app: &App, frame: Frame) {
    // Begin drawing
    let win = app.window_rect();
    let t = app.time;
    let draw = app.draw();
    draw.background().color(BLACK);

    let grid_width = 20;
    let grid_height = 20;
    // let mut mgrid: Grid;

    // Create a grid of Tile objects
    let mut grid: Vec<Vec<Tile>> = Vec::new();
    
    for i in 0..grid_width {
        let mut row: Vec<Tile> = Vec::new();
        for j in 0..grid_height {
            // Initialize each tile with a position
            let x = i as f32 - grid_width as f32 / 2.0;
            let y = j as f32 - grid_height as f32 / 2.0;
            row.push(Tile::new(x + 0.5, y + 0.5)); // center
        }
        grid.push(row);
    }

    // Draw each tile
    for row in &grid {
        for tile in row {
            tile.draw(&draw, &win, grid_width, grid_height);
        }
    }

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
