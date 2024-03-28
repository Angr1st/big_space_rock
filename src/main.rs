use macroquad::{miniquad::window, prelude::*};

const THICKNESS: f32 = 3.0;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("BIG SPACE ROCKS"),
        window_width: 1280,
        window_height: 960,
        window_resizable: false,
        ..Default::default()
    }
}

struct State {
    ship_position: Vec2,
}

#[macroquad::main(window_conf)]
async fn main() {
    debug!("Helloaaaa, world!\n");

    loop {
        clear_background(BLACK);

        let a = Vec2::splat(10.0);
        let b = Vec2::splat(100.0);

        draw_line_vec2(a, b, THICKNESS, WHITE);

        next_frame().await;
    }
}
fn draw_ship(pos: Vec2) {}

fn draw_line_vec2(pos1: Vec2, pos2: Vec2, thickness: f32, color: Color) {
    draw_line(pos1.x, pos1.y, pos2.x, pos2.y, thickness, color);
}
