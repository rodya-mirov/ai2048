use crate::game_structs::{GameState, Move};

pub trait Model<const N: usize> {
    fn get_next_move(&self, state: &GameState<N>) -> Move;
}