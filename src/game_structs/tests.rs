use crate::game_structs::{GameState, Move, RngPlacement};
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
                    out.grid[y][x] = 1;
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
                    out.grid[y][x] = 2;
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
            [1, 1, 0, 2],
            [1, 1, 1, 0],
            [3, 2, 2, 2],
            [4, 2, 3, 2],
        ],
    };

    out
}

fn crowded_grid_b() -> GameState<4> {
    #[rustfmt::skip]
    let out = GameState {
        grid: [
            [1, 0, 1, 1],    // merge + shift test
            [0, 2, 2, 0],    // merge in middle
            [3, 0, 0, 3],    // two far-apart merges possible
            [4, 3, 3, 3],   // only one merge allowed (8+8 once)
        ],
    };

    out
}

fn crowded_grid_c() -> GameState<4> {
    #[rustfmt::skip]
    let out = GameState {
        grid: [
            [1, 2, 3, 1],
            [1, 2, 3, 1],
            [2, 3, 4, 2],
            [2, 0, 0, 2],
        ],
    };

    out
}

fn crowded_grid_d() -> GameState<6> {
    #[rustfmt::skip]
    let out = GameState {
        grid: [
            [1, 1, 2, 3, 0, 0],
            [0, 2, 2, 0, 1, 1],
            [3, 4, 0, 0, 0, 3],
            [3, 4, 4, 3, 2, 2],
            [0, 0, 3, 3, 3, 0],
            [1, 2, 3, 4, 5, 6],
        ],
    };

    out
}

#[test]
fn test_move_left_boring() {
    let mut rng = NoPlacement {};

    let start = boring_grid();

    let actual = start.apply_move(Move::Left, &mut rng).unwrap();

    #[rustfmt::skip]
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
fn test_move_left_crowded_a() {
    let mut rng = NoPlacement {};

    let start = crowded_grid_a();

    let actual = start.apply_move(Move::Left, &mut rng).unwrap();

    #[rustfmt::skip]
    let expected = GameState {
        grid: [
            [2, 2, 0, 0],
            [2, 1, 0, 0],
            [3, 3, 2, 0],
            [4, 2, 3, 2],
        ],
    };

    assert_eq!(actual, expected);
}

#[test]
fn test_move_left_crowded_b() {
    let mut rng = NoPlacement {};

    let start = crowded_grid_b();

    let actual = start.apply_move(Move::Left, &mut rng).unwrap();

    #[rustfmt::skip]
    let expected = GameState {
        grid: [
            [2, 1, 0, 0],
            [3, 0, 0, 0],
            [4, 0, 0, 0],
            [4, 4, 3, 0]
        ],
    };

    assert_eq!(actual, expected);
}

#[test]
fn test_move_left_crowded_c() {
    let mut rng = NoPlacement {};

    let start = crowded_grid_c();

    let actual = start.apply_move(Move::Left, &mut rng).unwrap();

    #[rustfmt::skip]
    let expected = GameState {
        grid: [
            [1, 2, 3, 1],
            [1, 2, 3, 1],
            [2, 3, 4, 2],
            [3, 0, 0, 0],
        ],
    };

    assert_eq!(actual, expected);
}

#[test]
fn test_move_left_crowded_d() {
    let mut rng = NoPlacement {};

    let start = crowded_grid_d();

    let actual = start.apply_move(Move::Left, &mut rng).unwrap();

    #[rustfmt::skip]
    let expected = GameState {
        grid: [
            [2, 2, 3, 0, 0, 0],
            [3, 2, 0, 0, 0, 0],
            [3, 4, 3, 0, 0, 0],
            [3, 5, 3, 3, 0, 0],
            [4, 3, 0, 0, 0, 0],
            [1, 2, 3, 4, 5, 6],
        ],
    };

    assert_eq!(actual, expected);
}

#[test]
fn test_move_right_boring() {
    let mut rng = NoPlacement {};

    let start = boring_grid();

    let actual = start.apply_move(Move::Right, &mut rng).unwrap();

    #[rustfmt::skip]
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
fn test_move_right_crowded_a() {
    let mut rng = NoPlacement {};

    let start = crowded_grid_a();
    let actual = start.apply_move(Move::Right, &mut rng).unwrap();

    #[rustfmt::skip]
    let expected = GameState {
        grid: [
            [0, 0, 2, 2],
            [0, 0, 1, 2],
            [0, 3, 2, 3],
            [4, 2, 3, 2]
        ],
    };

    assert_eq!(actual, expected);
}

#[test]
fn test_move_right_crowded_b() {
    let mut rng = NoPlacement {};

    let start = crowded_grid_b();
    let actual = start.apply_move(Move::Right, &mut rng).unwrap();

    #[rustfmt::skip]
    let expected = GameState {
        grid: [
            [0, 0, 1, 2],
            [0, 0, 0, 3],
            [0, 0, 0, 4],
            [0, 4, 3, 4],
        ],
    };

    assert_eq!(actual, expected);
}

#[test]
fn test_move_right_crowded_c() {
    let mut rng = NoPlacement {};

    let start = crowded_grid_c();
    let actual = start.apply_move(Move::Right, &mut rng).unwrap();

    #[rustfmt::skip]
    let expected = GameState {
        grid: [
            [1, 2, 3, 1],
            [1, 2, 3, 1],
            [2, 3, 4, 2],
            [0, 0, 0, 3],
        ],
    };

    assert_eq!(actual, expected);
}

#[test]
fn test_move_right_crowded_d() {
    let mut rng = NoPlacement {};

    let start = crowded_grid_d();
    let actual = start.apply_move(Move::Right, &mut rng).unwrap();

    #[rustfmt::skip]
    let expected = GameState {
        grid: [
            [0, 0, 0, 2, 2, 3],
            [0, 0, 0, 0, 3, 2],
            [0, 0, 0, 3, 4, 3],
            [0, 0, 3, 5, 3, 3],
            [0, 0, 0, 0, 3, 4],
            [1, 2, 3, 4, 5, 6],
        ],
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
            [1, 0, 2, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
        ],
    };

    assert_eq!(actual, expected);
}

#[test]
fn test_move_up_crowded_a() {
    let mut rng = FirstPlaceFours {};

    let start = crowded_grid_a();

    let actual = start.apply_move(Move::Up, &mut rng).unwrap();

    #[rustfmt::skip]
    let expected = GameState {
        grid: [
            [2, 2, 1, 3],
            [3, 3, 2, 2],
            [4, 2, 3, 0],
            [0, 0, 0, 0]
        ],
    };

    assert_eq!(actual, expected);
}

#[test]
fn test_move_up_crowded_b() {
    let mut rng = NoPlacement {};

    let start = crowded_grid_b();

    let actual = start.apply_move(Move::Up, &mut rng).unwrap();

    #[rustfmt::skip]
    let expected = GameState {
        grid: [
            [1, 2, 1, 1],
            [3, 3, 2, 4],
            [4, 0, 3, 0],
            [0, 0, 0, 0],
        ],
    };

    assert_eq!(actual, expected);
}

#[test]
fn test_move_up_crowded_c() {
    let mut rng = NoPlacement {};

    let start = crowded_grid_c();

    let actual = start.apply_move(Move::Up, &mut rng).unwrap();

    #[rustfmt::skip]
    let expected = GameState {
        grid: [
            [2, 3, 4, 2],
            [3, 3, 4, 3],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
        ],
    };

    assert_eq!(actual, expected);
}

#[test]
fn test_move_up_crowded_d() {
    let mut rng = NoPlacement {};

    let start = crowded_grid_d();

    let actual = start.apply_move(Move::Up, &mut rng).unwrap();

    #[rustfmt::skip]
    let expected = GameState {
        grid: [
            [1, 1, 3, 4, 1, 1],
            [4, 2, 4, 3, 2, 3],
            [1, 5, 4, 4, 3, 2],
            [0, 2, 0, 0, 5, 6],
            [0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0],
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
            [1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 2, 0, 0],
        ],
    };

    assert_eq!(actual, expected);
}

#[test]
fn test_move_down_crowded_a() {
    let mut rng = FirstPlaceFours {};

    let start = crowded_grid_a();
    let actual = start.apply_move(Move::Down, &mut rng).unwrap();

    #[rustfmt::skip]
    let expected = GameState {
        grid: [
            [2, 0, 0, 0],
            [2, 0, 1, 0],
            [3, 2, 2, 2],
            [4, 3, 3, 3],
        ],
    };

    assert_eq!(actual, expected);
}

#[test]
fn test_move_down_crowded_b() {
    let mut rng = NoPlacement {};

    let start = crowded_grid_b();
    let actual = start.apply_move(Move::Down, &mut rng).unwrap();

    #[rustfmt::skip]
    let expected = GameState {
        grid: [
            [0, 0, 0, 0],
            [1, 0, 1, 0],
            [3, 2, 2, 1],
            [4, 3, 3, 4],
        ],
    };

    assert_eq!(actual, expected);
}

#[test]
fn test_move_down_crowded_c() {
    let mut rng = NoPlacement {};

    let start = crowded_grid_c();
    let actual = start.apply_move(Move::Down, &mut rng).unwrap();

    #[rustfmt::skip]
    let expected = GameState {
        grid: [
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [2, 3, 4, 2],
            [3, 3, 4, 3],
        ],
    };

    assert_eq!(actual, expected);
}

#[test]
fn test_move_down_crowded_d() {
    let mut rng = NoPlacement {};

    let start = crowded_grid_d();
    let actual = start.apply_move(Move::Down, &mut rng).unwrap();

    #[rustfmt::skip]
    let expected = GameState {
        grid: [
            [0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0],
            [0, 1, 0, 0, 1, 1],
            [1, 2, 3, 3, 2, 3],
            [4, 5, 4, 4, 3, 2],
            [1, 2, 4, 4, 5, 6],
        ],
    };

    assert_eq!(actual, expected);
}

#[test]
fn test_prng_pieces_on_new_places_only() {
    // uses system random; this test is non-deterministic by design
    let mut rng = RngPlacement::new();

    // A big enough grid that "most of the time," the overwrite bug I originally noticed
    // will be triggered at least once
    const N: usize = 20;

    let mut state: GameState<N> = GameState::new_random(&mut rng);

    while !state.is_finished() {
        let old_state = state.clone();
        state = rng.next_piece(&state);

        // check that if a square is occupied in old, it's the same value in new
        for y in 0 .. N {
            for x in 0 .. N {
                let old_val = old_state.get_val(x, y);
                let new_val = state.get_val(x, y);

                if old_val != 0 {
                    assert_eq!(old_val, new_val, "Existing values should not be changed");
                }
            }
        }

        // check that exactly one new value is occupied in new, and it's a 2 or 4
        let mut new_count = 0;
        for y in 0 .. N {
            for x in 0 .. N {
                let old_val = old_state.get_val(x, y);
                let new_val = state.get_val(x, y);

                if old_val == 0 && new_val != 0 {
                    new_count += 1;
                    assert!(new_val == 1 || new_val == 2);
                }
            }
        }
        assert_eq!(new_count, 1);
    }
}
