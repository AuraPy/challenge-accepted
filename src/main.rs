use macroquad::prelude::*;
use macroquad::rand::RandomRange;
use macroquad::audio::{load_sound, play_sound_once};

#[macroquad::main("Challenge Accepted")]
async fn main() {
    let mut ball_x = 330.0;
    let mut ball_y = 80.0;
    let mut dy = 1.0;
    let mut dx = 0.0;
    let kineticmult = 0.75;
    let mut orbs: Vec<(f32, f32)> = Vec::new();
    let mut yelloworbs: Vec<(f32, f32)> = Vec::new();
    let mut milestones = Vec::new();
    let mut score = 0;
    let point = load_sound("src/point.wav").await.unwrap();

    milestones.push(0);

    for _ in 0..5 {
        orbs.push((RandomRange::gen_range(0.0, screen_width()), RandomRange::gen_range(0.0, screen_height())));
    }

    loop {
        clear_background(BLACK);
        for pos in &orbs {
            draw_circle(pos.0, pos.1, 10.0,  WHITE);
        }
        for pos in &yelloworbs {
            draw_circle(pos.0, pos.1, 10.0,  YELLOW);
        }

        draw_circle(ball_x, ball_y, 20.0, Color::from_rgba(255, 100, 140, 255));
        for i in 0..orbs.len() {
            let pos = orbs[i];
            if ((pos.0 - ball_x).abs() < 30.0) && ((pos.1 - ball_y).abs() < 30.0) {
                orbs[i] = (RandomRange::gen_range(0.0, screen_width()), RandomRange::gen_range(0.0, screen_height()));
                score += 1;
                play_sound_once(&point);
            }
        }
        for i in 0..yelloworbs.len() {
            let pos = yelloworbs[i];
            if ((pos.0 - ball_x).abs() < 30.0) && ((pos.1 - ball_y).abs() < 30.0) {
                yelloworbs[i] = (RandomRange::gen_range(0.0, screen_width()), RandomRange::gen_range(0.0, screen_height()));
                score += 2;
                play_sound_once(&point);
            }
        }

        ball_y += dy;
        ball_x += dx;
        dy += 0.15;

        if score % 10 == 0 && !milestones.contains(&score) {
            if score >= 50 {
                yelloworbs.push((RandomRange::gen_range(0.0, screen_width()), RandomRange::gen_range(0.0, screen_height())));
            }
            orbs.push((RandomRange::gen_range(0.0, screen_width()), RandomRange::gen_range(0.0, screen_height())));
            milestones.push(score);
        }

        let text_size = measure_text(&score.to_string(), None, 80, 1.0);
        draw_text(&score.to_string(), screen_width()/2.0-text_size.width/2.0, 70.0, 80.0, WHITE);
        
        if ball_y + 20.0 >= screen_height() {
            if dy > 0.0 {
                dy = -(dy*kineticmult);
            }
            if dy > -0.8 && dy < 0.0 {
                dy = 0.0
            }
        }

        if is_key_down(KeyCode::Space) {
            if ball_y + 20.0 >= screen_height() {
                dy += -4.0
            }
        }
        if is_key_down(KeyCode::A) {
            dx += -0.2;
        } else{
            if is_key_down(KeyCode::D) {
                dx += 0.2;
            } else {
                if dx < -0.1 {
                    dx += 0.2;
                } else {
                    if dx > 0.1 {
                        dx += -0.2;
                    } else {
                        dx = 0.0;
                    }
                }
            }
        }
        if ball_x + 20.0 > screen_width() {
            ball_x = screen_width() - 20.0;
            dx = 0.0;
        }
        if ball_x - 20.0 < 0.0 {
            ball_x = 20.0;
            dx = 0.0;
        }

        next_frame().await
    }
}