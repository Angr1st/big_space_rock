use macroquad::prelude::*;

const THICKNESS: f32 = 3.0;
const LINE_COLOR: Color = WHITE;

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

        let points = [
            Vec2::new(-0.4, -0.5),
            Vec2::new(0.0, 0.5),
            Vec2::new(0.4, -0.5),
            Vec2::new(0.3, -0.4),
            Vec2::new(-0.3, -0.4),
        ];
        draw_lines(Vec2::splat(40.0), 16.0, &points);

        next_frame().await;
    }
}

fn draw_lines(origin: Vec2, scale: f32, points: &[Vec2]) {
    let apply = |p: Vec2| (p * scale) + origin;

    let length = points.len();
    for i in 0..=length - 1 {
        let pos1 = points.get(i).unwrap();
        let pos2 = points.get(i + 1 % length).unwrap();
        draw_line_vec2(apply(*pos1), apply(*pos2), THICKNESS, LINE_COLOR);
    }
}

//fn draw_ship(pos: Vec2) {}

fn draw_line_vec2(pos1: Vec2, pos2: Vec2, thickness: f32, color: Color) {
    draw_line(pos1.x, pos1.y, pos2.x, pos2.y, thickness, color);
}
