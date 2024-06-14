use std::{thread::sleep, time::Duration};

use macroquad::prelude::*;

mod screen;
use screen::Screen;
mod player;
use player::Player;

const WINDOW_HEIGHT: usize = 1000;
const WINDOW_WIDTH: usize = 1000;
const CELL_SIZE: usize = 50;
const GRID_THICKNESS: f32 = 3.0;
const SCREEN_WIDTH: usize = 12;
const SCREEN_HEIGHT: usize = 16;

struct GameConfig {
    bg_color: Color,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Retro Game Engine".to_string(),
        window_height: WINDOW_HEIGHT as i32,
        window_width: WINDOW_WIDTH as i32,
        window_resizable: false,
        ..Default::default()
    }
}

async fn game() {
    let config = GameConfig { bg_color: BLACK };

    let screen = Screen {
        width: SCREEN_WIDTH,
        height: SCREEN_HEIGHT,
        cell_size: CELL_SIZE,
        grid_thickness: GRID_THICKNESS,
        ..Default::default()
    };

    let mut player = Player {
        screen: &screen,
        x: 0,
        y: 0,
        moves: &[
            (KeyCode::W, (0, -1)),
            (KeyCode::A, (-1, 0)),
            (KeyCode::S, (0, 1)),
            (KeyCode::D, (1, 0)),
        ],
        move_delay: 0.15,
        current_move_delay: 0.0,
    };

    loop {
        let delta = get_frame_time();

        clear_background(config.bg_color);
        screen.draw_grid();
        draw_text(get_fps().to_string().as_str(), 10.0, 40.0, 40.0, WHITE);
        player.draw();
        player.handle_controls(delta);

        next_frame().await
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    game().await
}
