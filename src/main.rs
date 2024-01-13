use macroquad::prelude::*;
use std::env;
mod entity;

const SCORE_REQUIRED_FOR_WIN: u16 = 5;

fn reset_game(b: &mut entity::Ball, p1: &mut entity::Paddle, p2: &mut entity::Paddle) {
    b.reset();
    p1.score = 0;
    p2.score = 0;
    p1.y_pos = screen_height() / 2.0;
    p2.y_pos = screen_height() / 2.0;
}

#[macroquad::main("pong-rs2")]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let mut enable_logging = false;

    if args.len() > 1 {
        match args[1].as_str() {
            "--enable_logging" => enable_logging = true,
            _ => {}
        }
    }

    let mut ball = entity::Ball {
        x_pos: screen_width() / 2.0,
        y_pos: screen_height() / 2.0,
        y_speed: 350.0,
        x_speed: 350.0,
        radius: 25.0,
        colour: WHITE,
    };

    let mut player1 = entity::Paddle {
        x_pos: 25.0,
        y_pos: screen_height() / 2.0 - 100.0,
        y_speed: 500.0,
        height: 100.0,
        width: 25.0,
        colour: GREEN,
        id: 0,
        score: 0,
    };

    let mut player2 = entity::Paddle {
        x_pos: screen_width() - 50.0,
        y_pos: screen_height() / 2.0 - 100.0,
        y_speed: 500.0,
        height: 100.0,
        width: 25.0,
        colour: BLUE,
        id: 1,
        score: 0,
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

        on_player_scored(&mut ball, &mut player1, &mut player2);

        if player1.score >= SCORE_REQUIRED_FOR_WIN {
            println!("player 1 wins!");
            reset_game(&mut ball, &mut player1, &mut player2);
        }
        if player2.score >= SCORE_REQUIRED_FOR_WIN {
            println!("player 2 wins!");
            reset_game(&mut ball, &mut player1, &mut player2);
        }
        if enable_logging {
            player1.log_pos();
            player2.log_pos();
        }

        if entity::check_collision_circle_rect(&ball, &player1) {
            ball.x_speed = -ball.x_speed;
        }
        if entity::check_collision_circle_rect(&ball, &player2) {
            ball.x_speed = -ball.x_speed;
        }

        clear_background(BLACK);

        draw_text(&player1.score.to_string(), 50.0, 40.0, 40.0, player1.colour);
        draw_text(
            &player2.score.to_string(),
            screen_width() - 70.0,
            40.0,
            40.0,
            player2.colour,
        );
        ball.draw();
        player1.draw();
        player2.draw();

        next_frame().await;
    }
}

fn on_player_scored(b: &mut entity::Ball, r1: &mut entity::Paddle, r2: &mut entity::Paddle) {
    if b.x_pos - b.radius <= 0.0 {
        r2.score += 1;
        b.reset();
    }
    if b.x_pos + b.radius >= screen_width() {
        r1.score += 1;
        b.reset();
    }
}
