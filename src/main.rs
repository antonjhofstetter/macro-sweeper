use macroquad::prelude::*;

const MOVE_SPEED: f32 = 3.6;

#[macroquad::main("Sweeper")]
async fn main() {
    //setup
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    loop {
        //get input
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_down(KeyCode::A) {
            x -= MOVE_SPEED;
        }

        if is_key_down(KeyCode::D) {
            x += MOVE_SPEED;
        }

        if is_key_down(KeyCode::W) {
            y -= MOVE_SPEED;
        }

        if is_key_down(KeyCode::S) {
            y += MOVE_SPEED;
        }

        //update state
        let direction = Vec2::new(x, y).normalize() * MOVE_SPEED;

        //render
        clear_background(MAGENTA);
        draw_circle(direction.x + x, direction.y + y, 16.0, GREEN);

        //done
        next_frame().await;
    }
}
