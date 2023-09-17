use macroquad::prelude::*;

pub struct Paddle {
    pub name: String,
    pub xpos: f32,
    pub ypos: f32,
    pub width: f32,
    pub height: f32,
    pub speed: f32,
    pub colour: Color,
}

impl Paddle {
    pub fn draw(&mut self) {
        draw_rectangle(self.xpos, self.ypos, self.width, self.height, self.colour);
    }

    pub fn log_position(&mut self) {
        println!(
            "(Ball) {}: xpos={} ypos={}",
            self.name, self.xpos, self.ypos
        );
    }

    pub fn prevent_out_of_bounds(&mut self) {
        if self.ypos <= -1.0 {
            self.ypos = 1.0;
        }
        if self.ypos >= screen_height() - 80.0 {
            self.ypos = screen_height() - 80.0;
        }
    }
}
