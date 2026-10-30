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

    // Add the player unit (scaled to screen coordinates)
    graphics.add_unit(
        (simulation.entity.x as f32 - 5.0) * ENTITY_SCALE + SCREEN_WIDTH / 2.0,
        (simulation.entity.y as f32 - 5.0) * ENTITY_SCALE + SCREEN_HEIGHT / 2.0,
        RED,
    );

    // Add creeps
    for creep in &simulation.creeps {
        graphics.add_creep(
            (creep.x as f32 - 5.0) * CREEP_SCALE + SCREEN_WIDTH / 2.0,
            (creep.y as f32 - 5.0) * CREEP_SCALE + SCREEN_HEIGHT / 2.0,
            BLUE,
        );
    }

    loop {
        clear_background(BLACK);

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
            graphics.add_creep(
                (creep.x as f32 - 5.0) * CREEP_SCALE + SCREEN_WIDTH / 2.0,
                (creep.y as f32 - 5.0) * CREEP_SCALE + SCREEN_HEIGHT / 2.0,
                BLUE,
            );
        }

        // Draw everything
        graphics.draw();

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
