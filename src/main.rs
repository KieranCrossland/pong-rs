use macroquad::prelude::*;
use paddle::Paddle;
use ball::Ball;
mod ball;
mod paddle;

#[macroquad::main("pong-rs2")]
async fn main() {

    let mut ball = Ball {
        x_pos: screen_width() /2.0,
        y_pos: screen_height() /2.0,
        y_speed: 5.0,
        x_speed: 5.0,
        radius: 25.0,
        colour: WHITE,
        name: "ball",
    };

    let mut player1 = Paddle { //Left Paddle
        x_pos: 15.0,
        y_pos: 50.0,
        y_speed: 5.0,
        height: 100.0,
        width: 25.0,
        colour: GREEN,
        name: "green",
    };

    loop {
        if is_key_pressed(KeyCode::Escape) {
            println!("Shutting down.");
            return;
        }
        /*if is_key_pressed(KeyCode::Space) {
            println!("toggled pause.");
            toggle_pause();
        }*/

        ball.update();
        ball.reverse_if_out_of_window();
        clear_background(DARKGRAY);
        ball.draw();
        player1.draw();
        
        next_frame().await;
    }
}

