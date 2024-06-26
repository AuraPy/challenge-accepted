use macroquad::prelude::*;
use macroquad::ui;
use macroquad::rand::RandomRange;
use macroquad::audio::{load_sound, play_sound_once};

#[macroquad::main("Challenge Accepted")]
async fn main() {
    let left = {
        let button_style =  ui::root_ui().style_builder()
            .background(
                Image::from_file_with_format(
                    include_bytes!("left.png"),
                    None,
                )
                .unwrap(),
            )
            .background_margin(RectOffset::new(100.0, 0.0, 100.0, 0.0))
            .build();

        ui::Skin {
            button_style,
            ..ui::root_ui().default_skin()
        }
    };
    let right = {
        let button_style =  ui::root_ui().style_builder()
            .background(
                Image::from_file_with_format(
                    include_bytes!("right.png"),
                    None,
                )
                .unwrap(),
            )
            .background_margin(RectOffset::new(100.0, 0.0, 100.0, 0.0))
            .build();

        ui::Skin {
            button_style,
            ..ui::root_ui().default_skin()
        }
    };
    let jump = {
        let button_style =  ui::root_ui().style_builder()
            .background(
                Image::from_file_with_format(
                    include_bytes!("jump.png"),
                    None,
                )
                .unwrap(),
            )
            .background_margin(RectOffset::new(100.0, 0.0, 100.0, 0.0))
            .build();

        ui::Skin {
            button_style,
            ..ui::root_ui().default_skin()
        }
    };
    let mobile = {
        let button_style =  ui::root_ui().style_builder()
            .background(
                Image::from_file_with_format(
                    include_bytes!("mobile.png"),
                    None,
                )
                .unwrap(),
            )
            .background_margin(RectOffset::new(100.0, 0.0, 100.0, 0.0))
            .build();

        ui::Skin {
            button_style,
            ..ui::root_ui().default_skin()
        }
    };
    let retry = {
        let button_style =  ui::root_ui().style_builder()
            .background(
                Image::from_file_with_format(
                    include_bytes!("retry.png"),
                    None,
                )
                .unwrap(),
            )
            .background_margin(RectOffset::new(200.0, 0.0, 100.0, 0.0))
            .build();

        ui::Skin {
            button_style,
            ..ui::root_ui().default_skin()
        }
    };
    let mut alive = true;
    let mut ball_x = 330.0;
    let mut ball_y = 80.0;
    let mut dy = 1.0;
    let mut dx = 0.0;
    let kineticmult = 0.75;
    let mut orbs: Vec<(f32, f32)> = Vec::new();
    let mut yelloworbs: Vec<(f32, f32)> = Vec::new();
    let mut blueorbs: Vec<(f32, f32, f32, bool)> = Vec::new();
    let mut bomborbs: Vec<(f32, f32, f32, i32, f32, f32, i32)> = Vec::new();
    let mut milestones = Vec::new();
    let mut score = 0;
    let point = load_sound("src/point.wav").await.unwrap();
    let point2 = load_sound("src/point2.wav").await.unwrap();
    let point3 = load_sound("src/point3.wav").await.unwrap();
    let mobilemodesound = load_sound("src/mobilemode.wav").await.unwrap();
    let levelup = load_sound("src/levelup.wav").await.unwrap();
    let explosion = load_sound("src/explosion.wav").await.unwrap();
    let mut mobilemode = false;
    let mut rectright: Rect = Rect::new(0.0, 0.0, 0.0, 0.0);
    let mut rectleft: Rect = Rect::new(0.0, 0.0, 0.0, 0.0);
    let mut rectjump: Rect = Rect::new(0.0, 0.0, 0.0, 0.0);

    milestones.push(0);

    for _ in 0..5 {
        orbs.push((RandomRange::gen_range(0.0, screen_width()), RandomRange::gen_range(0.0, screen_height())));
    }

    loop {
        clear_background(BLACK);
        if alive {
            ui::root_ui().push_skin(&mobile);
            let buttonmobile = ui::root_ui().button(vec2(screen_width()-50.0, 0.0), "");
            if buttonmobile {
                play_sound_once(&mobilemodesound);
                if mobilemode {
                    mobilemode = false
                } else {
                    mobilemode = true
                }
            }
            if mobilemode {
                ui::root_ui().pop_skin();
                ui::root_ui().push_skin(&left);
                let _buttonleft = ui::root_ui().button(vec2(0.0, screen_height()-100.0), "");
                rectleft = Rect::new(0.0, screen_height()-100.0, 100.0, 100.0);
                ui::root_ui().pop_skin();
                ui::root_ui().push_skin(&right);
                let _buttonright = ui::root_ui().button(vec2(120.0, screen_height()-100.0), "");
                rectright = Rect::new(120.0, screen_height()-100.0, 100.0, 100.0);
                ui::root_ui().pop_skin();
                ui::root_ui().push_skin(&jump);
                let _buttonjump = ui::root_ui().button(vec2(screen_width()-100.0, screen_height()-100.0), "");
                rectjump = Rect::new(screen_width()-100.0, screen_height()-100.0, 100.0, 100.0);
            }

            for pos in &orbs {
                draw_circle(pos.0, pos.1, 10.0,  WHITE);
            }
            for pos in &yelloworbs {
                draw_circle(pos.0, pos.1, 10.0,  YELLOW);
            }
            for pos in &blueorbs {
                draw_circle(pos.0, pos.1, 10.0,  BLUE);
            }
            for pos in &bomborbs {
                draw_circle(pos.0, pos.1, 10.0,  GRAY);
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
                    play_sound_once(&point2);
                }
            }
            for i in 0..blueorbs.len() {
                let pos = blueorbs[i];
                if ((pos.0 - ball_x).abs() < 30.0) && ((pos.1 - ball_y).abs() < 30.0) {
                    let startx = RandomRange::gen_range(0.0, screen_width());
                    blueorbs[i] = (startx, RandomRange::gen_range(0.0, screen_height()), startx, false);
                    score += 5;
                    play_sound_once(&point3);
                }
            }
            for i in 0..bomborbs.len() {
                let pos = bomborbs[i];
                if ((pos.0 - ball_x).abs() < 30.0) && ((pos.1 - ball_y).abs() < 30.0) {
                    let startx = RandomRange::gen_range(0.0, screen_width());
                    let starty = RandomRange::gen_range(0.0, screen_height());
                    bomborbs[i] = (startx, starty, startx, 0, starty, 1.0, 0);
                    score += 8;
                    play_sound_once(&point3);
                }
                if ((pos.0 - ball_x).abs() < 30.0) && ((pos.4 - ball_y).abs() < 30.0) {
                    score = 0;
                    alive = false;
                    play_sound_once(&explosion);
                }
            }

            ball_y += dy;
            ball_x += dx;
            dy += 0.15;

            if score % 10 == 0 && !milestones.contains(&score) {
                if score > 0 && score < 150 {
                    orbs.push((RandomRange::gen_range(0.0, screen_width()), RandomRange::gen_range(0.0, screen_height())));
                }
                if score >= 50 && score < 250 {
                    if score == 50{
                        play_sound_once(&levelup);
                    }
                    yelloworbs.push((RandomRange::gen_range(0.0, screen_width()), RandomRange::gen_range(0.0, screen_height())));
                }
                if score >= 100 && score < 350 {
                    if score == 100{
                        play_sound_once(&levelup);
                    }
                    let startx = RandomRange::gen_range(0.0, screen_width());
                    blueorbs.push((startx, RandomRange::gen_range(0.0, screen_height()), startx, false));
                }
                if score >= 150 {
                    if score == 150{
                        play_sound_once(&levelup);
                    }
                    let startx = RandomRange::gen_range(0.0, screen_width());
                    let starty = RandomRange::gen_range(0.0, screen_height());
                    bomborbs.push((startx, starty, startx, 0, starty, 1.0, 0));
                }
                milestones.push(score);
            }

            for i in 0..blueorbs.len() {
                let pos = blueorbs[i];
                if !pos.3 {
                    blueorbs[i] = (pos.0+2.0, pos.1, pos.2, pos.3);
                } else if pos.3 {
                    blueorbs[i] = (pos.0-2.0, pos.1, pos.2, pos.3);
                }
                if blueorbs[i].0 > pos.2 + 300.0 {
                    blueorbs[i] = (pos.0, pos.1, pos.2, true);
                } else if blueorbs[i].0 < pos.2 {
                    blueorbs[i] = (pos.0, pos.1, pos.2, false);
                }
            }

            for i in 0..bomborbs.len() {
                let pos = bomborbs[i];
                if pos.3 == 0 {
                    bomborbs[i] = (pos.0+2.0, pos.1, pos.2, pos.3, pos.4, pos.5, 0);
                } else if pos.3 == 1 {
                    bomborbs[i] = (pos.0-2.0, pos.1, pos.2, pos.3, pos.4, pos.5, 0);
                } else if pos.3 == 2 {
                    let pos = bomborbs[i];
                    draw_circle(pos.0, pos.4, 8.0, RED);
                    bomborbs[i] = (pos.0, pos.1, pos.2, 2, pos.4+pos.5, pos.5 + 0.1, pos.6);
                    let pos = bomborbs[i];
                    if pos.4 > screen_height() {
                        if pos.6 == 0 {
                            bomborbs[i] = (pos.0, pos.1, pos.2, 0, screen_height()+10.0, pos.5, pos.6);
                        } else if pos.6 == 1 {
                            bomborbs[i] = (pos.0, pos.1, pos.2, 1, screen_height()+10.0, pos.5, pos.6);
                        }
                    }
                }
                if bomborbs[i].0 > pos.2 + 300.0 {
                    bomborbs[i] = (pos.0, pos.1, pos.2, 2, pos.1, 1.0, 1);
                } else if bomborbs[i].0 < pos.2 {
                    bomborbs[i] = (pos.0, pos.1, pos.2, 2, pos.1, 1.0, 0);
                }
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

            if mobilemode {
                if is_mouse_button_down(MouseButton::Left) && rectjump.contains(mouse_position().into()) {
                    if ball_y + 20.0 >= screen_height() {
                        dy += -4.0
                    }
                }
                if is_mouse_button_down(MouseButton::Left) && rectleft.contains(mouse_position().into()) {
                    dx += -0.2;
                } else if is_mouse_button_down(MouseButton::Left) && rectright.contains(mouse_position().into()) {
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
            } else {
                if is_key_down(KeyCode::Space) || is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
                    if ball_y + 20.0 >= screen_height() {
                        dy += -4.0
                    }
                }
                if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
                    dx += -0.2;
                } else{
                    if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
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
            }
            if ball_x + 20.0 > screen_width() {
                ball_x = screen_width() - 20.0;
                dx = 0.0;
            }
            if ball_x - 20.0 < 0.0 {
                ball_x = 20.0;
                dx = 0.0;
            }
            if ball_y - 20.0 < 0.0 {
                ball_y = 20.0;
                dy = 0.0;
            }
            if ball_y + 20.0 > screen_height() {
                ball_y = screen_height() - 20.0
            }

            next_frame().await
        } else {
            let text_size = measure_text("GAME OVER", None, 100, 1.0);
            draw_text("GAME OVER", screen_width()/2.0-text_size.width/2.0, screen_height()/2.0-120.0, 100.0, WHITE);
            ui::root_ui().pop_skin();
            ui::root_ui().push_skin(&retry);
            let buttonretry = ui::root_ui().button(vec2(screen_width()/2.0-100.0, screen_height()/2.0-50.0), "");
            if buttonretry {
                orbs = Vec::new();
                yelloworbs = Vec::new();
                blueorbs = Vec::new();
                bomborbs = Vec::new();
                milestones = Vec::new();
                ball_x = 330.0;
                ball_y = 80.0;
                dy = 1.0;
                dx = 0.0;
                for _ in 0..5 {
                    orbs.push((RandomRange::gen_range(0.0, screen_width()), RandomRange::gen_range(0.0, screen_height())));
                }
                alive = true;
            }

            next_frame().await
        }
    }
}