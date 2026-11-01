use std::{thread::sleep, time::Duration};

use macroquad::prelude::*;

mod rl;
mod simulation;

use rl::{Action, Config, Env};
use simulation::*;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const SCALE: f32 = 50.0;
const ENTITY_SCALE: f32 = 50.0;
const CREEP_SCALE: f32 = 50.0;
const OFFSET_X: f32 = SCREEN_WIDTH / 2.0;
const OFFSET_Y: f32 = SCREEN_HEIGHT / 2.0;

mod graphics;
use graphics::*;

#[macroquad::main("NeoRL Simulator")]
async fn main() {
    let config = Config {
        max_episode_steps: 10000,
        dt: 0.01,
        frame_skip: 1,
    };

    let mut env = Env::new(config);
    let (_, _) = env.reset(None);
    let mut graphics = SimulationGraphics::new();

    // Add the player unit
    graphics.add_unit(
        (env.state().entity.x as f32 - 5.0) * ENTITY_SCALE + OFFSET_X,
        (env.state().entity.y as f32 - 5.0) * ENTITY_SCALE + OFFSET_Y,
        WHITE,
    );

    // Initialize creep graphics
    graphics.update_creeps(&env.state().creeps, CREEP_SCALE, OFFSET_X, OFFSET_Y);

    // Track if we need to trigger all green completed effect
    let mut trigger_all_green_effect = false;

    const DELTA_TIME: f64 = 0.01;
    loop {
        let start_time: f64 = get_time();

        let action = handle_keyboard_intput();

        // Step the environment
        let result = env.step(action);

        // Handle simulation events for visual effects
        for event in &result.simulation_events {
            match event {
                SimulationEvent::Collision { x, y, creep_type } => {
                    // Create a temporary CollisionEvent for the graphics handler
                    let collision_event = CollisionEvent {
                        x: *x,
                        y: *y,
                        creep_type: *creep_type,
                    };
                    graphics.handle_collision_event(&collision_event, CREEP_SCALE, OFFSET_X, OFFSET_Y);
                }
                SimulationEvent::AllGreenCompleted => {
                    graphics.trigger_all_green_completed();
                    trigger_all_green_effect = true;
                }
            }
        }

        // Reset the flag after triggering
        if trigger_all_green_effect && graphics.get_reset_alpha() <= 0.0 {
            trigger_all_green_effect = false;
            env.state_mut().all_green_completed = false;
        }

        // Update player graphics
        if let Some(unit) = graphics.units.first_mut() {
            unit.x = (env.state().entity.x as f32 - 5.0) * ENTITY_SCALE + OFFSET_X;
            unit.y = (env.state().entity.y as f32 - 5.0) * ENTITY_SCALE + OFFSET_Y;
        }

        // Update creep graphics
        graphics.update_creeps(&env.state().creeps, CREEP_SCALE, OFFSET_X, OFFSET_Y);

        // Update visual effects
        graphics.update(DELTA_TIME as f32);
        graphics.flash_bakground_white_if_timer();

        // Draw everything
        graphics.draw();

        draw_controls_and_status(env.state_mut());
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

        // wait for set DELTA_TIME
        let remaining_time = DELTA_TIME - (get_time() - start_time);
        if remaining_time > 0.0 {
            sleep(Duration::from_secs_f64(remaining_time));
        }

        // Check for truncation (episode end)
        if result.truncated {
            let (_, _) = env.reset(None);
        }
    }
}

fn draw_controls_and_status(simulation: &mut Simulation) {
    draw_text("Controls:", 20.0, 20.0, 20.0, WHITE);
    draw_text("Arrow keys / WASD: Move", 20.0, 40.0, 20.0, WHITE);
    draw_text("Space: Pause", 20.0, 60.0, 20.0, WHITE);
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
        Color::new(0.0, 1.0, 1.0, 1.0),
    );

    if simulation.paused {
        draw_text(
            "SIMULATION PAUSED - Press SPACE to resume",
            SCREEN_WIDTH / 2.0 - 170.0,
            SCREEN_HEIGHT / 2.0,
            20.0,
            YELLOW,
        );
    }
}

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
                r: 0.8,
                g: 0.8,
                b: 0.8,
                a: reset_alpha,
            },
        );
    }
}

#[allow(unused)] // Not called: I find it too crowded in the GUI.
fn draw_red_green_count(simulation: &mut Simulation) {
    // Draw creep count with types
    let green_count = simulation
        .creeps
        .iter()
        .filter(|c| matches!(c.creep_type, CreepType::Green))
        .count();
    let red_count = simulation
        .creeps
        .iter()
        .filter(|c| matches!(c.creep_type, CreepType::Red))
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
/// Handle keyboard input for acceleration and pausing etc
fn handle_keyboard_intput() -> Action {
    // Handle input - WASD and Arrow keys for movement

    // Handle keyboard input
    let mut action = Action::NoOp;
    if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
        action = Action::Left;
    }
    if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
        action = Action::Right;
    }
    if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
        action = Action::Up;
    }
    if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
        action = Action::Down;
    }

    /* Toggle pause with Space
    if is_key_pressed(KeyCode::Space) {
        simulation.paused = !simulation.paused;
    }*/

    // Quit with ESC or Q
    if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
        std::process::exit(0);
    }

    action
}
