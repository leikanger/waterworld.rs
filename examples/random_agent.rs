//! Example: Random agent
//!
//! Runs a random policy on the simulator and prints episode returns.

use neoRL_simulator::rl::{Action, Config, Env};
use rand::Rng;

fn main() {
    let config = Config {
        max_episode_steps: 1000,
        dt: 0.01,
        frame_skip: 1,
    };

    let mut env = Env::new(config);
    let mut rng = rand::thread_rng();

    let num_episodes = 5;
    let actions = [Action::NoOp, Action::Up, Action::Down, Action::Left, Action::Right];

    for episode in 0..num_episodes {
        let (_, _) = env.reset(None);
        let mut total_reward = 0.0f32;
        let mut step_count = 0;

        loop {
            // Random action
            let idx = rng.gen_range(0..actions.len());
            let action = actions[idx];

            let result = env.step(action);
            total_reward += result.reward;
            step_count += 1;

            if result.truncated {
                break;
            }
        }

        println!(
            "Episode {}: Return = {:.2}, Steps = {}",
            episode + 1,
            total_reward,
            step_count
        );
    }
}
