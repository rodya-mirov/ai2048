use std::io;

mod game_structs;
mod game_traits;

mod tui;

/// Currently, main is just "run 2048 in the terminal"
/// It will be replaced by something more sophisticated in the future
fn main() -> io::Result<()> {
    tui::play::<4>()?;

    Ok(())
}
