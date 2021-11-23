use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(LIGHTGRAY);

        draw_rectangle(50.0, 30.0, 100.0, 50.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_text("Word", 51.0, 55.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
