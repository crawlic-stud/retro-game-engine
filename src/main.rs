use macroquad::prelude::*;

const WINDOW_HEIGHT: i32 = 1000;
const WINDOW_WIDTH: i32 = 1000;
const CELL_SIZE: i32 = 10;

const GRID_WIDTH: usize = (WINDOW_WIDTH / CELL_SIZE) as usize;
const GRID_HEIGHT: usize = (WINDOW_HEIGHT / CELL_SIZE) as usize;

fn window_conf() -> Conf {
    Conf {
        window_title: "Retro Game Engine".to_string(),
        window_height: WINDOW_HEIGHT,
        window_width: WINDOW_WIDTH,
        window_resizable: false,
        ..Default::default()
    }
}

async fn game() {
    loop {
        clear_background(BLACK);
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position();
            draw_circle_lines(mouse_pos.0, mouse_pos.1, 20.0, 1.0, RED);
        }
        next_frame().await
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    game().await
}
