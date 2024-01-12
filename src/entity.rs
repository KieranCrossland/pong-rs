use macroquad::prelude::*;

pub struct Ball {
    pub x_pos: f32,
    pub y_pos: f32,
    pub x_speed: f32,
    pub y_speed: f32,
    pub radius: f32,
    pub colour: Color,
    pub id: usize,
}

pub struct Paddle {
    pub x_pos: f32,
    pub y_pos: f32,
    pub y_speed: f32,
    pub height: f32,
    pub width: f32,
    pub colour: Color,
    pub id: usize,
}

impl Ball {
    pub fn update(&mut self) {
        self.y_pos += self.y_speed;
        self.x_pos += self.x_speed;
    }

    pub fn reverse_if_out_of_window(&mut self) {
        if self.x_pos < 0.0 + self.radius || self.x_pos + self.radius > screen_width() {
            self.x_speed = -self.x_speed;
        }
        if self.y_pos - self.radius < 0.0 || self.y_pos + self.radius > screen_height() {
            self.y_speed = -self.y_speed;
        }
    }

    pub fn draw(&self) {
        draw_circle(self.x_pos, self.y_pos, self.radius, self.colour);
    }

    fn log_pos(&self) {
        println!("id: {} x: {} y: {}", self.id, self.x_pos, self.y_pos);
    }
}

impl Paddle {
    pub fn update(&mut self) {}

    pub fn draw(&self) {
        draw_rectangle(self.x_pos, self.y_pos, self.width, self.height, self.colour);
    }

    fn log_pos(&self) {
        println!("id: {} x: {} y: {}", self.id, self.x_pos, self.y_pos);
    }
    pub fn control(&mut self) {
        if self.id == 0 {
            if is_key_down(KeyCode::S) {
                self.y_pos += self.y_speed;
            }
            if is_key_down(KeyCode::W) {
                self.y_pos -= self.y_speed;
            }
        }
        if self.id == 1 {
            if is_key_down(KeyCode::Down) {
                self.y_pos += self.y_speed;
            }
            if is_key_down(KeyCode::Up) {
                self.y_pos -= self.y_speed;
            }
        }
    }
}
