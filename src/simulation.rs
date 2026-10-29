// Define the environment constants
const BOX_WIDTH: f64 = 10.0;
const BOX_HEIGHT: f64 = 10.0;
const ENTITY_RADIUS: f64 = 0.2; // Circle radius in simulation units (10px / 50 scale)
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

    // Handle wall collisions - check if the edge of the circle hits the wall
    // Left wall: entity.x - radius <= 0
    // Right wall: entity.x + radius >= BOX_WIDTH
    // Top wall: entity.y - radius <= 0
    // Bottom wall: entity.y + radius >= BOX_HEIGHT
    
    if entity.x - ENTITY_RADIUS <= 0.0 {
        entity.vx = entity.vx.abs() * BOUNCE_COEFFICIENT; // Bounce right
        entity.x = ENTITY_RADIUS; // Position at wall + radius
    }
    if entity.x + ENTITY_RADIUS >= BOX_WIDTH {
        entity.vx = -entity.vx.abs() * BOUNCE_COEFFICIENT; // Bounce left
        entity.x = BOX_WIDTH - ENTITY_RADIUS; // Position at wall - radius
    }
    if entity.y - ENTITY_RADIUS <= 0.0 {
        entity.vy = entity.vy.abs() * BOUNCE_COEFFICIENT; // Bounce down
        entity.y = ENTITY_RADIUS; // Position at wall + radius
    }
    if entity.y + ENTITY_RADIUS >= BOX_HEIGHT {
        entity.vy = -entity.vy.abs() * BOUNCE_COEFFICIENT; // Bounce up
        entity.y = BOX_HEIGHT - ENTITY_RADIUS; // Position at wall - radius
    }

    // Apply friction
    entity.vx *= FRICTION;
    entity.vy *= FRICTION;
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
    fn test_update_simulation_collision_right_wall() {
        let mut sim = init_simulation();
        // Position entity so its right edge is at the wall
        sim.entity.x = BOX_WIDTH - ENTITY_RADIUS;
        sim.entity.vx = 1.0;

        update_simulation(&mut sim);

        // Should bounce and be positioned at wall - radius
        assert!((sim.entity.x - (BOX_WIDTH - ENTITY_RADIUS)).abs() < 0.001);
        assert!(sim.entity.vx < 0.0); // Moving left after bounce
    }

    #[test]
    fn test_update_simulation_collision_left_wall() {
        let mut sim = init_simulation();
        // Position entity so its left edge is at the wall
        sim.entity.x = ENTITY_RADIUS;
        sim.entity.vx = -1.0;

        update_simulation(&mut sim);

        // Should bounce and be positioned at wall + radius
        assert!((sim.entity.x - ENTITY_RADIUS).abs() < 0.001);
        assert!(sim.entity.vx > 0.0); // Moving right after bounce
    }

    #[test]
    fn test_update_simulation_collision_top_wall() {
        let mut sim = init_simulation();
        sim.entity.y = ENTITY_RADIUS;
        sim.entity.vy = -1.0;

        update_simulation(&mut sim);

        assert!((sim.entity.y - ENTITY_RADIUS).abs() < 0.001);
        assert!(sim.entity.vy > 0.0); // Moving down after bounce
    }

    #[test]
    fn test_update_simulation_collision_bottom_wall() {
        let mut sim = init_simulation();
        sim.entity.y = BOX_HEIGHT - ENTITY_RADIUS;
        sim.entity.vy = 1.0;

        update_simulation(&mut sim);

        assert!((sim.entity.y - (BOX_HEIGHT - ENTITY_RADIUS)).abs() < 0.001);
        assert!(sim.entity.vy < 0.0); // Moving up after bounce
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

        update_simulation(&mut sim);

        // Should hit both right and bottom walls
        assert!((sim.entity.x - (BOX_WIDTH - ENTITY_RADIUS)).abs() < 0.001);
        assert!((sim.entity.y - (BOX_HEIGHT - ENTITY_RADIUS)).abs() < 0.001);
        assert!(sim.entity.vx < 0.0); // Bouncing left
        assert!(sim.entity.vy < 0.0); // Bouncing up
    }

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
