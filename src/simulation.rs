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

#[derive(Clone, Copy, PartialEq)]
pub enum CreepType {
    Green, // +1.0 reward
    Red,   // -1.0 reward
}

#[derive(Clone, Copy)]
pub struct Creep {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub hit_timer: f64, // Timer for hit effect visualization
    pub creep_type: CreepType,
}

// Particle for explosion effects
#[derive(Clone, Copy)]
pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub life: f64,
    pub max_life: f64,
    pub color: ParticleColor,
}

#[derive(Clone, Copy)]
pub struct SoundWave {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub life: f64,
    pub max_life: f64,
}

#[derive(Clone, Copy)]
pub enum ParticleColor {
    #[allow(dead_code)]
    Blue,
    Cyan,
    White,
    Green,
    Red,
}

// Define the simulation state
pub struct Simulation {
    pub entity: Entity,
    pub creeps: Vec<Creep>,
    pub particles: Vec<Particle>,
    pub sound_waves: Vec<SoundWave>,
    pub score: u32,
    pub paused: bool,
    pub flash_timer: f64,          // Timer for background flash effect
    pub reset_timer: f64,          // Timer for showing reset message
    pub all_green_completed: bool, // Flag to track if all green creeps were collected
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
            hit_timer: 0.0,
            creep_type,
        }
    }

    pub fn get_value(&self) -> f64 {
        match self.creep_type {
            CreepType::Green => 1.0,
            CreepType::Red => -1.0,
        }
    }

    pub fn get_color(&self) -> ParticleColor {
        match self.creep_type {
            CreepType::Green => ParticleColor::Green,
            CreepType::Red => ParticleColor::Red,
        }
    }
}

impl Particle {
    #[allow(dead_code)]
    pub fn new_explosion(x: f64, y: f64) -> Vec<Self> {
        let mut rng = rand::thread_rng();
        let mut particles = Vec::new();

        // Create 15-25 particles per explosion
        let particle_count = rng.gen_range(15..25);

        for _ in 0..particle_count {
            let angle = rng.gen_range(0.0..std::f64::consts::TAU);
            let speed = rng.gen_range(0.3..0.8);
            let vx = angle.cos() * speed;
            let vy = angle.sin() * speed;
            let life = rng.gen_range(0.5..1.5);

            // Random color: mostly blue with some cyan and white for visual interest
            let color = match rng.gen_range(0..10) {
                0..=6 => ParticleColor::Blue,
                7..=8 => ParticleColor::Cyan,
                _ => ParticleColor::White,
            };

            particles.push(Particle {
                x,
                y,
                vx,
                vy,
                life,
                max_life: life,
                color,
            });
        }

        particles
    }

    pub fn new_explosion_with_color(x: f64, y: f64, color: ParticleColor) -> Vec<Self> {
        let mut rng = rand::thread_rng();
        let mut particles = Vec::new();

        // Create 15-25 particles per explosion
        let particle_count = rng.gen_range(15..25);

        for _ in 0..particle_count {
            let angle = rng.gen_range(0.0..std::f64::consts::TAU);
            let speed = rng.gen_range(0.3..0.8);
            let vx = angle.cos() * speed;
            let vy = angle.sin() * speed;
            let life = rng.gen_range(0.5..1.5);

            // Use the specified color, with some variation for visual interest
            let particle_color = match rng.gen_range(0..10) {
                0..=7 => color,
                8 => ParticleColor::White, // Some white particles for highlight
                _ => ParticleColor::Cyan,  // Some cyan particles for variation
            };

            particles.push(Particle {
                x,
                y,
                vx,
                vy,
                life,
                max_life: life,
                color: particle_color,
            });
        }

        particles
    }
}

impl SoundWave {
    pub fn new(x: f64, y: f64) -> Self {
        SoundWave {
            x,
            y,
            radius: 0.0,
            life: 1.0,
            max_life: 1.0,
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
        particles: Vec::new(),
        sound_waves: Vec::new(),
        score: 0,
        paused: false,
        flash_timer: 0.0,
        reset_timer: 0.0,
        all_green_completed: false,
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

    // Handle flash and reset timers
    if simulation.flash_timer > 0.0 {
        simulation.flash_timer -= 0.016;
        // When flash timer expires, trigger reset
        if simulation.flash_timer <= 0.0 && !simulation.all_green_completed {
            // Reset all creeps
            simulation.creeps.clear();
            for _ in 0..5 {
                simulation.creeps.push(Creep::new_random());
            }
            simulation.all_green_completed = true;
            simulation.reset_timer = 2.0; // Show message for 2 seconds
        }
    }

    if simulation.reset_timer > 0.0 {
        simulation.reset_timer -= 0.016;
        if simulation.reset_timer <= 0.0 {
            simulation.all_green_completed = false;
        }
    }

    // Check if all green creeps are gone (only red or no creeps left)
    if simulation.flash_timer <= 0.0 && simulation.reset_timer <= 0.0 && !simulation.all_green_completed {
        let has_green = simulation
            .creeps
            .iter()
            .any(|c| matches!(c.creep_type, CreepType::Green));
        if !has_green && !simulation.creeps.is_empty() {
            // All green creeps are gone - trigger flash
            simulation.flash_timer = 0.5; // Flash for 0.5 seconds
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

    // Update particles - they move and fade out
    simulation.particles.retain_mut(|particle| {
        particle.x += particle.vx;
        particle.y += particle.vy;
        particle.life -= 0.036; // Decrease life based on frame time (assuming ~60fps)
        particle.life > 0.0
    });

    // Update sound waves - they expand and fade out
    simulation.sound_waves.retain_mut(|wave| {
        wave.radius += 0.5; // Expand the wave
        wave.life -= 0.05;
        wave.life > 0.0
    });

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

        // Update hit timer for visual effects
        if creep.hit_timer > 0.0 {
            creep.hit_timer -= 0.016; // Decrease timer based on frame time
        }

        // Check collision with player
        let dx = entity.x - creep.x;
        let dy = entity.y - creep.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance < ENTITY_RADIUS + CREEP_RADIUS {
            // Collision! Create explosion particles with the creep's color
            let creep_color = creep.get_color();
            let explosion_particles = Particle::new_explosion_with_color(creep.x, creep.y, creep_color);
            simulation.particles.extend(explosion_particles);

            // Create sound wave effect
            simulation
                .sound_waves
                .push(SoundWave::new(creep.x, creep.y));

            // Add the creep's value to the score
            let value = creep.get_value();
            // Convert f64 to i64 for score (since score is u32, we need to handle negative values)
            simulation.score = (simulation.score as i64 + value as i64).max(0) as u32;

            // Set hit timer for visual effect on the new creep
            let mut new_creep = Creep::new_random();
            new_creep.hit_timer = 1.0; // 1 second of pulsing effect
            *creep = new_creep;
        }
    }
}

// Handle keyboard input
#[allow(dead_code)]
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
        assert_eq!(sim.score, 0);
        assert!(sim.particles.is_empty());
        assert!(sim.sound_waves.is_empty());
        assert_eq!(sim.flash_timer, 0.0);
        assert_eq!(sim.reset_timer, 0.0);
        assert!(!sim.all_green_completed);
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
            hit_timer: 0.0,
            creep_type: CreepType::Green,
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
            hit_timer: 0.0,
            creep_type: CreepType::Red,
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

    #[test]
    fn test_creep_values() {
        let green_creep = Creep {
            x: 1.0,
            y: 1.0,
            vx: 0.0,
            vy: 0.0,
            hit_timer: 0.0,
            creep_type: CreepType::Green,
        };
        let red_creep = Creep {
            x: 1.0,
            y: 1.0,
            vx: 0.0,
            vy: 0.0,
            hit_timer: 0.0,
            creep_type: CreepType::Red,
        };

        // Test that green creep has value +1.0
        assert_eq!(green_creep.get_value(), 1.0);

        // Test that red creep has value -1.0
        assert_eq!(red_creep.get_value(), -1.0);
    }

    #[test]
    fn test_creep_collision_with_green() {
        let mut sim = init_simulation();
        sim.creeps.clear();
        sim.creeps.push(Creep {
            x: BOX_WIDTH / 2.0,
            y: BOX_HEIGHT / 2.0,
            vx: 0.0,
            vy: 0.0,
            hit_timer: 0.0,
            creep_type: CreepType::Green,
        });
        sim.entity.x = BOX_WIDTH / 2.0;
        sim.entity.y = BOX_HEIGHT / 2.0;

        update_simulation(&mut sim);

        // Score should increase by 1 for green creep
        assert_eq!(sim.score, 1);
    }

    #[test]
    fn test_creep_collision_with_red() {
        let mut sim = init_simulation();
        sim.creeps.clear();
        sim.creeps.push(Creep {
            x: BOX_WIDTH / 2.0,
            y: BOX_HEIGHT / 2.0,
            vx: 0.0,
            vy: 0.0,
            hit_timer: 0.0,
            creep_type: CreepType::Red,
        });
        sim.entity.x = BOX_WIDTH / 2.0;
        sim.entity.y = BOX_HEIGHT / 2.0;

        update_simulation(&mut sim);

        // Score should stay at 0 (can't go negative)
        assert_eq!(sim.score, 0);
    }

    #[test]
    fn test_creep_color_mapping() {
        let green_creep = Creep {
            x: 1.0,
            y: 1.0,
            vx: 0.0,
            vy: 0.0,
            hit_timer: 0.0,
            creep_type: CreepType::Green,
        };
        let red_creep = Creep {
            x: 1.0,
            y: 1.0,
            vx: 0.0,
            vy: 0.0,
            hit_timer: 0.0,
            creep_type: CreepType::Red,
        };

        assert!(matches!(green_creep.get_color(), ParticleColor::Green));
        assert!(matches!(red_creep.get_color(), ParticleColor::Red));
    }

    #[test]
    fn test_all_green_completed_trigger() {
        let mut sim = init_simulation();
        // Set up simulation with only red creeps
        sim.creeps.clear();
        sim.creeps.push(Creep {
            x: 1.0,
            y: 1.0,
            vx: 0.0,
            vy: 0.0,
            hit_timer: 0.0,
            creep_type: CreepType::Red,
        });
        sim.creeps.push(Creep {
            x: 2.0,
            y: 2.0,
            vx: 0.0,
            vy: 0.0,
            hit_timer: 0.0,
            creep_type: CreepType::Red,
        });

        // Run one frame - should trigger flash since no green creeps
        update_simulation(&mut sim);

        // Flash timer should be active
        assert!(sim.flash_timer > 0.0);
    }

    #[test]
    fn test_all_green_completed_with_green_present() {
        let mut sim = init_simulation();
        // Set up simulation with at least one green creep
        sim.creeps.clear();
        sim.creeps.push(Creep {
            x: 1.0,
            y: 1.0,
            vx: 0.0,
            vy: 0.0,
            hit_timer: 0.0,
            creep_type: CreepType::Green,
        });
        sim.creeps.push(Creep {
            x: 2.0,
            y: 2.0,
            vx: 0.0,
            vy: 0.0,
            hit_timer: 0.0,
            creep_type: CreepType::Red,
        });

        // Run one frame - should NOT trigger flash since green creep exists
        update_simulation(&mut sim);

        // Flash timer should remain at 0
        assert_eq!(sim.flash_timer, 0.0);
    }
}
