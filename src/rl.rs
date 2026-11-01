//! RL Environment Interface
//!
//! Provides a Gymnasium-style interface for the simulator.

use crate::simulation::*;

/// Discrete actions for the agent
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    NoOp,
    Up,
    Down,
    Left,
    Right,
}

impl Action {
    /// Apply this action to an entity
    pub fn apply(&self, entity: &mut Entity) {
        match self {
            Action::NoOp => {}
            Action::Up => entity.accelerate_up(),
            Action::Down => entity.accelerate_down(),
            Action::Left => entity.accelerate_left(),
            Action::Right => entity.accelerate_right(),
        }
    }
}

/// Observation as a flat vector of f32
pub type Observation = Vec<f32>;

/// Result of a step
#[derive(Debug, Clone)]
pub struct StepResult {
    pub observation: Observation,
    pub reward: f32,
    pub terminated: bool,
    pub truncated: bool,
    pub info: std::collections::HashMap<String, String>,
    pub simulation_events: Vec<SimulationEvent>,
}

/// Configuration for the RL environment
#[derive(Debug, Clone)]
pub struct Config {
    pub max_episode_steps: usize,
    pub dt: f64,
    pub frame_skip: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_episode_steps: 1000,
            dt: 0.01,
            frame_skip: 1,
        }
    }
}

/// RL Environment
pub struct Env {
    simulation: Simulation,
    config: Config,
    step_count: usize,
    last_score: u32,
}

impl Env {
    /// Create a new environment
    pub fn new(config: Config) -> Self {
        Self {
            simulation: Simulation::default(),
            config,
            step_count: 0,
            last_score: 0,
        }
    }

    /// Reset the environment
    pub fn reset(&mut self, _seed: Option<u64>) -> (Observation, std::collections::HashMap<String, String>) {
        self.simulation = Simulation::default();
        self.step_count = 0;
        self.last_score = self.simulation.score;
        let obs = self.get_observation();
        let mut info = std::collections::HashMap::new();
        info.insert("episode_restarted".to_string(), "true".to_string());
        (obs, info)
    }

    /// Step the environment
    pub fn step(&mut self, action: Action) -> StepResult {
        // Apply action
        action.apply(&mut self.simulation.entity);

        // Step physics
        let mut total_reward = 0.0f32;
        let mut all_simulation_events = Vec::new();

        for _ in 0..self.config.frame_skip {
            let simulation_events = update_simulation(&mut self.simulation);

            // Calculate reward from collision events
            for event in &simulation_events {
                if let SimulationEvent::Collision { creep_type, .. } = event {
                    match creep_type {
                        CreepType::Green => total_reward += 1.0,
                        CreepType::Red => total_reward -= 1.0,
                    }
                }
            }

            // Collect all simulation events across frame skips
            all_simulation_events.extend(simulation_events);
        }

        // Small time penalty
        total_reward -= 0.01;

        self.step_count += 1;

        // Check truncation
        let truncated = self.step_count >= self.config.max_episode_steps;

        let observation = self.get_observation();
        let info = std::collections::HashMap::new();

        // Update last score
        self.last_score = self.simulation.score;

        StepResult {
            observation,
            reward: total_reward,
            terminated: false, // Never terminate, only truncate
            truncated,
            info,
            simulation_events: all_simulation_events,
        }
    }

    /// Get current observation
    fn get_observation(&self) -> Observation {
        let mut obs = Vec::new();

        // Entity state: x, y, vx, vy
        obs.push(self.simulation.entity.x as f32);
        obs.push(self.simulation.entity.y as f32);
        obs.push(self.simulation.entity.vx as f32);
        obs.push(self.simulation.entity.vy as f32);

        // Creep states: x, y, vx, vy, type (1.0 for Green, -1.0 for Red)
        for creep in &self.simulation.creeps {
            obs.push(creep.x as f32);
            obs.push(creep.y as f32);
            obs.push(creep.vx as f32);
            obs.push(creep.vy as f32);
            obs.push(match creep.creep_type {
                CreepType::Green => 1.0,
                CreepType::Red => -1.0,
            });
        }

        obs
    }

    /// Get the simulation state (for rendering)
    pub fn state(&self) -> &Simulation {
        &self.simulation
    }

    /// Get mutable simulation state
    pub fn state_mut(&mut self) -> &mut Simulation {
        &mut self.simulation
    }
}

impl Default for Env {
    fn default() -> Self {
        Self::new(Config::default())
    }
}
