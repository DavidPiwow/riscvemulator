use macroquad::color::BLACK;
use macroquad::window::{clear_background, next_frame};
use crate::app::{update_app, AppState};

mod cpu;
mod instruction;
mod assembler;
mod app;

#[macroquad::main("CPU Viewer")]
async fn main()  {
    let mut state = AppState::default();

    loop {
        clear_background(BLACK);
        update_app(&mut state);
        next_frame().await
    }

}
