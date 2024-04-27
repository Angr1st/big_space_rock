use std::{ops::Mul, time::SystemTime};

use ::rand::Rng;
use macroquad::prelude::*;
use rand_xoshiro::{rand_core::SeedableRng, Xoshiro256PlusPlus, Xoshiro256StarStar};

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
struct Rock {
    position: Vec2,
    velocity: Vec2,
    rotation: f32,
    size: RockSize,
    seed: u64,
}

impl Default for Rock {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            velocity: Vec2::ZERO,
            rotation: 0.0,
            size: RockSize::Big,
            seed: 0,
        }
    }
}

enum RockSize {
    Big,
    Medium,
    Small,
}

impl RockSize {
    pub fn get_size(self: &Self) -> f32 {
        match self {
            RockSize::Big => SCALE * 3.0,
            RockSize::Medium => SCALE * 1.4,
            RockSize::Small => SCALE * 0.8,
        }
    }

    pub fn new(size: f32) -> Self {
        if size < 0.3 {
            RockSize::Small
        } else if size >= 0.3 && size < 0.59 {
            RockSize::Medium
        } else {
            RockSize::Big
        }
    }
}

impl Into<RockSize> for f32 {
    fn into(self) -> RockSize {
        RockSize::new(self)
    }
}

struct State {
    now: f32,
    delta: f32,
    ship: Ship,
    render_thruster_plume: bool,
    rocks: Vec<Rock>,
    random: Xoshiro256PlusPlus,
}

impl Default for State {
    fn default() -> Self {
        let seed = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("We should be after 1970")
            .as_secs();
        Self {
            now: 0.0,
            delta: 0.0,
            ship: Ship::default(),
            render_thruster_plume: false,
            rocks: vec![],
            random: Xoshiro256PlusPlus::seed_from_u64(seed),
        }
    }
}

fn update(state: &mut State) {
    // rotations / second
    const ROTATION_SPEED: f32 = 2.0;
    const SHIP_SPEED: f32 = 24.0;

    let keys = get_keys_down();
    if keys.contains(&KeyCode::A) {
        state.ship.rotation += state.delta * std::f32::consts::TAU * ROTATION_SPEED;
    }

    if keys.contains(&KeyCode::D) {
        state.ship.rotation -= state.delta * std::f32::consts::TAU * ROTATION_SPEED;
    }

    let corrected_ship_angle = state.ship.rotation + (std::f32::consts::PI * 0.5);
    let ship_direction: Vec2 = Vec2::from_angle(corrected_ship_angle);

    if keys.contains(&KeyCode::W) {
        state.ship.velocity = state.ship.velocity + (ship_direction * state.delta * SHIP_SPEED);
        state.render_thruster_plume = (((state.now.round() as i32) * 10) % 2) == 0;
    } else {
        state.render_thruster_plume = false;
    }
    const DRAG: f32 = 0.015;
    const DRAG_MINUS_ONE: f32 = 1.0 - DRAG;
    state.ship.velocity = state.ship.velocity * DRAG_MINUS_ONE;
    state.ship.position = state.ship.position + state.ship.velocity;
    state.ship.position = keep_in_frame(state.ship.position);

    for rock in state.rocks.iter_mut() {
        rock.position = rock.position + rock.velocity;
        rock.position = keep_in_frame(rock.position);
    }
}

fn keep_in_frame(vec: Vec2) -> Vec2 {
    let new_x = if vec.x <= 0.0 { SIZE.x } else { vec.x % SIZE.x };
    let new_y = if vec.y <= 0.0 { SIZE.y } else { vec.y % SIZE.y };
    // debug!("x:{}, y:{}", new_x, new_y);
    Vec2::new(new_x, new_y)
}

fn render(state: &State) {
    let ship_points = [
        Vec2::new(-0.4, -0.5),
        Vec2::new(0.0, 0.5),
        Vec2::new(0.4, -0.5),
        Vec2::new(0.3, -0.4),
        Vec2::new(-0.3, -0.4),
    ];
    draw_lines(
        state.ship.position,
        SCALE,
        state.ship.rotation,
        &ship_points,
    );
    if state.render_thruster_plume {
        let thruster_points = [
            Vec2::new(-0.3, -0.4),
            Vec2::new(0.0, -1.0),
            Vec2::new(0.3, -0.4),
        ];

        draw_lines(
            state.ship.position,
            SCALE,
            state.ship.rotation,
            &thruster_points,
        );
    }

    for rock in state.rocks.iter() {
        draw_space_rock(rock.position, &rock.size, rock.seed);
    }
}

fn init_level(state: &mut State) {
    for _ in 0..20 {
        let angle = std::f32::consts::TAU * state.random.gen::<f32>();
        let direction = Vec2::from_angle(angle);
        let rock = Rock {
            position: Vec2::new(
                state.random.gen::<f32>() * SIZE.x,
                state.random.gen::<f32>() * SIZE.y,
            ),
            velocity: direction * 3.0 * state.random.gen::<f32>(),
            size: state.random.gen::<f32>().into(),
            seed: state.random.gen::<u64>(),
            ..Default::default()
        };
        state.rocks.push(rock);
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // debug!("Helloaaaa, world!\n");
    let mut state = State::default();

    init_level(&mut state);

    loop {
        clear_background(BLACK);
        state.delta = get_frame_time();
        state.now += state.delta;

        update(&mut state);
        render(&state);
        next_frame().await;
    }
}

fn draw_space_rock(pos: Vec2, size: &RockSize, seed: u64) {
    let mut random = Xoshiro256StarStar::seed_from_u64(seed);
    let mut points: Vec<Vec2> = Vec::with_capacity(16);
    let n = random.gen_range(8..15);
    for i in 0..n {
        let mut radius = 0.3 + (0.2 * random.gen::<f32>());
        if random.gen::<f32>() < 0.2 {
            radius -= 0.2;
        }
        let angle = i as f32 * (std::f32::consts::TAU / n as f32)
            + (std::f32::consts::PI * 0.125 * random.gen::<f32>());
        let direction = Vec2::from_angle(angle);
        debug!("radius: {}, angle: {}", radius, angle);
        points.push(direction * radius);
    }
    debug!("points: {}", points.len());
    draw_lines(pos, size.get_size(), 0.0, &points);
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
