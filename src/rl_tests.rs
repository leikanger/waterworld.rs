//! Tests for the RL interface

use crate::rl::{Action, Config, Env};

#[test]
fn test_reset_produces_observation() {
    let config = Config::default();
    let mut env = Env::new(config);
    let (obs, _info) = env.reset(None);

    // Observation should have entity (4) + creeps (5 * 5 = 25) = 29 elements
    assert_eq!(obs.len(), 29);
}

#[test]
fn test_step_increments_step_count() {
    let config = Config::default();
    let mut env = Env::new(config);
    env.reset(None);

    // Step count starts at 0 after reset
    // We can't access step_count directly, but we can verify truncation
}

#[test]
fn test_truncation_at_max_steps() {
    let config = Config {
        max_episode_steps: 10,
        dt: 0.01,
        frame_skip: 1,
        ..Config::default()
    };
    let mut env = Env::new(config);
    env.reset(None);

    // Step 9 times (should not be truncated)
    for _ in 0..9 {
        let result = env.step(Action::NoOp);
        assert!(!result.truncated);
    }

    // 10th step should be truncated
    let result = env.step(Action::NoOp);
    assert!(result.truncated);
}

#[test]
fn test_action_affects_velocity() {
    let config = Config::default();
    let mut env = Env::new(config);
    env.reset(None);

    let initial_vy = env.state().entity.vy;
    env.step(Action::Up);
    let new_vy = env.state().entity.vy;

    // Up action should decrease vy (accelerate upward)
    assert!(new_vy < initial_vy);
}

#[test]
fn custom_friction_survives_reset_and_applies_per_frame() {
    let mut env = Env::new(Config {
        friction: 0.5,
        frame_skip: 2,
        ..Config::default()
    });
    assert_eq!(env.state().friction, 0.5);

    for reset in [false, true] {
        if reset {
            // A reset restores the configured value, even if the state was changed.
            env.state_mut().friction = 0.1;
            env.reset(None);
        }
        assert_eq!(env.state().friction, 0.5);
        let state = env.state_mut();
        state.creeps.clear();
        state.entity.vx = 1.0;
        state.entity.vy = -0.5;
        env.step(Action::NoOp);
        assert_eq!(env.state().entity.vx, 0.25);
        assert_eq!(env.state().entity.vy, -0.125);
    }
}

#[test]
fn default_friction_is_unchanged() {
    assert_eq!(Config::default().friction, 0.99);
    assert_eq!(Env::default().state().friction, 0.99);
}

#[test]
fn env_rejects_invalid_configured_friction() {
    for friction in [-0.01, 1.01, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let result = std::panic::catch_unwind(|| {
            Env::new(Config {
                friction,
                ..Config::default()
            })
        });
        assert!(result.is_err());
    }
}

#[test]
fn env_rejects_invalid_mutated_friction_before_action() {
    let mut env = Env::default();
    env.state_mut().friction = 1.01;
    let initial_entity = env.state().entity;
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| env.step(Action::Up)));
    assert!(result.is_err());
    assert_eq!(env.state().entity, initial_entity);
}

#[test]
fn test_simulation_events_returned() {
    use crate::simulation::{Creep, CreepType, Entity, SimulationEvent};

    let config = Config::default();
    let mut env = Env::new(config);

    // Manually set up a collision scenario
    {
        let state = env.state_mut();
        // Place entity at origin
        state.entity = Entity {
            x: 1.0,
            y: 1.0,
            vx: 0.0,
            vy: 0.0,
        };
        // Place a creep at the same position to guarantee collision
        state.creeps.clear();
        state.creeps.push(Creep {
            x: 1.0,
            y: 1.0,
            vx: 0.0,
            vy: 0.0,
            creep_type: CreepType::Green,
        });
    }

    // Step should produce simulation events
    let result = env.step(Action::NoOp);

    // Should have exactly one collision event
    assert_eq!(
        result.simulation_events.len(),
        1,
        "Should have exactly one simulation event"
    );

    // Verify the event is a Collision with green creep
    match &result.simulation_events[0] {
        SimulationEvent::Collision { creep_type, .. } => {
            assert!(matches!(creep_type, CreepType::Green));
        }
        _ => panic!("Expected Collision event"),
    }
}

#[test]
fn test_all_green_completed_event() {
    use crate::simulation::{Creep, CreepType, Entity, SimulationEvent};

    let config = Config::default();
    let mut env = Env::new(config);

    // Set up a scenario where all creeps are red (no green)
    // This should trigger AllGreenCompleted event
    {
        let state = env.state_mut();
        state.entity = Entity {
            x: 5.0,
            y: 5.0,
            vx: 0.0,
            vy: 0.0,
        };
        // Replace all creeps with red ones
        state.creeps.clear();
        for _ in 0..5 {
            state.creeps.push(Creep {
                x: 1.0,
                y: 1.0,
                vx: 0.0,
                vy: 0.0,
                creep_type: CreepType::Red,
            });
        }
        // Mark that we've already completed green creeps
        // (This simulates the state where all green creeps were already collected)
        state.all_green_completed = false;
    }

    // Step should produce AllGreenCompleted event
    let result = env.step(Action::NoOp);

    // Should have AllGreenCompleted event
    assert!(
        result
            .simulation_events
            .iter()
            .any(|e| matches!(e, SimulationEvent::AllGreenCompleted)),
        "Should have AllGreenCompleted event"
    );
}
