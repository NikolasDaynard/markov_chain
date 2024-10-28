use nannou::prelude::*;

// Define a Tile struct with position and size
#[derive(Copy, Clone)]
pub struct Tile {
    x: f32,
    y: f32,
}
pub struct Grid {
    sx: i32,
    sy: i32,
    rows: Vec<Vec<Tile>>,
}

impl Grid {
    pub fn new(sx: usize, sy: usize) -> Self {
        Grid {
            sx: sx as i32,
            sy: sy as i32,
            rows: vec![vec![Tile::new(0.0, 0.0); sx]; sy],  // Initializes the grid with 0s
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<Tile> {
        Some(*self.rows.get(row)?.get(col)?)
    }
    pub fn set(&mut self, row: usize, col: usize, value: Tile) -> Option<()> {
        let row_ref = self.rows.get_mut(row)?; // Mutable reference to the row
        let col_ref = row_ref.get_mut(col)?;   // Mutable reference to the column
        *col_ref = value;
        Some(())
    }
    pub fn draw(&self, draw: &Draw, win: &Rect) {
        for row in self.rows.clone() {
            for tile in row {
                tile.draw(draw, win, self.sx, self.sy);
            }  
        }
    }
}

impl Tile {
    pub fn new(x: f32, y: f32) -> Self {
        Tile { x, y }
    }

    pub fn draw(&self, draw: &Draw, win: &Rect, grid_width: i32, grid_height: i32) {
        let tile_width = (win.w() / grid_width as f32) * 0.5;
        let tile_height = (win.h() / grid_height as f32) * 0.5;
        let xpos = (((self.x + 0.5) - (grid_width as f32 / 2.0)) / grid_width as f32) * win.w();
        let ypos = (((self.y + 0.5) - (grid_height as f32 / 2.0)) / grid_height as f32) * win.h();

        draw.rect()
            .x(xpos)
            .y(ypos)
            .w(tile_width)
            .h(tile_height)
            .color(BEIGE);
    }
}
