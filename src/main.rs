use macroquad::prelude::*;

#[macroquad::main("Sweeper")]
async fn main() {
    loop {
        clear_background(MAGENTA);
        next_frame().await;
    }
}
