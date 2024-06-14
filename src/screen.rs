use macroquad::prelude::*;

pub(crate) struct Screen {
    pub width: usize,
    pub height: usize,
    pub cell_size: usize,
    pub grid_thickness: f32,
    pub padding: usize,
    pub grid_outline_color: Color,
    pub grid_bg_color: Color,
    pub pixel_color: Color,
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            width: 1000,
            height: 1000,
            cell_size: 45,
            grid_thickness: 2.0,
            padding: 1,
            grid_outline_color: LIGHTGRAY,
            grid_bg_color: GRAY,
            pixel_color: WHITE,
        }
    }
}

impl Screen {
    fn _translate_coord(&self, coord: usize) -> f32 {
        (coord * self.cell_size) as f32
    }

    pub fn draw_grid(&self) {
        draw_rectangle(
            self.cell_size as f32,
            self.cell_size as f32,
            self._translate_coord(self.width),
            self._translate_coord(self.height),
            self.grid_bg_color,
        );
        for i in self.padding..(self.width + self.padding) {
            for j in self.padding..(self.height + self.padding) {
                draw_rectangle_lines(
                    self._translate_coord(i),
                    self._translate_coord(j),
                    self.cell_size as f32,
                    self.cell_size as f32,
                    self.grid_thickness,
                    self.grid_outline_color,
                );
            }
        }
    }

    pub fn draw_pixel(&self, x: i32, y: i32) {
        if (x as usize >= self.width) | (y as usize >= self.height) | (x < 0) | (y < 0) {
            return;
        }

        draw_rectangle(
            self._translate_coord(x as usize + self.padding),
            self._translate_coord(y as usize + self.padding),
            self.cell_size as f32,
            self.cell_size as f32,
            self.pixel_color,
        )
    }
}
