use nannou::prelude::*;

pub struct Grid {
    pub sx: i32,
    pub sy: i32,
    pub rows: Vec<Vec<Tile>>,
}

impl Grid {
    pub fn new(sx: usize, sy: usize) -> Self {
        Grid {
            sx: sx as i32,
            sy: sy as i32,
            rows: vec![vec![Tile::new(0.0, 0.0, WHITE); sx]; sy],  // Initializes the grid with 0s
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
            for mut tile in row {
                tile.draw(draw, win, self.sx, self.sy);
            }  
        }
    }
    pub fn iterate(&mut self) {
        for row in &mut self.rows {
            for tile in row.iter_mut() {
                tile.iterations += 1;
            }
        }
    }  
    pub fn reset_iterations(&mut self) {
        for row in &mut self.rows {
            for tile in row.iter_mut() {
                tile.iterations = 0;
            }
        }
    }    
}

// Define a Tile struct with position and size
#[derive(Copy, Clone)]
pub struct Tile {
    pub x: f32,
    pub y: f32,
    pub col: rgb::Rgb<nannou::color::encoding::Srgb, u8>,
    pub iterations: i32,
}
impl Tile {
    pub fn new(x: f32, y: f32, col: rgb::Rgb<nannou::color::encoding::Srgb, u8>) -> Self {
        Tile { x, y, col, iterations: 0}
    }

    pub fn draw(&self, draw: &Draw, win: &Rect, grid_width: i32, grid_height: i32) {
        if self.iterations > 1 {
            return;
        }

        let tile_width = (win.w() / grid_width as f32) * 0.9;
        let tile_height = (win.h() / grid_height as f32) * 0.9;
        let mut xpos = (((self.x + 0.5) - (grid_width as f32 / 2.0)) / grid_width as f32) * win.w();
        let mut ypos = (((self.y + 0.5) - (grid_height as f32 / 2.0)) / grid_height as f32) * win.h();
        // xpos = xpos * 0.9;
        // ypos = ypos * 0.9;
        draw.quad()
            .x(xpos)
            .y(ypos)
            .w(tile_width)
            .h(tile_height)
            .color(self.col);

        // draw.rect()
        //     .x(xpos)
        //     .y(ypos)
        //     .w(tile_width)
        //     .h(tile_height)
        //     .color(self.col);
    }

    pub fn set_color(&mut self, new_color: rgb::Rgb<nannou::color::encoding::Srgb, u8>) {
        self.col = new_color;
        self.iterations = 0;
    }
}

pub struct Pattern {
    pub pattern_to_replace: Vec<rgb::Rgb<nannou::color::encoding::Srgb, u8>>,
    pub replacement_pattern: Vec<rgb::Rgb<nannou::color::encoding::Srgb, u8>>,
}

impl Pattern {
    pub fn new(pattern_to_replace: Vec<rgb::Rgb<nannou::color::encoding::Srgb, u8>>, replacement_pattern: Vec<rgb::Rgb<nannou::color::encoding::Srgb, u8>>) -> Self {
        Pattern { pattern_to_replace, replacement_pattern }
    }
    pub fn print(&self) {
        for color in self.pattern_to_replace.clone() {
            print!("replacing {} {} {} ", color.red, color.green, color.blue)
        }
        print!("\n");
        for color in self.replacement_pattern.clone() {
            print!("with {} {} {} ", color.red, color.green, color.blue)
        }
    }
}