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

#[derive(Clone, Copy)]
struct DeathTime {
    death_timer: f32,
    death_time: f32,
}

#[derive(Clone, Copy)]
enum ShipStatus {
    Alive,
    Dead(DeathTime),
}

impl Into<bool> for ShipStatus {
    fn into(self) -> bool {
        match self {
            ShipStatus::Alive => true,
            _ => false,
        }
    }
}

struct Ship {
    position: Vec2,
    velocity: Vec2,
    rotation: f32,
    status: ShipStatus,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            position: SIZE.mul(0.5),
            velocity: Vec2::ZERO,
            rotation: 0.0,
            status: ShipStatus::Alive,
        }
    }
}
struct Rock {
    position: Vec2,
    velocity: Vec2,
    rotation: f32,
    size: RockSize,
    seed: u64,
    removed: bool,
}

impl Default for Rock {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            velocity: Vec2::ZERO,
            rotation: 0.0,
            size: RockSize::Big,
            seed: 0,
            removed: false,
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

    pub fn get_collision_scale(self: &Self) -> f32 {
        match self {
            RockSize::Big => 0.4,
            RockSize::Medium => 0.65,
            RockSize::Small => 1.0,
        }
    }

    pub fn get_velocity(self: &Self) -> f32 {
        match self {
            RockSize::Big => 0.75,
            RockSize::Medium => 1.0,
            RockSize::Small => 1.6,
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
    particles: Vec<Particle>,
    projectiles: Vec<Projectile>,
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
            particles: vec![],
            projectiles: vec![],
            random: Xoshiro256PlusPlus::seed_from_u64(seed),
        }
    }
}

struct LineParticle {
    rotation: f32,
    length: f32,
}

impl LineParticle {
    pub fn new(rotation: f32, length: f32) -> Self {
        Self { rotation, length }
    }
}

impl Into<ParticleType> for LineParticle {
    fn into(self) -> ParticleType {
        ParticleType::Line(self)
    }
}

struct DotParticle {
    radius: f32,
}

impl DotParticle {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

impl Into<ParticleType> for DotParticle {
    fn into(self) -> ParticleType {
        ParticleType::Dot(self)
    }
}

enum ParticleType {
    Line(LineParticle),
    Dot(DotParticle),
}

struct Particle {
    position: Vec2,
    velocity: Vec2,
    time_to_live: f32,
    particle_type: ParticleType,
}

struct Projectile {
    position: Vec2,
    velocity: Vec2,
    time_to_live: f32,
}

fn update(state: &mut State) {
    if Into::<bool>::into(state.ship.status) {
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

        if keys.contains(&KeyCode::Space) {
            let position = state.ship.position + (ship_direction * (SCALE * 0.55));
            let velocity = ship_direction * 10.0;
            let projetile = Projectile {
                position,
                velocity,
                time_to_live: 1.0,
            };
            state.projectiles.push(projetile);
        }
    }

    for rock in state.rocks.iter_mut() {
        rock.position = rock.position + rock.velocity;
        rock.position = keep_in_frame(rock.position);

        if Into::<bool>::into(state.ship.status)
            && Vec2::distance(rock.position, state.ship.position)
                < rock.size.get_size() * rock.size.get_collision_scale()
        {
            // debug!("You died!");
            state.ship.status = ShipStatus::Dead(DeathTime {
                death_timer: state.now + 3.0,
                death_time: state.now,
            });

            for _ in 0..5 {
                let angle = std::f32::consts::TAU * state.random.gen::<f32>();
                let direction = Vec2::from_angle(angle);
                let position = state.ship.position
                    + Vec2::new(state.random.gen::<f32>(), state.random.gen::<f32>());
                let velocity = direction * 2.0 * state.random.gen::<f32>();
                let time_to_live = 3.0 + state.random.gen::<f32>();
                let line_particle = LineParticle::new(
                    std::f32::consts::TAU * state.random.gen::<f32>(),
                    SCALE * (0.6 + (0.4 * state.random.gen::<f32>())),
                );
                let particle = Particle {
                    position,
                    velocity,
                    time_to_live,
                    particle_type: line_particle.into(),
                };
                state.particles.push(particle);
            }
        }
    }

    for particle in state.particles.iter_mut() {
        particle.position = particle.position + particle.velocity;
        particle.position = keep_in_frame(particle.position);
        particle.time_to_live -= state.delta;
    }

    for projectile in state.projectiles.iter_mut() {
        projectile.position = projectile.position + projectile.velocity;
        projectile.position = keep_in_frame(projectile.position);
        projectile.time_to_live -= state.delta;
    }

    state.rocks.retain(|rock| !rock.removed);
    state
        .particles
        .retain(|particle| particle.time_to_live > 0.0);
    state
        .projectiles
        .retain(|projectile| projectile.time_to_live > 0.0);

    if let ShipStatus::Dead(value) = state.ship.status {
        // debug!("We dead!");
        if state.now > value.death_timer {
            reset_level(state);
        }
    }
}

fn keep_in_frame(vec: Vec2) -> Vec2 {
    let new_x = if vec.x <= 0.0 { SIZE.x } else { vec.x % SIZE.x };
    let new_y = if vec.y <= 0.0 { SIZE.y } else { vec.y % SIZE.y };
    // debug!("x:{}, y:{}", new_x, new_y);
    Vec2::new(new_x, new_y)
}

fn render(state: &State) {
    if Into::<bool>::into(state.ship.status) {
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
    }

    for rock in state.rocks.iter() {
        draw_space_rock(rock.position, &rock.size, rock.seed);
    }

    let line_points = [Vec2::new(-0.5, 0.0), Vec2::new(0.5, 0.0)];

    for particle in state.particles.iter() {
        match &particle.particle_type {
            ParticleType::Line(line) => {
                draw_lines(particle.position, line.length, line.rotation, &line_points)
            }
            ParticleType::Dot(dot) => {
                draw_circle_vec2(particle.position, dot.radius, THICKNESS, LINE_COLOR)
            }
        };
    }

    for projectile in state.projectiles.iter() {
        draw_circle_vec2(
            projectile.position,
            (SCALE * 0.05).max(1.0),
            THICKNESS,
            LINE_COLOR,
        )
    }
}

fn reset_asteroids(state: &mut State) {
    if !state.rocks.is_empty() {
        state.rocks.clear();
    }

    for _ in 0..20 {
        let angle = std::f32::consts::TAU * state.random.gen::<f32>();
        let direction = Vec2::from_angle(angle);
        let rock_size: RockSize = state.random.gen::<f32>().into();
        let rock = Rock {
            position: Vec2::new(
                state.random.gen::<f32>() * SIZE.x,
                state.random.gen::<f32>() * SIZE.y,
            ),
            velocity: direction * 3.0 * state.random.gen::<f32>() * rock_size.get_velocity(),
            size: rock_size,
            seed: state.random.gen::<u64>(),
            ..Default::default()
        };
        state.rocks.push(rock);
    }
}

fn reset_level(state: &mut State) {
    state.ship = Ship::default();
}

#[macroquad::main(window_conf)]
async fn main() {
    // debug!("Helloaaaa, world!\n");
    let mut state = State::default();

    reset_level(&mut state);
    reset_asteroids(&mut state);

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
        // debug!("radius: {}, angle: {}", radius, angle);
        points.push(direction * radius);
    }
    // debug!("points: {}", points.len());
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

fn draw_circle_vec2(pos: Vec2, radius: f32, thickness: f32, color: Color) {
    draw_circle_lines(pos.x, pos.y, radius, thickness, color);
}

fn draw_line_vec2(pos1: Vec2, pos2: Vec2, thickness: f32, color: Color) {
    draw_line(pos1.x, pos1.y, pos2.x, pos2.y, thickness, color);
}
