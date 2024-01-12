use macroquad::prelude::*;
mod entity;

#[macroquad::main("pong-rs2")]
async fn main() {
    let mut ball = entity::Ball {
        x_pos: screen_width() / 2.0,
        y_pos: screen_height() / 2.0,
        y_speed: 5.0,
        x_speed: 5.0,
        radius: 25.0,
        colour: WHITE,
        id: 0,
    };

    let mut player1 = entity::Paddle {
        //Left Paddle
        x_pos: 15.0,
        y_pos: 50.0,
        y_speed: 5.0,
        height: 100.0,
        width: 25.0,
        colour: GREEN,
        id: 0,
    };

    let mut player2 = entity::Paddle {
        //Left Paddle
        x_pos: screen_width() - 45.0,
        y_pos: 50.0,
        y_speed: 5.0,
        height: 100.0,
        width: 25.0,
        colour: BLUE,
        id: 1,
    };

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        player1.control();
        player2.control();
        ball.update();
        player1.update();
        ball.reverse_if_out_of_window();
        clear_background(DARKGRAY);
        ball.draw();
        player1.draw();
        player2.draw();

        next_frame().await;
    }
}
