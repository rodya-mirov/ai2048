use burn::prelude::*;

use crate::game_structs::GameState;
use crate::game_structs::RngPlacement;
use crate::game_traits::FullGame;
use crate::model_structs::PolicyNet;
use crate::model_traits::Model;

pub struct Reward<const N: usize, B: Backend> {
    /// Game state that was acted on, in tensor form
    state: Tensor<B, 1>,
    /// Softmax of the forward pass of the state through the model
    /// TODO: investigate if we actually need this?
    outputs: Tensor<B, 1>,
    /// Including discounted future rewards, then normalized
    reward: f32,
}

fn mean_stddev(xs: &[f32]) -> (f32, f32) {
    let n = xs.len() as f32;
    let mean = xs.iter().sum::<f32>() / n;
    let var = xs.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / n;

    (mean, var.sqrt())
}

// Picked arbitrarily; probably parametrize this at some point
const DISCOUNT_FACTOR: f32 = 0.99;

// Used to prevent divide by zero when normalizing
const EPSILON: f32 = 0.0001;

#[allow(unused)] // TODO: obviously, use this
pub fn simulate_one_game<const N: usize, B: Backend>(model: &PolicyNet<N, B>) -> Vec<Reward<N, B>> {
    let mut prng = RngPlacement::new();
    let mut game_state = GameState::new_random(&mut prng);

    let mut rewards: Vec<Reward<N, B>> = Vec::new();

    while !game_state.is_finished() {
        let input_tensor = model.input_to_tensor(&game_state);
        let output_tensor = model.get_output_tensor(input_tensor.clone());
        let next_move = model.get_move_from_output(&game_state, output_tensor.clone());

        let new_game_state = game_state
            .apply_move(next_move, &mut prng)
            .expect("Should only generate valid moves");

        let reward = (new_game_state.current_score() - game_state.current_score()) as f32;

        rewards.push(Reward {
            state: input_tensor,
            outputs: output_tensor,
            reward,
        });

        game_state = new_game_state;
    }

    // Apply discounted rewards
    // TODO: unit test this section frfr; I'm pretty sure this is right but it seems too easy
    let mut running_reward = 0.;
    for reward in rewards.iter_mut().rev() {
        running_reward = (running_reward * DISCOUNT_FACTOR) + reward.reward;
        reward.reward = running_reward;
    }

    // Normalize rewards
    // TODO: unit test this section frfr; I'm pretty sure this is right but again it's typo fodder
    let mut rewards_vec: Vec<f32> = rewards.iter().map(|x| x.reward).collect();
    let (mean, stddev) = mean_stddev(&rewards_vec);
    for reward in rewards_vec.iter_mut() {
        *reward = (*reward - mean) / (stddev + EPSILON);
    }

    for (reward, normalized) in rewards.iter_mut().zip(rewards_vec.into_iter()) {
        reward.reward = normalized;
    }

    rewards
}
