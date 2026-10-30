// Define the environment constants
const BOX_WIDTH: f64 = 10.0;
const BOX_HEIGHT: f64 = 10.0;
const ENTITY_RADIUS: f64 = 0.2;
const CREEP_RADIUS: f64 = 0.15;
const BOUNCE_COEFFICIENT: f64 = 0.8;
const FRICTION: f64 = 0.99;

use rand::Rng;

// Define the entity state
#[derive(Clone, Copy)]
pub struct Entity {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
}

#[derive(Clone, Copy)]
pub struct Creep {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
}

// Define the simulation state
pub struct Simulation {
    pub entity: Entity,
    pub creeps: Vec<Creep>,
    pub paused: bool,
}

impl Creep {
    pub fn new_random() -> Self {
        let mut rng = rand::thread_rng();
        Creep {
            x: rng.gen_range(CREEP_RADIUS..BOX_WIDTH - CREEP_RADIUS),
            y: rng.gen_range(CREEP_RADIUS..BOX_HEIGHT - CREEP_RADIUS),
            vx: rng.gen_range(-0.2..0.2),
            vy: rng.gen_range(-0.2..0.2),
        }
    }
}

// Initialize the simulation
pub fn init_simulation() -> Simulation {
    let mut simulation = Simulation {
        entity: Entity {
            x: BOX_WIDTH / 2.0,
            y: BOX_HEIGHT / 2.0,
            vx: 0.0,
            vy: 0.0,
        },
        creeps: Vec::new(),
        paused: false,
    };

    // Add some creeps
    for _ in 0..5 {
        simulation.creeps.push(Creep::new_random());
    }

    simulation
}

// Update the simulation state
pub fn update_simulation(simulation: &mut Simulation) {
    if simulation.paused {
        return;
    }

    let entity = &mut simulation.entity;

    // Update player entity position based on velocity
    entity.x += entity.vx;
    entity.y += entity.vy;

    // Handle wall collisions for player - check if the edge of the circle hits the wall
    if entity.x - ENTITY_RADIUS <= 0.0 {
        entity.vx = entity.vx.abs() * BOUNCE_COEFFICIENT;
        entity.x = ENTITY_RADIUS;
    }
    if entity.x + ENTITY_RADIUS >= BOX_WIDTH {
        entity.vx = -entity.vx.abs() * BOUNCE_COEFFICIENT;
        entity.x = BOX_WIDTH - ENTITY_RADIUS;
    }
    if entity.y - ENTITY_RADIUS <= 0.0 {
        entity.vy = entity.vy.abs() * BOUNCE_COEFFICIENT;
        entity.y = ENTITY_RADIUS;
    }
    if entity.y + ENTITY_RADIUS >= BOX_HEIGHT {
        entity.vy = -entity.vy.abs() * BOUNCE_COEFFICIENT;
        entity.y = BOX_HEIGHT - ENTITY_RADIUS;
    }

    // Apply friction to player
    entity.vx *= FRICTION;
    entity.vy *= FRICTION;

    // Update creeps
    for creep in &mut simulation.creeps {
        // Creeps move with constant velocity (no friction)
        creep.x += creep.vx;
        creep.y += creep.vy;

        // Handle wall collisions for creeps - they bounce with full coefficient
        if creep.x - CREEP_RADIUS <= 0.0 {
            creep.vx = creep.vx.abs();
            creep.x = CREEP_RADIUS;
        }
        if creep.x + CREEP_RADIUS >= BOX_WIDTH {
            creep.vx = -creep.vx.abs();
            creep.x = BOX_WIDTH - CREEP_RADIUS;
        }
        if creep.y - CREEP_RADIUS <= 0.0 {
            creep.vy = creep.vy.abs();
            creep.y = CREEP_RADIUS;
        }
        if creep.y + CREEP_RADIUS >= BOX_HEIGHT {
            creep.vy = -creep.vy.abs();
            creep.y = BOX_HEIGHT - CREEP_RADIUS;
        }

        // Check collision with player
        let dx = entity.x - creep.x;
        let dy = entity.y - creep.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance < ENTITY_RADIUS + CREEP_RADIUS {
            // Collision! Respawn creep at random location
            let new_creep = Creep::new_random();
            *creep = new_creep;
        }
    }
}

// Handle keyboard input
pub fn handle_input(simulation: &mut Simulation, input: &str) {
    match input {
        "up" => simulation.entity.vy -= 0.1,
        "down" => simulation.entity.vy += 0.1,
        "left" => simulation.entity.vx -= 0.1,
        "right" => simulation.entity.vx += 0.1,
        "space" => simulation.paused = !simulation.paused,
        "escape" => std::process::exit(0),
        _ => (),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_simulation() {
        let sim = init_simulation();
        assert_eq!(sim.entity.x, BOX_WIDTH / 2.0);
        assert_eq!(sim.entity.y, BOX_HEIGHT / 2.0);
        assert_eq!(sim.entity.vx, 0.0);
        assert_eq!(sim.entity.vy, 0.0);
        assert!(!sim.paused);
        assert_eq!(sim.creeps.len(), 5);
    }

    #[test]
    fn test_update_simulation_no_collision() {
        let mut sim = init_simulation();
        sim.entity.vx = 1.0;
        sim.entity.vy = 1.0;
        sim.creeps.clear(); // Remove creeps for this test

        update_simulation(&mut sim);

        assert_eq!(sim.entity.x, BOX_WIDTH / 2.0 + 1.0);
        assert_eq!(sim.entity.y, BOX_HEIGHT / 2.0 + 1.0);
        assert_eq!(sim.entity.vx, 1.0 * FRICTION);
        assert_eq!(sim.entity.vy, 1.0 * FRICTION);
    }

    #[test]
    fn test_update_simulation_collision_right_wall() {
        let mut sim = init_simulation();
        sim.entity.x = BOX_WIDTH - ENTITY_RADIUS;
        sim.entity.vx = 1.0;
        sim.creeps.clear();

        update_simulation(&mut sim);

        assert!((sim.entity.x - (BOX_WIDTH - ENTITY_RADIUS)).abs() < 0.001);
        assert!(sim.entity.vx < 0.0);
    }

    #[test]
    fn test_creep_wall_bounce() {
        let mut sim = init_simulation();
        sim.creeps.clear();
        sim.creeps.push(Creep {
            x: BOX_WIDTH - CREEP_RADIUS,
            y: BOX_HEIGHT / 2.0,
            vx: 0.1,
            vy: 0.0,
        });

        update_simulation(&mut sim);

        assert!((sim.creeps[0].x - (BOX_WIDTH - CREEP_RADIUS)).abs() < 0.001);
        assert!(sim.creeps[0].vx < 0.0); // Bounced left
    }

    #[test]
    fn test_creep_constant_velocity() {
        let mut sim = init_simulation();
        sim.creeps.clear();
        let initial_vx = 0.3;
        let initial_vy = 0.4;
        sim.creeps.push(Creep {
            x: BOX_WIDTH / 2.0,
            y: BOX_HEIGHT / 2.0,
            vx: initial_vx,
            vy: initial_vy,
        });

        update_simulation(&mut sim);

        // Creeps should maintain their velocity (no friction)
        assert_eq!(sim.creeps[0].vx, initial_vx);
        assert_eq!(sim.creeps[0].vy, initial_vy);
    }

    #[test]
    fn test_handle_input() {
        let mut sim = init_simulation();

        handle_input(&mut sim, "up");
        assert_eq!(sim.entity.vy, -0.1);

        handle_input(&mut sim, "down");
        assert_eq!(sim.entity.vy, 0.0);

        handle_input(&mut sim, "left");
        assert_eq!(sim.entity.vx, -0.1);

        handle_input(&mut sim, "right");
        assert_eq!(sim.entity.vx, 0.0);

        handle_input(&mut sim, "space");
        assert!(sim.paused);

        handle_input(&mut sim, "space");
        assert!(!sim.paused);
    }

    #[test]
    fn test_corner_collision() {
        let mut sim = init_simulation();
        sim.entity.x = BOX_WIDTH - ENTITY_RADIUS - 0.01;
        sim.entity.y = BOX_HEIGHT - ENTITY_RADIUS - 0.01;
        sim.entity.vx = 0.5;
        sim.entity.vy = 0.5;
        sim.creeps.clear();

        update_simulation(&mut sim);

        assert!((sim.entity.x - (BOX_WIDTH - ENTITY_RADIUS)).abs() < 0.001);
        assert!((sim.entity.y - (BOX_HEIGHT - ENTITY_RADIUS)).abs() < 0.001);
        assert!(sim.entity.vx < 0.0);
        assert!(sim.entity.vy < 0.0);
    }

    #[test]
    fn test_high_velocity() {
        let mut sim = init_simulation();
        sim.entity.vx = 10.0;
        sim.entity.vy = 10.0;
        sim.creeps.clear();

        update_simulation(&mut sim);

        assert!(sim.entity.x > BOX_WIDTH / 2.0);
        assert!(sim.entity.y > BOX_HEIGHT / 2.0);
        assert!(sim.entity.vx < 10.0);
        assert!(sim.entity.vy < 10.0);
    }

    #[test]
    fn test_negative_velocity() {
        let mut sim = init_simulation();
        sim.entity.vx = -1.0;
        sim.entity.vy = -1.0;
        sim.creeps.clear();

        update_simulation(&mut sim);

        assert_eq!(sim.entity.x, BOX_WIDTH / 2.0 - 1.0);
        assert_eq!(sim.entity.y, BOX_HEIGHT / 2.0 - 1.0);
        assert_eq!(sim.entity.vx, -FRICTION);
        assert_eq!(sim.entity.vy, -FRICTION);
    }

    #[test]
    fn test_center_zero_velocity() {
        let mut sim = init_simulation();
        sim.creeps.clear();

        update_simulation(&mut sim);

        assert_eq!(sim.entity.x, BOX_WIDTH / 2.0);
        assert_eq!(sim.entity.y, BOX_HEIGHT / 2.0);
        assert_eq!(sim.entity.vx, 0.0);
        assert_eq!(sim.entity.vy, 0.0);
    }
}
