use nannou::prelude::*;
// inspired by markov jr
fn main() {
    nannou::sketch(view).run()
}

// Define a Tile struct with position and size
#[derive(Copy, Clone)]
struct Tile {
    x: f32,
    y: f32,
}
pub struct Grid {
    sx: i32,
    sy: i32,
    rows: Vec<Vec<Tile>>,
}

impl Grid {
    fn new(sx: usize, sy: usize) -> Self {
        Grid {
            sx: sx as i32,
            sy: sy as i32,
            rows: vec![vec![Tile::new(0.0, 0.0); sx]; sy],  // Initializes the grid with 0s
        }
    }

    fn get(&self, row: usize, col: usize) -> Option<Tile> {
        Some(*self.rows.get(row)?.get(col)?)
    }
    fn set(&mut self, row: usize, col: usize, value: Tile) -> Option<()> {
        let row_ref = self.rows.get_mut(row)?; // Mutable reference to the row
        let col_ref = row_ref.get_mut(col)?;   // Mutable reference to the column
        *col_ref = value;
        Some(())
    }
    fn draw(&self, draw: &Draw, win: &Rect) {
        for row in self.rows.clone() {
            for tile in row {
                tile.draw(draw, win, self.sx, self.sy);
            }  
        }
    }
}

impl Tile {
    fn new(x: f32, y: f32) -> Self {
        Tile { x, y }
    }

    fn draw(&self, draw: &Draw, win: &Rect, grid_width: i32, grid_height: i32) {
        let tile_width = (win.w() / grid_width as f32) * 0.5;
        let tile_height = (win.h() / grid_height as f32) * 0.5;
        let xpos = (((self.x + 0.5) - (grid_width as f32 / 2.0)) / grid_width as f32) * win.w();
        let ypos = (((self.y + 0.5) - (grid_height as f32 / 2.0)) / grid_height as f32) * win.h();

        draw.rect()
            .x(xpos)
            .y(ypos)
            .w(tile_width)
            .h(tile_height)
            .color(WHITE);
    }
}

fn view(app: &App, frame: Frame) {
    // Begin drawing
    let win = app.window_rect();
    let t = app.time;
    let draw = app.draw();
    draw.background().color(BLACK);

    let grid_width: i32 = 20;
    let grid_height: i32 = 20;
    let mut grid: Grid = Grid::new(grid_width as usize, grid_height as usize);
    
    for i in 0..grid_width {
        for j in 0..grid_height {
            grid.set(i as usize, j as usize, Tile::new(i as f32, j as f32));
        }
    }

    grid.draw(&draw, &win);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
