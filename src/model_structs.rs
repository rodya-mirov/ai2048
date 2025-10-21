//! Here's where we actually define our model. This will be a neural net built using Burn.

use burn::nn::Linear;
use burn::nn::LinearConfig;
use burn::nn::Relu;
use burn::prelude::*;
use burn::tensor::Tensor;
use burn::tensor::backend::Backend;

use crate::game_structs::GameState;
use crate::game_structs::Move;
use crate::game_traits::FullGame;
use crate::model_traits::Model;

#[derive(Config, Debug)]
pub struct PolicyNetConfig {}

impl PolicyNetConfig {
    pub fn init<const N: usize, B: Backend>(&self, device: &B::Device) -> PolicyNet<N, B> {
        PolicyNet {
            inner: InnerModel {
                // note: this does a good job initializing these things
                // shared portion
                linear1: LinearConfig::new(N * N, N * N * N * N).init(device),
                linear2: LinearConfig::new(N * N * N * N, N * N * N * N).init(device),

                // head-specific portions
                actor_head: LinearConfig::new(N * N * N * N, 4).init(device),
                critic_head: LinearConfig::new(N * N * N * N, 1).init(device),

                // not sure why we even need this
                activation: Relu::new(),
            },
        }
    }
}

pub struct PolicyNet<const N: usize, B: Backend> {
    pub inner: InnerModel<B>,
}

#[derive(Module, Debug)]
pub struct InnerModel<B: Backend> {
    // shared portion
    linear1: Linear<B>,
    linear2: Linear<B>,

    // actor portion
    actor_head: Linear<B>,

    // critic portion
    critic_head: Linear<B>,

    // used between all layers
    activation: Relu,
}

impl<B: Backend> InnerModel<B> {
    /// Forward application of the network, yielding (actor_output, critic_output)
    fn forward<const D: usize>(&self, x: Tensor<B, D>) -> (Tensor<B, D>, Tensor<B, D>) {
        let x = self.linear1.forward(x);
        let x = self.activation.forward(x);
        let x = self.linear2.forward(x);
        let shared = self.activation.forward(x);

        let actor_logits = self.actor_head.forward(shared.clone()); // [B, 4]
        let critic_value = self.critic_head.forward(shared); // [B, 1]

        (actor_logits, critic_value)
    }
}

impl<const N: usize, B: Backend> Model<N, B> for PolicyNet<N, B> {
    fn input_to_tensor(&self, state: &GameState<N>, device: &B::Device) -> Tensor<B, 1> {
        let mut inputs: Vec<f32> = Vec::with_capacity(N * N);
        for y in 0..N {
            for x in 0..N {
                inputs.push(state.get_val(x, y) as f32);
            }
        }

        let input = Tensor::<B, 1>::from_data(inputs.as_slice(), device);

        input
    }

    fn get_output_tensor<const D: usize>(&self, input: Tensor<B, D>) -> (Tensor<B, D>, Tensor<B, D>) {
        self.inner.forward(input)
    }

    fn get_move_from_output(&self, state: &GameState<N>, output: Tensor<B, 1>) -> Move {
        let row: Vec<f32> = output.into_data().into_vec().expect("Should be able to convert to vec");

        let mut row_idxes: Vec<(usize, f32)> = row.into_iter().enumerate().collect();
        row_idxes.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap().reverse());

        for (ind, _weight) in row_idxes {
            let next_move = Move::from_idx(ind);
            if state.is_legal_move(next_move) {
                return next_move;
            }
        }

        panic!("No legal move found");
    }
}
