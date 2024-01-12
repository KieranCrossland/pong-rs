use macroquad::prelude::*;

pub struct Paddle {
    pub x_pos: f32,
    pub y_pos: f32,
    pub y_speed: f32,
    pub height: f32,
    pub width: f32,
	pub colour: Color,
	pub name: &'static str,
}

impl Paddle {
	pub fn update(&mut self) {
	}
    
	pub fn draw(&self) {
        draw_rectangle(15.0, self.y_pos, self.width, self.height, self.colour);
    }

    pub fn log_pos(&self) {
        println!("{} x: {} y: {}",self.name, self.x_pos, self.y_pos);
    }
}