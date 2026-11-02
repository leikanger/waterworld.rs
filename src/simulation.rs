use crate::graphics;

// Define the environment constants
const BOX_WIDTH: f64 = 10.0;
const BOX_HEIGHT: f64 = 10.0;
// const ENTITY_SIZE: f64 = 0.5;
const BOUNCE_COEFFICIENT: f64 = 0.8;
const FRICTION: f64 = 0.99;

// Define the entity state
pub struct Entity {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
}

// Define the simulation state
pub struct Simulation {
    pub entity: Entity,
    pub paused: bool,
    pub graphics: graphics::SimulationGraphics,
}

// Initialize the simulation
pub fn init_simulation() -> Simulation {
    Simulation {
        entity: Entity {
            x: BOX_WIDTH / 2.0,
            y: BOX_HEIGHT / 2.0,
            vx: 0.0,
            vy: 0.0,
        },
        paused: false,
        graphics: graphics::SimulationGraphics::new(),
    }
}

// Update the simulation state
pub fn update_simulation(simulation: &mut Simulation) {
    if simulation.paused {
        return;
    }

    let entity = &mut simulation.entity;

    // Update position based on velocity
    entity.x += entity.vx;
    entity.y += entity.vy;

    // Handle wall collisions
    if entity.x <= 0.0 || entity.x >= BOX_WIDTH {
        entity.vx = -entity.vx * BOUNCE_COEFFICIENT;
        entity.x = entity.x.clamp(0.0, BOX_WIDTH);
    }
    if entity.y <= 0.0 || entity.y >= BOX_HEIGHT {
        entity.vy = -entity.vy * BOUNCE_COEFFICIENT;
        entity.y = entity.y.clamp(0.0, BOX_HEIGHT);
    }

    // Apply friction
    entity.vx *= FRICTION;
    entity.vy *= FRICTION;

    // Update graphics
    simulation.graphics.update_graphics();
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
    }

    #[test]
    fn test_update_simulation_no_collision() {
        let mut sim = init_simulation();
        sim.entity.vx = 1.0;
        sim.entity.vy = 1.0;

        update_simulation(&mut sim);

        assert_eq!(sim.entity.x, BOX_WIDTH / 2.0 + 1.0);
        assert_eq!(sim.entity.y, BOX_HEIGHT / 2.0 + 1.0);
        assert_eq!(sim.entity.vx, 1.0 * FRICTION);
        assert_eq!(sim.entity.vy, 1.0 * FRICTION);
    }

    #[test]
    fn test_update_simulation_collision() {
        let mut sim = init_simulation();
        sim.entity.x = BOX_WIDTH - 0.1;
        sim.entity.vx = 1.0;

        update_simulation(&mut sim);

        assert_eq!(sim.entity.x, BOX_WIDTH);
        assert_eq!(sim.entity.vx, -BOUNCE_COEFFICIENT * FRICTION);
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

    // Test for edge case: entity at the corner
    #[test]
    fn test_corner_collision() {
        let mut sim = init_simulation();
        sim.entity.x = BOX_WIDTH - 0.1;
        sim.entity.y = BOX_HEIGHT - 0.1;
        sim.entity.vx = 1.0;
        sim.entity.vy = 1.0;

        update_simulation(&mut sim);

        assert_eq!(sim.entity.x, BOX_WIDTH);
        assert_eq!(sim.entity.y, BOX_HEIGHT);
        assert_eq!(sim.entity.vx, -BOUNCE_COEFFICIENT * FRICTION);
        assert_eq!(sim.entity.vy, -BOUNCE_COEFFICIENT * FRICTION);
    }

    // Test for edge case: very high velocity
    #[test]
    fn test_high_velocity() {
        let mut sim = init_simulation();
        sim.entity.vx = 10.0;
        sim.entity.vy = 10.0;

        update_simulation(&mut sim);

        assert!(sim.entity.x > BOX_WIDTH / 2.0);
        assert!(sim.entity.y > BOX_HEIGHT / 2.0);
        assert!(sim.entity.vx < 10.0);
        assert!(sim.entity.vy < 10.0);
    }

    // Test for edge case: negative velocity
    #[test]
    fn test_negative_velocity() {
        let mut sim = init_simulation();
        sim.entity.vx = -1.0;
        sim.entity.vy = -1.0;

        update_simulation(&mut sim);

        assert_eq!(sim.entity.x, BOX_WIDTH / 2.0 - 1.0);
        assert_eq!(sim.entity.y, BOX_HEIGHT / 2.0 - 1.0);
        assert_eq!(sim.entity.vx, -FRICTION);
        assert_eq!(sim.entity.vy, -FRICTION);
    }

    // Test for edge case: entity at the center with zero velocity
    #[test]
    fn test_center_zero_velocity() {
        let mut sim = init_simulation();

        update_simulation(&mut sim);

        assert_eq!(sim.entity.x, BOX_WIDTH / 2.0);
        assert_eq!(sim.entity.y, BOX_HEIGHT / 2.0);
        assert_eq!(sim.entity.vx, 0.0);
        assert_eq!(sim.entity.vy, 0.0);
    }
}
