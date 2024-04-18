use std::ops::Mul;

use macroquad::prelude::*;

const THICKNESS: f32 = 2.5;
const SCALE: f32 = 38.0;
const LINE_COLOR: Color = WHITE;
const WIDTH: i32 = 1280;
const HEIGHT: i32 = 960;
const SIZE: Vec2 = Vec2::new(WIDTH as f32, HEIGHT as f32);

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("BIG SPACE ROCKS"),
        window_width: WIDTH,
        window_height: HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}

struct Ship {
    position: Vec2,
    velocity: Vec2,
    rotation: f32,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            position: SIZE.mul(0.5),
            velocity: Vec2::ZERO,
            rotation: 0.0,
        }
    }
}

struct State {
    now: f32,
    delta: f32,
    ship: Ship,
}

impl Default for State {
    fn default() -> Self {
        Self {
            now: 0.0,
            delta: 0.0,
            ship: Ship::default(),
        }
    }
}

fn update(state: &mut State) {
    // rotations / second
    const ROTATION_SPEED: f32 = 2.0;
    const SHIP_SPEED: f32 = 32.0;

    let keys = get_keys_down();
    if keys.contains(&KeyCode::A) {
        state.ship.rotation += state.delta * std::f32::consts::TAU * ROTATION_SPEED;
    }

    if keys.contains(&KeyCode::D) {
        state.ship.rotation -= state.delta * std::f32::consts::TAU * ROTATION_SPEED;
    }

    let ship_direction: Vec2 = Vec2::from_angle(state.ship.rotation);

    if keys.contains(&KeyCode::W) {
        state.ship.velocity = state.ship.velocity + (ship_direction * state.delta * SHIP_SPEED);
    }

    const DRAG: f32 = 0.03;
    const DRAG_MINUS_ONE: f32 = 1.0 - DRAG;
    state.ship.velocity = state.ship.velocity * DRAG_MINUS_ONE;
    state.ship.position = state.ship.position + state.ship.velocity;
}

fn render(state: &State) {
    let points = [
        Vec2::new(-0.4, -0.5),
        Vec2::new(0.0, 0.5),
        Vec2::new(0.4, -0.5),
        Vec2::new(0.3, -0.4),
        Vec2::new(-0.3, -0.4),
    ];
    draw_lines(state.ship.position, SCALE, state.ship.rotation, &points);
}

#[macroquad::main(window_conf)]
async fn main() {
    // debug!("Helloaaaa, world!\n");
    let mut state = State::default();

    loop {
        clear_background(BLACK);
        state.delta = get_frame_time();
        state.now += state.delta;

        update(&mut state);
        render(&state);
        next_frame().await;
    }
}

fn draw_lines(origin: Vec2, scale: f32, rotation: f32, points: &[Vec2]) {
    let rotation_vec = Vec2::from_angle(rotation);
    let apply = |p: Vec2| (p.rotate(rotation_vec) * scale) + origin;

    let length = points.len();
    for i in 0..=length - 1 {
        let wrap = (i + 1) % length;
        //debug!("i {}, wrap: {}", i, wrap);
        let pos1 = points.get(i).unwrap();
        let pos2 = points.get(wrap).unwrap();
        draw_line_vec2(apply(*pos1), apply(*pos2), THICKNESS, LINE_COLOR);
    }
}

//fn draw_ship(pos: Vec2) {}

fn draw_line_vec2(pos1: Vec2, pos2: Vec2, thickness: f32, color: Color) {
    draw_line(pos1.x, pos1.y, pos2.x, pos2.y, thickness, color);
}
