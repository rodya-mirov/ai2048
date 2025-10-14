use crate::game_structs::{GameState, Move};
use crate::game_traits::{AddRandomPiece, FullGame};

/// Test fixture helper which allows movement with no additional placement afterward
struct NoPlacement {}

impl<State: Clone + Sized> AddRandomPiece<State> for NoPlacement {
    fn next_piece(&mut self, in_state: &State) -> State {
        in_state.clone()
    }
}

/// Test fixture helper which sweeps from top to bottom, left to right,
/// and places a two in the first empty spot (if any)
struct FirstPlaceTwos {}

impl<const N: usize> AddRandomPiece<GameState<N>> for FirstPlaceTwos {
    fn next_piece(&mut self, in_state: &GameState<N>) -> GameState<N> {
        let mut out = in_state.clone();

        for y in 0..N {
            for x in 0..N {
                if out.grid[y][x] == 0 {
                    out.grid[y][x] = 2;
                    return out;
                }
            }
        }

        // no placement possible
        out
    }
}

/// Test fixture helper which sweeps from top to bottom, left to right,
/// and places a four in the first empty spot (if any)
struct FirstPlaceFours {}

impl<const N: usize> AddRandomPiece<GameState<N>> for FirstPlaceFours {
    fn next_piece(&mut self, in_state: &GameState<N>) -> GameState<N> {
        let mut out = in_state.clone();

        for y in 0..N {
            for x in 0..N {
                if out.grid[y][x] == 0 {
                    out.grid[y][x] = 4;
                    return out;
                }
            }
        }

        // no moves possible
        out
    }
}

fn boring_grid() -> GameState<5> {
    GameState {
        grid: [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 2, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
        ],
    }
}

fn crowded_grid_a() -> GameState<4> {
    #[rustfmt::skip]
    let out = GameState {
        grid: [
            [2, 2, 0, 4],
            [2, 2, 2, 0],
            [8, 4, 4, 4],
            [16, 4, 8, 4]
        ],
    };

    out
}

#[test]
fn test_move_left_boring() {
    let mut rng = NoPlacement {};

    let start = boring_grid();

    let actual = start.apply_move(Move::Left, &mut rng).unwrap();

    let expected = GameState {
        grid: [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [2, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
        ],
    };

    assert_eq!(actual, expected);
}

#[test]
fn test_move_left_crowded() {
    let mut rng = NoPlacement {};

    let start = crowded_grid_a();

    let actual = start.apply_move(Move::Left, &mut rng).unwrap();

    let expected = GameState {
        grid: [[4, 4, 0, 0], [4, 2, 0, 0], [8, 8, 4, 0], [16, 4, 8, 4]],
    };

    assert_eq!(actual, expected);
}

#[test]
fn test_move_right_boring() {
    let mut rng = NoPlacement {};

    let start = boring_grid();

    let actual = start.apply_move(Move::Right, &mut rng).unwrap();

    let expected = GameState {
        grid: [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 2],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
        ],
    };

    assert_eq!(actual, expected);
}

#[test]
fn test_move_right_crowded() {
    let mut rng = NoPlacement {};

    let start = crowded_grid_a();
    let actual = start.apply_move(Move::Right, &mut rng).unwrap();

    let expected = GameState {
        grid: [[0, 0, 4, 4], [0, 0, 2, 4], [0, 8, 4, 8], [16, 4, 8, 4]],
    };

    assert_eq!(actual, expected);
}

#[test]
fn test_move_up_boring() {
    let mut rng = FirstPlaceTwos {};

    let start = boring_grid();

    let actual = start.apply_move(Move::Up, &mut rng).unwrap();

    #[rustfmt::skip]
    let expected = GameState {
        grid: [
            [2, 0, 2, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
        ],
    };

    assert_eq!(actual, expected);
}

#[test]
fn test_move_up_crowded() {
    let mut rng = FirstPlaceFours {};

    let start = crowded_grid_a();

    let actual = start.apply_move(Move::Up, &mut rng).unwrap();

    #[rustfmt::skip]
    let expected = GameState {
        grid: [
            [4, 4, 2, 8],
            [8, 8, 4, 4],
            [16, 4, 8, 0],
            [0, 0, 0, 0]
        ],
    };

    assert_eq!(actual, expected);
}

#[test]
fn test_move_down_boring() {
    let mut rng = FirstPlaceTwos {};

    let start = boring_grid();

    let actual = start.apply_move(Move::Down, &mut rng).unwrap();

    let expected = GameState {
        grid: [
            [2, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 2, 0, 0],
        ],
    };

    assert_eq!(actual, expected);
}

#[test]
fn test_move_down_crowded() {
    let mut rng = FirstPlaceFours {};

    let start = crowded_grid_a();
    let actual = start.apply_move(Move::Down, &mut rng).unwrap();

    #[rustfmt::skip]
    let expected = GameState {
        grid: [
            [4, 0, 0, 0],
            [4, 0, 2, 0],
            [8, 4, 4, 4],
            [16, 8, 8, 8]
        ],
    };

    assert_eq!(actual, expected);
}
