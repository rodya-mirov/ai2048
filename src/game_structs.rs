// This really is what I want, clippy, get off my back
#![allow(clippy::needless_range_loop)]

use crate::game_traits::{AddRandomPiece, FullGame};
use rand::{Rng, SeedableRng};

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct GameState<const N: usize> {
    // 0 means empty square; n>0 means 2<<n
    // u32 should be fine; I don't believe it's possible to overflow 18 bits in a 16 square grid
    grid: [[u8; N]; N],
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
    pub fn new_empty() -> Self {
        Self { grid: [[0; N]; N] }
    }

    pub fn new_random<R: AddRandomPiece<Self>>(r: &mut R) -> Self {
        let out = Self::new_empty();
        r.next_piece(&out)
    }

    #[inline(always)]
    pub fn get_val(&self, x: usize, y: usize) -> u8 {
        self.grid[y][x]
    }

    fn left(&self) -> Self {
        let mut out = *self;

        // (indexed by y, output is an x-value) -- cannot merge in a row twice in the same space,
        // so this tracks the last merge point, if any. Merges can happen at this point but not
        // earlier.
        let mut merge_limits: [usize; N] = [0; N];

        for y in 0..N {
            // from left to right, attempt to move each piece left until it can't anymore,
            // except that you cannot merge the same square more than once
            for x in 1..N {
                let val = out.grid[y][x];
                if val == 0 {
                    continue;
                }

                // now slide it left continually until it hits a wall or the merge limit
                let mut new_x = x - 1;

                loop {
                    if out.grid[y][new_x] == 0 {
                        // blank space; move and continue
                        out.grid[y][new_x] = val;
                        out.grid[y][new_x + 1] = 0;
                    } else if out.grid[y][new_x] == val {
                        // merge; do the merge and stop the train
                        out.grid[y][new_x] = val + 1;
                        out.grid[y][new_x + 1] = 0;
                        merge_limits[y] = new_x + 1;
                        break;
                    } else {
                        // we hit a wall, stop moving
                        break;
                    }

                    // if we're at the end of the line, stop
                    if new_x == merge_limits[y] {
                        break;
                    }

                    // otherwise set up the next loop
                    new_x -= 1;
                }
            }
        }

        out
    }

    fn up(&self) -> Self {
        let mut out = *self;

        // (indexed by x, output is a y-value) -- cannot merge in a column twice in the same space,
        // so this tracks the last merge point, if any. Merges can happen at this point but not
        // earlier.
        let mut merge_limits: [usize; N] = [0; N];

        for x in 0..N {
            // from top to bottom, attempt to move each piece upward until it can't anymore,
            // except that you cannot merge the same square more than once
            for y in 1..N {
                let val = out.grid[y][x];
                if val == 0 {
                    continue;
                }

                // now slide it up continually until it hits a wall or the merge limit
                let mut new_y = y - 1;

                loop {
                    if out.grid[new_y][x] == 0 {
                        out.grid[new_y][x] = val;
                        out.grid[new_y + 1][x] = 0;
                    } else if out.grid[new_y][x] == val {
                        out.grid[new_y][x] = val + 1;
                        out.grid[new_y + 1][x] = 0;
                        merge_limits[x] = new_y + 1;
                        break;
                    } else {
                        // we hit a wall, stop moving
                        break;
                    }

                    // if we're at the end of the line, stop
                    if new_y == merge_limits[x] {
                        break;
                    }

                    // otherwise set up the next loop
                    new_y -= 1;
                }
            }
        }

        out
    }

    fn right(&self) -> Self {
        let mut out = *self;

        // (indexed by y, output is an x-value) -- cannot merge in a row twice in the same space,
        // so this tracks the last merge point, if any. Merges can happen at this point but not
        // earlier.
        let mut merge_limits: [usize; N] = [N - 1; N];

        for y in 0..N {
            // from right to left, attempt to move each piece right until it can't anymore,
            // except that you cannot merge the same square more than once
            for x in (0..N - 1).rev() {
                let val = out.grid[y][x];
                if val == 0 {
                    continue;
                }

                // now slide it right continually until it hits a wall or the merge limit
                let mut new_x = x + 1;

                loop {
                    if out.grid[y][new_x] == 0 {
                        // blank space; move and continue
                        out.grid[y][new_x] = val;
                        out.grid[y][new_x - 1] = 0;
                    } else if out.grid[y][new_x] == val {
                        // merge; do the merge and stop the train
                        out.grid[y][new_x] = val + 1;
                        out.grid[y][new_x - 1] = 0;
                        merge_limits[y] = new_x - 1;
                        break;
                    } else {
                        // we hit a wall, stop moving
                        break;
                    }

                    // if we're at the end of the line, stop
                    if new_x == merge_limits[y] {
                        break;
                    }

                    // otherwise set up the next loop
                    new_x += 1;
                }
            }
        }

        out
    }

    fn down(&self) -> Self {
        let mut out = *self;

        // (indexed by x, output is a y-value) -- cannot merge in a column twice in the same space,
        // so this tracks the last merge point, if any. Merges can happen at this point but not
        // earlier.
        let mut merge_limits: [usize; N] = [N - 1; N];

        for x in 0..N {
            // from bottom to top, attempt to move each piece downward until it can't anymore,
            // except that you cannot merge the same square more than once
            for y in (0..N - 1).rev() {
                let val = out.grid[y][x];
                if val == 0 {
                    continue;
                }

                // now slide it down continually until it hits a wall or the merge limit
                let mut new_y = y + 1;

                loop {
                    if out.grid[new_y][x] == 0 {
                        out.grid[new_y][x] = val;
                        out.grid[new_y - 1][x] = 0;
                    } else if out.grid[new_y][x] == val {
                        out.grid[new_y][x] = val + 1;
                        out.grid[new_y - 1][x] = 0;
                        merge_limits[x] = new_y - 1;
                        break;
                    } else {
                        // we hit a wall, stop moving
                        break;
                    }

                    // if we're at the end of the line, stop
                    if new_y == merge_limits[x] {
                        break;
                    }

                    // otherwise set up the next loop
                    new_y += 1;
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
        let mut next_state = *self;

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
        self.grid.iter().flat_map(|row| row.iter()).map(|bits| 1_u32 << bits).sum()
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

        let (x, y) = free_spaces[self.rng.random_range(0..free_spaces.len())];

        let is_two = self.rng.random_bool(0.5);

        let mut out_state = *in_state;
        out_state.grid[y][x] = if is_two { 1 } else { 2 };

        out_state
    }
}
