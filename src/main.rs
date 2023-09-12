use macroquad::prelude::*;
use std::process;
mod ball;
mod paddle;

#[macroquad::main("Kieran's Pong in Rust (macroquad)")]
async fn main() {
    let mut player1 = paddle::Paddle {
        name: "player1".to_string(),
        xpos: 10.0,
        ypos: screen_height() / 2.0 - 70.0,
        width: 20.0,
        height: 80.0,
        speed: 2.0,
        colour: BLUE,
    };

    let mut player2 = paddle::Paddle {
        name: "player2".to_string(),
        xpos: screen_width() - 30.0,
        ypos: screen_height() / 2.0 - 70.0,
        width: 20.0,
        height: 80.0,
        speed: 2.0,
        colour: RED,
    };

    let mut ball1 = ball::Ball {
        name: "Ball1".to_string(),
        xpos: screen_width() / 2.0,
        ypos: screen_height() / 2.0 - 30.0,
        radius: 20.0,
        speed: 2.0,
        colour: WHITE,
    };

    println!("(Game) Entering gameloop");
    player1.log_position();
    player2.log_position();
    ball1.log_position();
    loop {
        clear_background(BLACK);
        //system keybinds
        if is_key_down(KeyCode::Escape) {
            println!("(KeyCode::Escape) Exiting process");
            process::exit(0);
        }
        //player1 keybinds
        if is_key_down(KeyCode::W) {
            player1.ypos -= player1.speed;
        }
        if is_key_down(KeyCode::S) {
            player1.ypos += player1.speed;
        }

        //player2 keybinds
        if is_key_down(KeyCode::Up) {
            player2.ypos -= player2.speed;
        }
        if is_key_down(KeyCode::Down) {
            player2.ypos += player2.speed;
        }

        //ball keybinds (for debugging)
        if is_key_down(KeyCode::A) {
            ball1.xpos -= ball1.speed;
        }
        if is_key_down(KeyCode::D) {
            ball1.xpos += ball1.speed;
        }

        player1.draw();
        player2.draw();
        ball1.draw();
        ball1.check_if_out_of_bounds();
        player1.check_if_out_of_bounds();
        player2.check_if_out_of_bounds();
        ball1.update_position();

        draw_text("pong-rs", 30.0, 40.0, 64.0, WHITE);
        next_frame().await;
    }
}
