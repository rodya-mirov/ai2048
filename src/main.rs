use crate::game_structs::{GameState, Move, RngPlacement};
use crate::game_traits::FullGame;
use crossterm::event::{Event, KeyCode};
use crossterm::{execute, terminal};
use std::io;
use std::time::Duration;

mod game_structs;
mod game_traits;

mod tui;

/// Currently, main is just "run 2048 in the terminal"
/// It will be replaced by something more sophisticated in the future
fn main() -> io::Result<()> {
    let mut rng = RngPlacement::new();
    let mut game = GameState::<4>::new_random(&mut rng);
    let mut old_state = game.clone();

    // prepare terminal
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        crossterm::cursor::Hide
    )?;

    tui::render(&old_state, &game)?;

    loop {
        if crossterm::event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = crossterm::event::read()? {
                let mv = match key.code {
                    KeyCode::Char('q') => break, // quit
                    KeyCode::Up | KeyCode::Char('w') => Some(Move::Up),
                    KeyCode::Down | KeyCode::Char('s') => Some(Move::Down),
                    KeyCode::Left | KeyCode::Char('a') => Some(Move::Left),
                    KeyCode::Right | KeyCode::Char('d') => Some(Move::Right),
                    _ => None,
                };

                if let Some(mv) = mv {
                    match game.apply_move(mv, &mut rng) {
                        Ok(new_state) => {
                            old_state = game;
                            game = new_state;
                        }
                        Err(_) => {} // ignore illegal moves
                    }
                    tui::render(&old_state, &game)?;
                    if game.is_finished() {
                        execute!(stdout, crossterm::style::Print("Game over!"))?;
                        break;
                    }
                }
            }
        }
    }

    // cleanup
    execute!(
        stdout,
        crossterm::cursor::Show,
        terminal::LeaveAlternateScreen
    )?;
    terminal::disable_raw_mode()?;
    Ok(())
}
