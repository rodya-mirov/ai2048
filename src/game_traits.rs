use crate::game_structs::{Move, MoveError};

pub trait FullGame: Sized {
    /// Apply the move, then use an appropriate RNG to add the next square
    /// Returns an error if the move is invalid (results in no moves)
    fn apply_move<R: AddRandomPiece<Self>>(&self, m: Move, r: &mut R) -> Result<Self, MoveError>;

    /// Check if the game is finished (no legal moves / all squares full)
    fn is_finished(&self) -> bool;

    /// Get the current score of the game (sum of all values).
    fn current_score(&self) -> u32;
}

pub trait AddRandomPiece<State> {
    /// Functionality for adding the next piece to the game state. Left intentionally very
    /// vague to support testability and, frankly, to make the type definitions simpler.
    fn next_piece(&mut self, in_state: &State) -> State;
}