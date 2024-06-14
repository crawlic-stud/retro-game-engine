use macroquad::prelude::*;

use crate::screen::Screen;

pub(crate) struct Player<'a> {
    pub screen: &'a Screen,
    pub x: i32,
    pub y: i32,
    pub moves: &'a [(KeyCode, (i32, i32)); 4],
    pub move_delay: f32,
    pub current_move_delay: f32,
}

impl Player<'_> {
    pub fn draw(&self) {
        self.screen.draw_pixel(self.x, self.y);
    }

    fn _check_can_move(&self, move_x: i32, move_y: i32) -> bool {
        let new_x = self.x + move_x;
        let new_y = self.y + move_y;

        return (new_x < self.screen.width as i32)
            & (new_x >= 0)
            & (new_y < self.screen.height as i32)
            & (new_y >= 0);
    }

    fn _move(&mut self, move_x: i32, move_y: i32) {
        if !self._check_can_move(move_x, move_y) {
            return;
        }
        self.x += move_x;
        self.y += move_y;
        self.current_move_delay = 0.0;
    }

    pub fn handle_controls(&mut self, delta: f32) {
        for (key, movement) in self.moves.iter() {
            if is_key_pressed(*key) {
                self._move(movement.0, movement.1);
            } else if is_key_down(*key) {
                if self.current_move_delay >= self.move_delay {
                    self._move(movement.0, movement.1);
                } else {
                    self.current_move_delay += delta;
                }
            }
        }
    }
}
