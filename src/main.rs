use std::{thread::sleep, time::Duration};

use macroquad::prelude::*;

const WINDOW_HEIGHT: usize = 1000;
const WINDOW_WIDTH: usize = 1000;
const CELL_SIZE: usize = 45;

const GRID_THICKNESS: f32 = 2.0;
const SCREEN_WIDTH: usize = 15;
const SCREEN_HEIGHT: usize = 20;
const FPS: f32 = 60.0;
const TIME_PER_FRAME: f32 = 1.0 / FPS;

struct GameConfig {
    bg_color: Color,
}

struct Screen {
    width: usize,
    height: usize,
    padding: usize,
    grid_outline_color: Color,
    grid_bg_color: Color,
    pixel_color: Color,
}
impl Default for Screen {
    fn default() -> Self {
        Self {
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            padding: 1,
            grid_outline_color: LIGHTGRAY,
            grid_bg_color: GRAY,
            pixel_color: WHITE,
        }
    }
}

struct Player<'a> {
    screen: &'a Screen,
    x: i32,
    y: i32,
    moves: &'a [(KeyCode, (i32, i32)); 4],
}

impl Screen {
    fn _translate_coord(&self, coord: usize) -> f32 {
        (coord * CELL_SIZE) as f32
    }

    fn draw_grid(&self) {
        draw_rectangle(
            CELL_SIZE as f32,
            CELL_SIZE as f32,
            self._translate_coord(self.width),
            self._translate_coord(self.height),
            self.grid_bg_color,
        );
        for i in self.padding..(self.width + self.padding) {
            for j in self.padding..(self.height + self.padding) {
                draw_rectangle_lines(
                    self._translate_coord(i),
                    self._translate_coord(j),
                    CELL_SIZE as f32,
                    CELL_SIZE as f32,
                    GRID_THICKNESS,
                    self.grid_outline_color,
                );
            }
        }
    }

    fn draw_pixel(&self, x: i32, y: i32) {
        if (x as usize >= self.width) | (y as usize >= self.height) | (x < 0) | (y < 0) {
            return;
        }

        draw_rectangle(
            self._translate_coord(x as usize + self.padding),
            self._translate_coord(y as usize + self.padding),
            CELL_SIZE as f32,
            CELL_SIZE as f32,
            self.pixel_color,
        )
    }
}

impl Player<'_> {
    fn draw(&self) {
        self.screen.draw_pixel(self.x, self.y);
    }

    fn _move(&mut self, move_x: i32, move_y: i32) {
        self.x += move_x;
        self.y += move_y;
    }

    fn handle_controls(&mut self) {
        for (key, movement) in self.moves.iter() {
            if is_key_pressed(*key) {
                self._move(movement.0, movement.1);
            }
        }
    }
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
    };

    loop {
        let delta = get_frame_time();

        clear_background(config.bg_color);
        screen.draw_grid();
        draw_text(get_fps().to_string().as_str(), 10.0, 40.0, 40.0, WHITE);
        player.draw();
        player.handle_controls();

        sleep(Duration::from_millis(
            ((TIME_PER_FRAME - delta) * 1000.0) as u64,
        ));

        next_frame().await
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    game().await
}
