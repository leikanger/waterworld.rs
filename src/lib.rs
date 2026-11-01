//! NeoRL Simulator Library
//!
//! A simple 2D navigation simulator for reinforcement learning.

pub mod rl;
pub mod simulation;

pub use rl::{Action, Config, Env, Observation, StepResult};
pub use simulation::{Creep, CreepType, Entity, Simulation, update_simulation};

#[cfg(test)]
mod rl_tests;
