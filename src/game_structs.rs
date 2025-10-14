use crate::game_traits::{AddRandomPiece, FullGame};
use rand::{Rng, RngCore, SeedableRng};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct GameState<const N: usize> {
    // 0 means empty square
    // u32 should be fine; I don't believe it's possible to overflow 18 bits in a 16 square grid
    grid: [[u32; N]; N],
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum MoveError {
    IllegalMove,
    // other variants?
}

impl<const N: usize> GameState<N> {
    // TODO: unit tests, including the order of operations, which is important
    fn left(&self) -> Self {
        let mut out = *self;

        for x in 1..N {
            for y in 0..N {
                if out.grid[y][x - 1] == out.grid[y][x] {
                    out.grid[y][x - 1] <<= 1;
                    out.grid[y][x] = 0;
                } else if out.grid[y][x - 1] == 0 {
                    out.grid[y][x - 1] = out.grid[y][x];
                    out.grid[y][x] = 0;
                }
            }
        }

        out
    }

    // TODO: unit tests, including the order of operations, which is important
    fn up(&self) -> Self {
        let mut out = *self;

        for y in 1..N {
            for x in 0..N {
                if out.grid[y - 1][x] == out.grid[y][x] {
                    out.grid[y - 1][x] <<= 1;
                    out.grid[y][x] = 0;
                } else if out.grid[y - 1][x] == 0 {
                    out.grid[y - 1][x] = out.grid[y][x];
                    out.grid[y][x] = 0;
                }
            }
        }

        out
    }

    // TODO: unit tests, including the order of operations, which is important
    fn right(&self) -> Self {
        let mut out = *self;

        for x in (0..N - 1).rev() {
            for y in 0..N {
                if out.grid[y][x + 1] == out.grid[y][x] {
                    out.grid[y][x + 1] <<= 1;
                    out.grid[y][x] = 0;
                } else if out.grid[y][x + 1] == 0 {
                    out.grid[y][x + 1] = out.grid[y][x];
                    out.grid[y][x] = 0;
                }
            }
        }

        out
    }

    // TODO: unit tests, including the order of operations, which is important
    fn down(&self) -> Self {
        let mut out = *self;

        for y in (0..N - 1).rev() {
            for x in 0..N {
                if out.grid[y + 1][x] == out.grid[y][x] {
                    out.grid[y + 1][x] <<= 1;
                    out.grid[y][x] = 0;
                } else if out.grid[y + 1][x] == 0 {
                    out.grid[y + 1][x] = out.grid[y][x];
                    out.grid[y][x] = 0;
                }
            }
        }

        out
    }
}

impl<const N: usize> FullGame for GameState<N> {
    fn apply_move<R: AddRandomPiece<GameState<N>>>(
        &self,
        m: Move,
        r: &mut R,
    ) -> Result<GameState<N>, MoveError> {
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
        self.grid
            .iter()
            .all(|row| row.iter().all(|&cell| cell != 0))
    }

    fn current_score(&self) -> u32 {
        self.grid.iter().flat_map(|row| row.iter()).sum()
    }
}

pub struct RngPlacement {
    rng: rand::rngs::StdRng,
}

impl RngPlacement {
    pub fn new() -> RngPlacement {
        let seed: u64 = rand::random();
        Self::new_from_seed(seed)
    }

    pub fn new_from_seed(seed: u64) -> RngPlacement {
        RngPlacement {
            rng: rand::rngs::StdRng::seed_from_u64(seed),
        }
    }
}

impl<const N: usize> AddRandomPiece<GameState<N>> for RngPlacement {
    fn next_piece(&mut self, in_state: &GameState<N>) -> GameState<N> {
        let mut free_spaces = Vec::new();

        for x in 0..N {
            for y in 0..N {
                if in_state.grid[y][x] == 0 {
                    free_spaces.push((x, y));
                }
            }
        }

        let (y, x) = free_spaces[self.rng.random_range(0..free_spaces.len())];

        let is_two = self.rng.random_bool(0.5);

        let mut out_state = in_state.clone();
        out_state.grid[y][x] = if is_two { 2 } else { 4 };

        out_state
    }
}
