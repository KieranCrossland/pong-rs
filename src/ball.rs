use macroquad::prelude::*;

#[allow(dead_code)]

pub struct Ball {
    pub x_pos: f32,
    pub y_pos: f32,
    pub x_speed: f32,
    pub y_speed: f32,
    pub radius: f32,
    pub colour: Color,
    pub name: &'static str,
}

impl Ball {
    pub fn update(&mut self) {
            self.y_pos += self.y_speed;
            self.x_pos += self.x_speed;
    }

    pub fn reverse_if_out_of_window(&mut self) {
        if self.x_pos < 0.0 + self.radius || self.x_pos + self.radius > screen_width() {
            self.x_speed =-self.x_speed;
        }

        if self.y_pos - self.radius < 0.0 || self.y_pos + self.radius > screen_height() {
            self.y_speed = -self.y_speed;
        }
    }
    pub fn draw(&self) {
        draw_circle(self.x_pos, self.y_pos, self.radius, self.colour);
    }

    pub fn log_pos(&self) {
        println!("{} x: {} y: {}",self.name, self.x_pos, self.y_pos);
    } 
}
