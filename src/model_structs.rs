//! Here's where we actually define our model. This will be a neural net built using Burn.

use burn::backend::NdArray;
use burn::nn::Linear;
use burn::nn::LinearConfig;
use burn::nn::Relu;
use burn::prelude::*;
use burn::tensor::Tensor;
use burn::tensor::activation::softmax;
use burn::tensor::backend::Backend;

use crate::game_structs::GameState;
use crate::game_structs::Move;
use crate::game_traits::FullGame;
use crate::model_traits::Model;

#[allow(unused)] // for now
pub struct PolicyNet<const N: usize, B: Backend = NdArray> {
    inner: InnerModel<B>,
    device: B::Device,
}

#[derive(Module, Debug)]
struct InnerModel<B: Backend> {
    linear1: Linear<B>,
    linear2: Linear<B>,
    activation: Relu,
}

impl<const N: usize, B: Backend> PolicyNet<N, B> {
    pub fn new() -> Self {
        let device: B::Device = Default::default();
        Self {
            // MLP
            //      input layer is the N*N inputs
            //      hidden layer of size N^3 (whatever)
            //      output layer of size 4 (up/left/down/right preferences)
            inner: InnerModel {
                // note: this does a good job initializing these things
                linear1: LinearConfig::new(N * N, N * N * N).init(&device),
                linear2: LinearConfig::new(N * N * N, 4).init(&device),
                activation: Relu::new(),
            },
            device,
        }
    }
}

impl<B: Backend> InnerModel<B> {
    fn forward(&self, x: Tensor<B, 1>) -> Tensor<B, 1> {
        let x = self.linear1.forward(x);
        let x = self.activation.forward(x);
        let x = self.linear2.forward(x);
        let x = self.activation.forward(x);
        let x = softmax(x, 0);

        x
    }
}

impl<const N: usize, B: Backend> Model<N, B> for PolicyNet<N, B> {
    fn input_to_tensor(&self, state: &GameState<N>) -> Tensor<B, 1> {
        let mut inputs: Vec<f32> = Vec::with_capacity(N * N);
        for y in 0..N {
            for x in 0..N {
                inputs.push(state.get_val(x, y) as f32);
            }
        }

        let input = Tensor::<B, 1>::from_data(inputs.as_slice(), &self.device);

        input
    }

    fn get_output_tensor(&self, input: Tensor<B, 1>) -> Tensor<B, 1> {
        self.inner.forward(input)
    }

    fn get_move_from_output(&self, state: &GameState<N>, output: Tensor<B, 1>) -> Move {
        let row: Vec<f32> = output.into_data().into_vec().expect("Should be able to convert to vec");
        let mut best_move = Move::Left;
        let mut best_score = f32::NEG_INFINITY;
        for (ind, val) in row.iter().copied().enumerate() {
            let next_move = match ind {
                0 => Move::Left,
                1 => Move::Right,
                2 => Move::Down,
                _ => Move::Up,
            };
            if val > best_score && state.is_legal_move(next_move) {
                best_move = next_move;
                best_score = val;
            }
        }

        best_move
    }
}
