use macroquad::prelude::*;

mod simulation;
use simulation::*;

mod graphics;
use graphics::*;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const SCALE: f32 = 50.0;
const ENTITY_SCALE: f32 = 50.0;
const CREEP_SCALE: f32 = 50.0;

#[macroquad::main("NeoRL Simulator")]
async fn main() {
    let mut simulation = init_simulation();
    let mut graphics = SimulationGraphics::new();

    // Add the player unit (scaled to screen coordinates) - WHITE
    graphics.add_unit(
        (simulation.entity.x as f32 - 5.0) * ENTITY_SCALE + SCREEN_WIDTH / 2.0,
        (simulation.entity.y as f32 - 5.0) * ENTITY_SCALE + SCREEN_HEIGHT / 2.0,
        WHITE,
    );

    // Add creeps
    for creep in &simulation.creeps {
        let creep_color = match creep.creep_type {
            simulation::CreepType::Green => GREEN,
            simulation::CreepType::Red => RED,
        };
        graphics.add_creep(
            (creep.x as f32 - 5.0) * CREEP_SCALE + SCREEN_WIDTH / 2.0,
            (creep.y as f32 - 5.0) * CREEP_SCALE + SCREEN_HEIGHT / 2.0,
            creep_color,
        );
    }

    loop {
        // Flash background white if flash timer is active
        if simulation.flash_timer > 0.0 {
            // Alternate between white and black for flashing effect
            let flash_intensity = (simulation.flash_timer * 10.0).sin().abs() as f32;
            let bg_color = Color {
                r: flash_intensity,
                g: flash_intensity,
                b: flash_intensity,
                a: 1.0,
            };
            clear_background(bg_color);
        } else {
            clear_background(BLACK);
        }

        // Handle input - WASD and Arrow keys for movement
        let mut input_vx = 0.0;
        let mut input_vy = 0.0;

        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            input_vx -= 0.02;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            input_vx += 0.02;
        }
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            input_vy -= 0.02;
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            input_vy += 0.02;
        }

        // Apply input acceleration to velocity
        simulation.entity.vx += input_vx;
        simulation.entity.vy += input_vy;

        // Toggle pause with Space
        if is_key_pressed(KeyCode::Space) {
            simulation.paused = !simulation.paused;
        }

        // Quit with ESC or Q
        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
            std::process::exit(0);
        }

        // Update simulation
        update_simulation(&mut simulation);

        // Update player graphics
        if let Some(unit) = graphics.units.first_mut() {
            unit.x = (simulation.entity.x as f32 - 5.0) * ENTITY_SCALE + SCREEN_WIDTH / 2.0;
            unit.y = (simulation.entity.y as f32 - 5.0) * ENTITY_SCALE + SCREEN_HEIGHT / 2.0;
        }

        // Update creep graphics
        graphics.clear_creeps();
        for creep in &simulation.creeps {
            // Get base color based on creep type
            let base_color = match creep.creep_type {
                simulation::CreepType::Green => GREEN,
                simulation::CreepType::Red => RED,
            };

            // Add pulsing effect for recently hit creeps
            let mut creep_color = base_color;
            if creep.hit_timer > 0.0 {
                // Pulse between the base color and white based on hit timer
                let pulse_factor = (creep.hit_timer * 5.0).sin().abs();
                let white_factor = pulse_factor as f32;
                creep_color = Color {
                    r: base_color.r * (1.0 - white_factor) + WHITE.r * white_factor,
                    g: base_color.g * (1.0 - white_factor) + WHITE.g * white_factor,
                    b: base_color.b * (1.0 - white_factor) + WHITE.b * white_factor,
                    a: 1.0,
                };
            }

            graphics.add_creep(
                (creep.x as f32 - 5.0) * CREEP_SCALE + SCREEN_WIDTH / 2.0,
                (creep.y as f32 - 5.0) * CREEP_SCALE + SCREEN_HEIGHT / 2.0,
                creep_color,
            );
        }

        // Update particle graphics
        graphics.clear_particles();
        for particle in &simulation.particles {
            // Calculate alpha based on particle life (fade out effect)
            let alpha = (particle.life / particle.max_life) as f32;
            let size = 3.0 * (1.0 - alpha * 0.7); // Particles shrink as they fade

            let color = match particle.color {
                simulation::ParticleColor::Blue => BLUE,
                simulation::ParticleColor::Cyan => Color::new(0.0, 1.0, 1.0, 1.0), // Cyan
                simulation::ParticleColor::White => WHITE,
                simulation::ParticleColor::Green => GREEN,
                simulation::ParticleColor::Red => RED,
            };

            // Apply alpha to the color
            let particle_color = Color {
                r: color.r,
                g: color.g,
                b: color.b,
                a: alpha,
            };

            graphics.add_particle(
                (particle.x as f32 - 5.0) * CREEP_SCALE + SCREEN_WIDTH / 2.0,
                (particle.y as f32 - 5.0) * CREEP_SCALE + SCREEN_HEIGHT / 2.0,
                size,
                particle_color,
            );
        }

        // Update sound wave graphics
        graphics.clear_sound_waves();
        for wave in &simulation.sound_waves {
            // Calculate alpha based on wave life (fade out effect)
            let alpha = (wave.life / wave.max_life) as f32;
            let wave_color = Color {
                r: 0.0,
                g: 1.0,
                b: 1.0,
                a: alpha * 0.5, // Semi-transparent cyan
            };

            graphics.add_sound_wave(
                (wave.x as f32 - 5.0) * CREEP_SCALE + SCREEN_WIDTH / 2.0,
                (wave.y as f32 - 5.0) * CREEP_SCALE + SCREEN_HEIGHT / 2.0,
                (wave.radius as f32) * CREEP_SCALE,
                wave_color,
            );
        }

        // Draw everything
        graphics.draw();

        // Draw creep count with types
        let green_count = simulation
            .creeps
            .iter()
            .filter(|c| matches!(c.creep_type, simulation::CreepType::Green))
            .count();
        let red_count = simulation
            .creeps
            .iter()
            .filter(|c| matches!(c.creep_type, simulation::CreepType::Red))
            .count();
        draw_text(
            format!("Green creeps: {} (+1.0)", green_count).as_str(),
            20.0,
            140.0,
            20.0,
            GREEN,
        );
        draw_text(
            format!("Red creeps: {} (-1.0)", red_count).as_str(),
            20.0,
            160.0,
            20.0,
            RED,
        );

        // Draw controls and status
        draw_text("Controls:", 20.0, 20.0, 20.0, WHITE);
        draw_text("Arrow keys / WASD: Move", 20.0, 40.0, 20.0, WHITE);
        draw_text("Space: Pause/Resume", 20.0, 60.0, 20.0, WHITE);
        draw_text("ESC or Q: Quit", 20.0, 80.0, 20.0, WHITE);
        draw_text(
            format!("Creeps: {}", simulation.creeps.len()).as_str(),
            20.0,
            100.0,
            20.0,
            WHITE,
        );
        draw_text(
            format!("Score: {}", simulation.score).as_str(),
            20.0,
            120.0,
            20.0,
            Color::new(0.0, 1.0, 1.0, 1.0), // Cyan for score
        );

        // Draw pause status - big and clear
        if simulation.paused {
            draw_text(
                "SIMULATION FROZEN - Press SPACE to resume",
                SCREEN_WIDTH / 2.0 - 170.0,
                SCREEN_HEIGHT / 2.0,
                20.0,
                YELLOW,
            );
        }

        // Draw reset completion message
        if simulation.reset_timer > 0.0 {
            let alpha = (simulation.reset_timer / 2.0).min(1.0) as f32;
            let message = "All Green Creeps Collected! Creeps Reset";
            let text_width = measure_text(message, None, 20, 1.0).width;
            draw_text(
                message,
                SCREEN_WIDTH / 2.0 - text_width / 2.0,
                SCREEN_HEIGHT / 2.0 - 50.0,
                30.0,
                Color {
                    r: 0.0,
                    g: 1.0,
                    b: 0.0,
                    a: alpha,
                },
            );
        }

        // Draw box boundaries
        draw_rectangle_lines(
            (0.0 - 5.0) * SCALE + SCREEN_WIDTH / 2.0,
            (0.0 - 5.0) * SCALE + SCREEN_HEIGHT / 2.0,
            10.0 * SCALE,
            10.0 * SCALE,
            2.0,
            WHITE,
        );

        next_frame().await;
    }
}
