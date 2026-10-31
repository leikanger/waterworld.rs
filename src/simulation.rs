// Define the environment constants
const BOX_WIDTH: f64 = 10.0;
const BOX_HEIGHT: f64 = 10.0;
const ENTITY_RADIUS: f64 = 0.2;
const CREEP_RADIUS: f64 = 0.15;
const BOUNCE_COEFFICIENT: f64 = 0.8;
const FRICTION: f64 = 0.99;
const NUMBER_OF_CREEPS: u32 = 5;
const ACCELERATION: f64 = 0.02;

use rand::Rng;

// Define the entity state
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Entity {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
}

impl Entity {
    pub fn accelerate_up(&mut self) {
        self.vy -= ACCELERATION;
    }
    pub fn accelerate_down(&mut self) {
        self.vy += ACCELERATION;
    }

    pub fn accelerate_left(&mut self) {
        self.vx -= ACCELERATION;
    }
    pub fn accelerate_right(&mut self) {
        self.vx += ACCELERATION;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CreepType {
    Green, // +1.0 reward
    Red,   // -1.0 reward
}

#[derive(Clone, Copy, Debug)]
pub struct Creep {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub creep_type: CreepType,
}

// Collision event for graphics to handle visual effects
#[derive(Clone, Copy, Debug)]
pub struct CollisionEvent {
    pub x: f64,
    pub y: f64,
    pub creep_type: CreepType,
}

// Define the simulation state (physics only, no visual elements)
pub struct Simulation {
    pub entity: Entity,
    pub creeps: Vec<Creep>,
    pub score: u32,
    pub paused: bool,
    pub all_green_completed: bool, // Flag to track if all green creeps were collected
}

impl Default for Simulation {
    fn default() -> Self {
        init_simulation()
    }
}

impl Simulation {
    fn respawn_creeps(&mut self) {
        self.creeps.clear();
        // Add same number of creeps
        for _ in 0..NUMBER_OF_CREEPS {
            self.creeps.push(Creep::new_random());
        }
    }
}

impl Creep {
    pub fn new_random() -> Self {
        let mut rng = rand::thread_rng();
        let creep_type = if rng.gen_bool(0.5) {
            CreepType::Green
        } else {
            CreepType::Red
        };
        Creep {
            x: rng.gen_range(CREEP_RADIUS..BOX_WIDTH - CREEP_RADIUS),
            y: rng.gen_range(CREEP_RADIUS..BOX_HEIGHT - CREEP_RADIUS),
            vx: rng.gen_range(-0.05..0.05),
            vy: rng.gen_range(-0.05..0.05),
            creep_type,
        }
    }

    pub fn get_value(&self) -> f64 {
        match self.creep_type {
            CreepType::Green => 1.0,
            CreepType::Red => -1.0,
        }
    }

    #[allow(unused)]
    pub fn get_color(&self) -> ParticleColor {
        match self.creep_type {
            CreepType::Green => ParticleColor::Green,
            CreepType::Red => ParticleColor::Red,
        }
    }
}

// Color enum for visual representation (kept here for interface compatibility)
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(unused)] // allow colors that are unused
pub enum ParticleColor {
    Blue,
    Cyan,
    White,
    Green,
    Red,
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
        score: 0,
        paused: false,
        all_green_completed: false,
    };

    simulation.respawn_creeps();

    simulation
}

// Update the simulation state
pub fn update_simulation(simulation: &mut Simulation) -> Vec<CollisionEvent> {
    if simulation.paused {
        return Vec::new();
    }

    let mut collision_events = Vec::new();

    // Check if all green creeps are gone (only red or no creeps left)
    // Note: This logic only makes sense in the context of the game where
    // green creeps are being collected. For tests with only red creeps,
    // this won't trigger.
    if !simulation.all_green_completed {
        let has_green = simulation
            .creeps
            .iter()
            .any(|c| matches!(c.creep_type, CreepType::Green));
        // This assumes that at least one green creep was present initially
        if !has_green && !simulation.creeps.is_empty() {
            // All green creeps are gone - mark as completed
            // The actual reset will be handled by the graphics/UI layer
            simulation.all_green_completed = true;
            simulation.respawn_creeps();
        }
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
            // Collision! Record the event for graphics to handle
            let value = creep.get_value();
            collision_events.push(CollisionEvent {
                x: creep.x,
                y: creep.y,
                creep_type: creep.creep_type,
            });

            // Add the creep's value to the score
            simulation.score = (simulation.score as i64 + value as i64).max(0) as u32;

            // Replace with new random creep that's not too close to the player
            let mut new_creep = Creep::new_random();
            // Ensure the new creep doesn't spawn too close to the player
            let new_dx = entity.x - new_creep.x;
            let new_dy = entity.y - new_creep.y;
            let new_distance = (new_dx * new_dx + new_dy * new_dy).sqrt();

            // If too close, try to move it away
            if new_distance < ENTITY_RADIUS + CREEP_RADIUS + 0.5 {
                // Move the creep away from the player
                let angle = new_dx.atan2(new_dy);
                new_creep.x = entity.x + (ENTITY_RADIUS + CREEP_RADIUS + 1.0) * angle.cos();
                new_creep.y = entity.y + (ENTITY_RADIUS + CREEP_RADIUS + 1.0) * angle.sin();
                // Ensure it's still within bounds
                new_creep.x = new_creep.x.clamp(CREEP_RADIUS, BOX_WIDTH - CREEP_RADIUS);
                new_creep.y = new_creep.y.clamp(CREEP_RADIUS, BOX_HEIGHT - CREEP_RADIUS);
            }
            *creep = new_creep;
        }
    }

    collision_events
}

// Getters for simulation constants (useful for graphics scaling)
#[allow(unused)]
pub fn get_box_width() -> f64 {
    BOX_WIDTH
}

#[allow(unused)]
pub fn get_box_height() -> f64 {
    BOX_HEIGHT
}

#[allow(unused)]
pub fn get_entity_radius() -> f64 {
    ENTITY_RADIUS
}

#[allow(unused)]
pub fn get_creep_radius() -> f64 {
    CREEP_RADIUS
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
        assert_eq!(sim.score, 0);
        assert!(!sim.all_green_completed);
    }

    #[test]
    fn test_update_simulation_no_collision() {
        let mut sim = init_simulation();
        sim.entity.vx = 1.0;
        sim.entity.vy = 1.0;
        sim.creeps.clear(); // Remove creeps for this test

        let events = update_simulation(&mut sim);
        assert!(events.is_empty());

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

        let events = update_simulation(&mut sim);
        assert!(events.is_empty());

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
            creep_type: CreepType::Green,
        });

        let events = update_simulation(&mut sim);
        assert!(events.is_empty());

        assert!((sim.creeps[0].x - (BOX_WIDTH - CREEP_RADIUS)).abs() < 0.001);
        assert!(sim.creeps[0].vx < 0.0); // Bounced left
    }

    #[test]
    fn test_creep_constant_velocity() {
        let mut sim = Simulation {
            entity: Entity {
                x: 1.0,
                y: 1.0,
                vx: 0.0,
                vy: 0.0,
            },
            creeps: vec![
                Creep {
                    x: 8.0, // Far from entity at (1,1)
                    y: 8.0,
                    vx: 0.3,
                    vy: 0.4,
                    creep_type: CreepType::Red,
                },
                Creep {
                    x: 1.0,
                    y: 5.0,
                    vx: 0.0,
                    vy: 0.0,
                    creep_type: CreepType::Green, // Add a green creep to prevent all_green_completed trigger
                },
            ],
            score: 0,
            paused: false,
            all_green_completed: false,
        };

        let initial_vx = 0.3;
        let initial_vy = 0.4;

        let events = update_simulation(&mut sim);
        // Should have no collision events for the red creep
        dbg!(&events);
        assert!(events.is_empty());

        // Creeps should maintain their velocity (no friction)
        // Find the red creep (it should be at index 0 or 1 depending on order)
        let red_creep = sim
            .creeps
            .iter()
            .find(|c| matches!(c.creep_type, CreepType::Red))
            .unwrap();
        assert_eq!(red_creep.vx, initial_vx);
        assert_eq!(red_creep.vy, initial_vy);
    }

    #[test]
    fn test_corner_collision() {
        let mut sim = init_simulation();
        sim.entity.x = BOX_WIDTH - ENTITY_RADIUS - 0.01;
        sim.entity.y = BOX_HEIGHT - ENTITY_RADIUS - 0.01;
        sim.entity.vx = 0.5;
        sim.entity.vy = 0.5;
        sim.creeps.clear();

        let events = update_simulation(&mut sim);
        assert!(events.is_empty());

        // Should bounce off both walls
        assert!((sim.entity.x - (BOX_WIDTH - ENTITY_RADIUS)).abs() < 0.001);
        assert!((sim.entity.y - (BOX_HEIGHT - ENTITY_RADIUS)).abs() < 0.001);
        assert!(sim.entity.vx < 0.0);
        assert!(sim.entity.vy < 0.0);
    }

    #[test]
    fn test_creep_collision_with_player() {
        let mut sim = init_simulation();
        sim.entity.x = BOX_WIDTH / 2.0;
        sim.entity.y = BOX_HEIGHT / 2.0;
        sim.entity.vx = 0.0;
        sim.entity.vy = 0.0;

        sim.creeps.clear();
        sim.creeps.push(Creep {
            x: BOX_WIDTH / 2.0,
            y: BOX_HEIGHT / 2.0,
            vx: 0.0,
            vy: 0.0,
            creep_type: CreepType::Green,
        });

        let events = update_simulation(&mut sim);
        assert_eq!(events.len(), 1);
        assert_eq!(sim.score, 1);
    }

    #[test]
    fn test_red_creep_collision() {
        let mut sim = Simulation {
            entity: Entity {
                x: 5.0,
                y: 5.0,
                vx: 0.0,
                vy: 0.0,
            },
            creeps: vec![
                Creep {
                    x: 5.0, // Same position as entity
                    y: 5.0,
                    vx: 0.0,
                    vy: 0.0,
                    creep_type: CreepType::Red,
                },
                Creep {
                    x: 1.0,
                    y: 1.0,
                    vx: 0.0,
                    vy: 0.0,
                    creep_type: CreepType::Green, // Add a green creep to prevent all_green_completed trigger
                },
            ],
            score: 0,
            paused: false,
            all_green_completed: false,
        };

        let events = update_simulation(&mut sim);
        // Should have one collision event for the red creep
        assert_eq!(events.len(), 1);
        assert_eq!(sim.score, 0); // Score shouldn't go below 0
        // After collision, should still have 2 creeps (red was replaced)
        assert_eq!(sim.creeps.len(), 2);
    }

    #[test]
    fn test_all_green_completed_reset() {
        let mut sim = init_simulation();
        // Replace all creeps with red ones
        sim.creeps.clear();
        for _ in 0..NUMBER_OF_CREEPS {
            sim.creeps.push(Creep {
                x: 1.0,
                y: 1.0,
                vx: 0.0,
                vy: 0.0,
                creep_type: CreepType::Red,
            });
        }

        let events = update_simulation(&mut sim);
        assert!(events.is_empty());

        // Should trigger reset since no green creeps
        assert!(sim.all_green_completed);
        assert_eq!(sim.creeps.len(), 5); // Reset to 3 creeps
    }

    #[test]
    fn test_getters() {
        assert_eq!(get_box_width(), BOX_WIDTH);
        assert_eq!(get_box_height(), BOX_HEIGHT);
        assert_eq!(get_entity_radius(), ENTITY_RADIUS);
        assert_eq!(get_creep_radius(), CREEP_RADIUS);
    }

    #[test]
    fn test_creep_get_color() {
        let green_creep = Creep {
            x: 0.0,
            y: 0.0,
            vx: 0.0,
            vy: 0.0,
            creep_type: CreepType::Green,
        };
        let red_creep = Creep {
            x: 0.0,
            y: 0.0,
            vx: 0.0,
            vy: 0.0,
            creep_type: CreepType::Red,
        };

        assert_eq!(green_creep.get_color(), ParticleColor::Green);
        assert_eq!(red_creep.get_color(), ParticleColor::Red);
    }
}
