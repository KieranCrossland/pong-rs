use macroquad::prelude::*;

pub struct Ball {
    pub x_pos: f32,
    pub y_pos: f32,
    pub x_speed: f32,
    pub y_speed: f32,
    pub radius: f32,
    pub colour: Color,
}

pub struct Paddle {
    pub x_pos: f32,
    pub y_pos: f32,
    pub y_speed: f32,
    pub height: f32,
    pub width: f32,
    pub colour: Color,
    pub id: usize,
    pub score: u16,
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
    pub fn reset(&mut self) {
        self.x_pos = screen_width() / 2.0;
        self.y_pos = screen_height() / 2.0;
    }
    pub fn draw(&self) {
        draw_circle(self.x_pos, self.y_pos, self.radius, self.colour);
    }

    pub fn log_pos(&self) {
        println!("ball: x: {} y: {}", self.x_pos, self.y_pos);
    }
}

impl Paddle {
    pub fn update(&mut self) {}

    pub fn draw(&self) {
        draw_rectangle(self.x_pos, self.y_pos, self.width, self.height, self.colour);
    }

    pub fn log_pos(&self) {
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

fn clamp(val: f32, min: f32, max: f32) -> f32 {
    if val < min {
        return min;
    } else if val > max {
        return max;
    } else {
        return val;
    }
}

pub fn check_collision_circle_rect(b: &Ball, mut r: &Paddle) -> bool {
    let closest_x = clamp(b.x_pos, r.x_pos, r.x_pos + r.width);
    let closest_y = clamp(b.y_pos, r.y_pos, r.y_pos + r.height);
    let distance_x = b.x_pos - closest_x;
    let distance_y = b.y_pos - closest_y;
    if (distance_x * distance_x + distance_y * distance_y) < (b.radius * b.radius) {
        return true;
    } else {
        return false;
    }
}
