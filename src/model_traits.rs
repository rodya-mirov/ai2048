use burn::prelude::Backend;
use burn::prelude::Tensor;

use crate::game_structs::GameState;
use crate::game_structs::Move;

pub trait Model<const N: usize, B: Backend> {
    fn input_to_tensor(&self, state: &GameState<N>, device: &B::Device) -> Tensor<B, 1>;
    fn get_output_tensor<const D: usize>(&self, input: Tensor<B, D>) -> Tensor<B, D>;
    fn get_move_from_output(&self, state: &GameState<N>, output: Tensor<B, 1>) -> Move;

    fn get_next_move(&self, state: &GameState<N>, device: &B::Device) -> Move {
        let input_tensor = self.input_to_tensor(state, device);
        let output_tensor = self.get_output_tensor(input_tensor);
        let next_move = self.get_move_from_output(state, output_tensor);
        next_move
    }
}
