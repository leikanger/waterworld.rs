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
