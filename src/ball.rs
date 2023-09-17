use macroquad::prelude::*;

pub struct Ball {
    pub name: String,
    pub xpos: f32,
    pub ypos: f32,
    pub radius: f32,
    pub speed: f32,
    pub colour: Color,
}

impl Ball {
    pub fn draw(&mut self) {
        draw_circle(self.xpos, self.ypos, self.radius, self.colour);
    }

    pub fn log_position(&mut self) {
        println!(
            "(Ball) {}: xpos={} ypos={}",
            self.name, self.xpos, self.ypos
        );
    }

    pub fn prevent_out_of_bounds(&mut self) {
        if self.ypos + self.radius >= screen_height() || self.ypos - self.radius <= 0.0 {
            self.speed *= -1.0;
        }
        if self.xpos - 200.0 + self.radius >= screen_height() || self.xpos - self.radius <= 0.0 {
            self.speed *= -1.0;
        }
    }

    pub fn update_position(&mut self) {
        self.xpos += self.speed;
        self.ypos += self.speed;
    }
}
