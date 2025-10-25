use burn::prelude::Backend;
use burn::prelude::Tensor;

use crate::game_structs::GameState;
use crate::game_structs::Move;

pub trait Model<const N: usize, B: Backend> {
    /// Converts input to a tensor
    fn input_to_tensor(&self, state: &GameState<N>, device: &B::Device) -> Tensor<B, 1>;

    /// Returns the output (action) as well as critic (value) tensors for the input
    fn get_output_tensor<const D: usize>(&self, input: Tensor<B, D>) -> (Tensor<B, D>, Tensor<B, D>);

    /// Given an output tensor, compute the move it corresponds to
    fn get_move_from_output(&self, state: &GameState<N>, output: Tensor<B, 1>) -> MoveResult;

    fn get_next_move(&self, state: &GameState<N>, device: &B::Device) -> MoveResult {
        let input_tensor = self.input_to_tensor(state, device);
        let (actor_logits, _critic_value) = self.get_output_tensor(input_tensor);
        let next_move = self.get_move_from_output(state, actor_logits);
        next_move
    }
}

pub struct MoveResult {
    pub next_move: Move,
    // if the first choice was illegal but the second was legal, this is 1
    // values go from zero to three
    pub num_illegal_choices: u8,
}
