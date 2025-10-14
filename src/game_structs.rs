use crate::game_traits::{AddRandomPiece, FullGame};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct GameState<const N: usize> {
    // 0 means empty square
    // u32 should be fine; I don't believe it's possible to overflow 18 bits in a 16 square grid
    grid: [[u32; N]; N]
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Move {
    Up, Down, Left, Right,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum MoveError {
    IllegalMove,
    // other variants?
}

impl <const N: usize> GameState<N> {
    // TODO: unit tests, including the order of operations, which is important
    fn left(&self) -> Self {
        let mut out = *self;

        for x in 1 .. N {
            for y in 0 .. N {
                if out.grid[y][x-1] == out.grid[y][x] {
                    out.grid[y][x-1] <<= 1;
                    out.grid[y][x] = 0;
                } else if out.grid[y][x-1] == 0 {
                    out.grid[y][x-1] = out.grid[y][x];
                    out.grid[y][x] = 0;
                }
            }
        }

        out
    }

    // TODO: unit tests, including the order of operations, which is important
    fn up(&self) -> Self {
        let mut out = *self;

        for y in 1 .. N {
            for x in 0 .. N {
                if out.grid[y-1][x] == out.grid[y][x] {
                    out.grid[y-1][x] <<= 1;
                    out.grid[y][x] = 0;
                } else if out.grid[y-1][x] == 0 {
                    out.grid[y-1][x] = out.grid[y][x];
                    out.grid[y][x] = 0;
                }
            }
        }

        out
    }

    // TODO: unit tests, including the order of operations, which is important
    fn right(&self) -> Self {
        let mut out = *self;

        for x in (0 .. N-1).rev() {
            for y in 0 .. N {
                if out.grid[y][x+1] == out.grid[y][x] {
                    out.grid[y][x+1] <<= 1;
                    out.grid[y][x] = 0;
                } else if out.grid[y][x+1] == 0 {
                    out.grid[y][x+1] = out.grid[y][x];
                    out.grid[y][x] = 0;
                }
            }
        }

        out
    }

    // TODO: unit tests, including the order of operations, which is important
    fn down(&self) -> Self {
        let mut out = *self;

        for y in (0 .. N-1).rev() {
            for x in 0 .. N {
                if out.grid[y+1][x] == out.grid[y][x] {
                    out.grid[y+1][x] <<= 1;
                    out.grid[y][x] = 0;
                } else if out.grid[y+1][x] == 0 {
                    out.grid[y+1][x] = out.grid[y][x];
                    out.grid[y][x] = 0;
                }
            }
        }

        out
    }
}

impl <const N: usize> FullGame for GameState<N> {
    fn apply_move<R: AddRandomPiece<GameState<N>>>(&self, m: Move, r: &mut R) -> Result<GameState<N>, MoveError> {
        let mut next_state = self.clone();

        next_state = match m {
            Move::Up => next_state.up(),
            Move::Down => next_state.down(),
            Move::Left => next_state.left(),
            Move::Right => next_state.right(),
        };

        if &next_state == self {
            return Err(MoveError::IllegalMove);
        }

        next_state = r.next_piece(&next_state);

        Ok(next_state)
    }

    fn is_finished(&self) -> bool {
        self.grid.iter().all(|row| row.iter().all(|&cell| cell != 0))
    }

    fn current_score(&self) -> u32 {
        self.grid.iter().flat_map(|row| row.iter()).sum()
    }
}