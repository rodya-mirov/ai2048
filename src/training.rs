use std::time::Instant;

use burn::backend::Autodiff;
use burn::backend::NdArray;
use burn::optim::AdamConfig;
use burn::optim::GradientsParams;
use burn::optim::Optimizer;
use burn::optim::decay::WeightDecayConfig;
use burn::prelude::*;
use burn::tensor::Tensor;
use burn::tensor::activation::log_softmax;

use crate::game_structs::GameState;
use crate::game_structs::Move;
use crate::game_structs::RngPlacement;
use crate::game_traits::FullGame;
use crate::model_structs::InnerModel;
use crate::model_structs::PolicyNet;
use crate::model_traits::Model;

pub struct Reward<const N: usize, B: Backend> {
    /// Game state that was acted on, in tensor form
    state: Tensor<B, 1>,
    /// Chosen move from the model
    output: Move,
    /// Including discounted future rewards, then normalized
    reward: f32,
}

fn mean_stddev(xs: &[f32]) -> (f32, f32) {
    let n = xs.len() as f32;
    let mean = xs.iter().sum::<f32>() / n;
    let var = xs.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / n;

    (mean, var.sqrt())
}

// Used to prevent divide by zero when normalizing
const EPSILON: f32 = 0.0001;

type AD = Autodiff<NdArray<f32>>;

struct BatchifyResult {
    x: Tensor<AD, 2>,            // game states
    returns: Tensor<AD, 1>,      // discounted, normalized rewards
    actions: Tensor<AD, 1, Int>, // actions (outputs taken)
}

/// Batch the recorded steps into tensors
fn batchify<const N: usize>(batch: &[Reward<N, AD>], device: &<AD as Backend>::Device) -> BatchifyResult {
    let b = batch.len();

    let mut xs = Vec::with_capacity(b);
    let mut returns: Vec<f32> = Vec::with_capacity(b);
    let mut actions: Vec<i32> = Vec::with_capacity(b);

    for step in batch {
        xs.push(step.state.clone());
        returns.push(step.reward);
        actions.push(step.output.to_idx() as i32);
    }

    let x = Tensor::stack(xs, 0);
    let actions = Tensor::<AD, 1, Int>::from_ints(actions.as_slice(), device);
    let returns = Tensor::<AD, 1>::from_floats(returns.as_slice(), device);

    BatchifyResult { x, returns, actions }
}

pub fn train<const N: usize>(
    model: &mut PolicyNet<N, AD>,
    num_batches: usize,
    lr: f64,
    games_per_batch: usize,
    learning_steps_per_batch: usize,
    discount_factor: f32,
    l2_reg: f32,
) {
    let device = <AD as Backend>::Device::default();
    let mut opt = AdamConfig::new()
        .with_weight_decay(Some(WeightDecayConfig::new(l2_reg)))
        .init::<AD, InnerModel<AD>>();

    let start_time = Instant::now();

    for batch_idx in 1..=num_batches {
        let batch_start_time = Instant::now();

        // 1) Collect experience
        let mut batch: Vec<Reward<N, AD>> = Vec::new();

        let mut final_scores: Vec<f32> = Vec::new();

        let play_start_time = Instant::now();

        for _ in 0..games_per_batch {
            let (game_results, final_score) = simulate_one_game(model, &device, discount_factor);
            final_scores.push(final_score as f32);
            batch.extend(game_results);
        }

        let play_elapsed = play_start_time.elapsed().as_secs_f64();

        let (mean_score, stddev_score) = mean_stddev(&final_scores);

        // 2) Compute discounted returns per-episode
        // (this is already done)

        // 3) normalize across the entire batch
        normalize_rewards(&mut batch);

        // 4) Build tensors
        let mut last_loss = 0.0;

        let BatchifyResult { x, returns, actions } = batchify(&batch, &device);

        let learning_start = Instant::now();

        for _ in 0..learning_steps_per_batch {
            // 5) Forward application to get logits, then logit probabilities
            let logits = model.get_output_tensor(x.clone()); // [B, 4]
            let log_props = log_softmax(logits, 1); // [B, 4]

            // 6) Select log p(a_t|s_t) for taken actions
            //      gather expects indices shape to match the gather result; expand to [B, 1], then squeeze
            let idx: Tensor<AD, 2, Int> = actions.clone().unsqueeze_dim(1); // Shape [B, 1] (Int)
            let chosen_log_p = log_props.gather(1, idx).squeeze::<1>(1); // TODO: what on earth is this doing

            // 7) Reinforce loss: -(log pi * returns).mean()
            let loss: Tensor<AD, 1> = -(chosen_log_p * returns.clone()).mean();

            // 8) Backprop + step
            let grads = loss.backward();
            let grads = GradientsParams::from_grads::<AD, _>(grads, &model.inner);
            model.inner = opt.step(lr, model.inner.clone(), grads);

            last_loss = loss.to_data().into_vec::<f32>().unwrap()[0];
        }

        let learning_elapsed = learning_start.elapsed().as_secs_f64();

        let batch_elapsed = batch_start_time.elapsed().as_secs_f64();
        let total_elapsed = start_time.elapsed().as_secs_f64();

        // (Optional) diagnostics
        println!("batch {batch_idx:>5} | loss={last_loss} | mean_score={mean_score:.2} | score_std={stddev_score:.2}");
        println!(
            "    Timing: Play time {:0.3} sec | learning {:0.3} sec | Total (batch) {:0.3} sec | Total (all) {:0.3} sec",
            play_elapsed, learning_elapsed, batch_elapsed, total_elapsed
        );
    }
}

/// Applies mean/stddev normalization across the entire training batch
fn normalize_rewards<const N: usize, B: Backend>(reward_batch: &mut [Reward<N, B>]) {
    // TODO: unit test this section frfr; I'm pretty sure this is right but again it's typo fodder
    let mut rewards_vec: Vec<f32> = reward_batch.iter().map(|x| x.reward).collect();
    let (mean, stddev) = mean_stddev(&rewards_vec);
    for reward in rewards_vec.iter_mut() {
        *reward = (*reward - mean) / (stddev + EPSILON);
    }

    for (reward, normalized) in reward_batch.iter_mut().zip(rewards_vec.into_iter()) {
        reward.reward = normalized;
    }
}

/// Returns rewards for a single game with the given model. These are discounted (that is, credit
/// is sent backwards across time) but not normalized, which should be done per batch.
fn simulate_one_game<const N: usize, B: Backend>(
    model: &PolicyNet<N, B>,
    device: &B::Device,
    discount_factor: f32,
) -> (Vec<Reward<N, B>>, u32) {
    let mut prng = RngPlacement::new();
    let mut game_state = GameState::new_random(&mut prng);

    let mut rewards: Vec<Reward<N, B>> = Vec::new();

    while !game_state.is_finished() {
        let input_tensor = model.input_to_tensor(&game_state, device);
        let output_tensor = model.get_output_tensor(input_tensor.clone());
        let next_move = model.get_move_from_output(&game_state, output_tensor.clone());

        let new_game_state = game_state
            .apply_move(next_move, &mut prng)
            .expect("Should only generate valid moves");

        let reward = (new_game_state.current_score() - game_state.current_score()) as f32;

        rewards.push(Reward {
            state: input_tensor,
            output: next_move,
            reward,
        });

        game_state = new_game_state;
    }

    // Apply discounted rewards
    // TODO: unit test this section frfr; I'm pretty sure this is right but it seems too easy
    let mut running_reward = 0.;
    for reward in rewards.iter_mut().rev() {
        running_reward = (running_reward * discount_factor) + reward.reward;
        reward.reward = running_reward;
    }

    let final_score = game_state.current_score();

    (rewards, final_score)
}
