use std::vec;

use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run()
}

fn view(app: &App, frame: Frame) {
    // Begin drawing
    let win = app.window_rect();
    let t = app.time;
    let draw = app.draw();

    let grid_width = 20;
    let grid_height = 20; //(20.0 * ((t.sin() + 1.0) / 2.0)) as i32;

    // Clear the background to black.
    draw.background().color(BLACK);

    let mut grid = Vec::new();

    for i in (-grid_width / 2)..(grid_width / 2) {
        for j in (-grid_height / 2)..(grid_height / 2) {
            grid.push(vec2(i as f32 + 0.5, j as f32 + 0.5));
        }
    }

    for point in &grid {
        draw.rect()
            .x(point.x / grid_width as f32 * win.w())
            .y(point.y as f32 / grid_width as f32 * win.h())
            .w((win.w() / grid_width as f32) * 0.5)
            .h((win.h() / grid_height as f32) * 0.5);
    }

    // for i in (-grid_width / 2)..(grid_width / 2) {
    //     for j in (-grid_height / 2)..(grid_height / 2) {
    //         // print!("{}", i as f32 / grid_width  as f32);
    //         draw.rect()
    //             .x(i as f32 / grid_width as f32 * win.w())
    //             .y(j as f32 / grid_width as f32 * win.h())
    //             .w((win.w() / grid_width as f32) * 0.5)
    //             .h((win.h() / grid_height as f32) * 0.5);
    //     }
    // }

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}