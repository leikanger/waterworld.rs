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
const OFFSET_X: f32 = SCREEN_WIDTH / 2.0;
const OFFSET_Y: f32 = SCREEN_HEIGHT / 2.0;

#[macroquad::main("NeoRL Simulator")]
async fn main() {
    let mut simulation = init_simulation();
    let mut graphics = SimulationGraphics::new();

    // Add the player unit (scaled to screen coordinates) - WHITE
    graphics.add_unit(
        (simulation.entity.x as f32 - 5.0) * ENTITY_SCALE + OFFSET_X,
        (simulation.entity.y as f32 - 5.0) * ENTITY_SCALE + OFFSET_Y,
        WHITE,
    );

    // Initialize creep graphics
    graphics.update_creeps(&simulation.creeps, CREEP_SCALE, OFFSET_X, OFFSET_Y);

    // Track if we need to trigger all green completed effect
    let mut trigger_all_green_effect = false;

    loop {
        // Get delta time for frame-independent updates
        let delta_time = get_frame_time();

        // after completion of all green: flash background for a timer.
        flash_bakground_white_if_timer(&mut graphics);

        // Read keys: in addition to channel input.
        handle_keyboard_intput(&mut simulation);

        // Update simulation - returns collision events
        let collision_events = update_simulation(&mut simulation);

        // Handle collision events for visual effects
        for event in &collision_events {
            graphics.handle_collision_event(event, CREEP_SCALE, OFFSET_X, OFFSET_Y);
        }

        // Check if we need to trigger all green completed effect
        if simulation.all_green_completed && !trigger_all_green_effect {
            graphics.trigger_all_green_completed();
            trigger_all_green_effect = true;
        }

        // Reset the flag after triggering
        if trigger_all_green_effect && graphics.get_reset_alpha() <= 0.0 {
            trigger_all_green_effect = false;
            simulation.all_green_completed = false;
        }

        // Update player graphics
        if let Some(unit) = graphics.units.first_mut() {
            unit.x = (simulation.entity.x as f32 - 5.0) * ENTITY_SCALE + OFFSET_X;
            unit.y = (simulation.entity.y as f32 - 5.0) * ENTITY_SCALE + OFFSET_Y;
        }

        // Update creep graphics
        graphics.update_creeps(&simulation.creeps, CREEP_SCALE, OFFSET_X, OFFSET_Y);

        // Update visual effects
        graphics.update(delta_time);

        // Draw everything
        graphics.draw();

        draw_red_green_count(&mut simulation);
        draw_controls_and_status(&mut simulation);

        draw_completion_message_when_green_reset(&mut graphics);

        // Draw box boundaries
        draw_rectangle_lines(
            (0.0 - 5.0) * SCALE + OFFSET_X,
            (0.0 - 5.0) * SCALE + OFFSET_Y,
            10.0 * SCALE,
            10.0 * SCALE,
            2.0,
            WHITE,
        );

        next_frame().await;
    }
}

fn draw_red_green_count(simulation: &mut Simulation) {
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
}
/// Draw information about simulation: controls and if simulation is paused.
fn draw_controls_and_status(simulation: &mut Simulation) {
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
}
/// Handle keyboard input for acceleration and pausing etc
fn handle_keyboard_intput(simulation: &mut Simulation) {
    // Handle input - WASD and Arrow keys for movement

    if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
        simulation.entity.accelerate_left();
    }
    if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
        simulation.entity.accelerate_right();
    }
    if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
        simulation.entity.accelerate_up();
    }
    if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
        simulation.entity.accelerate_down();
    }

    // Toggle pause with Space
    if is_key_pressed(KeyCode::Space) {
        simulation.paused = !simulation.paused;
    }

    // Quit with ESC or Q
    if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
        std::process::exit(0);
    }
}
/// Draw reset completion message after last green
fn draw_completion_message_when_green_reset(graphics: &mut graphics::SimulationGraphics) {
    let reset_alpha = graphics.get_reset_alpha() * 2.0;
    if reset_alpha > 0.0 {
        let message = "All Green Creeps Collected! Creeps Reset";
        let text_width = measure_text(message, None, 20, 1.0).width;
        draw_text(
            message,
            SCREEN_WIDTH / 2.0 - text_width / 2.0,
            SCREEN_HEIGHT / 2.0 - 50.0,
            20.0,
            Color {
                r: 0.0,
                g: 1.0,
                b: 0.0,
                a: reset_alpha,
            },
        );
    }
}
/// Special graphical feedback when all green are eaten: flash background white
fn flash_bakground_white_if_timer(graphics: &mut graphics::SimulationGraphics) {
    // Flash background white if flash timer is active
    let flash_intensity = graphics.get_flash_intensity();
    if flash_intensity > 0.0 {
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
}
