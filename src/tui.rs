//! Module for visualizing a 2048 game in crossterm.
//!
//! Note this is more or less completely AI-generated because I don't care very much about this
//! part of the project.
use crate::game_structs::{GameState, Move, RngPlacement};
use crate::game_traits::FullGame;
use crossterm::event::{Event, KeyCode};
use crossterm::{
    cursor, execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal,
};
use std::io;
use std::io::Write;
use std::time::Duration;

pub fn render<const N: usize>(game: &GameState<N>) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

        for y in 0..N {
            for x in 0..N {
                let val = game.get_val(x, y);
                let color = match val {
                    0 => Color::DarkGrey,
                    2 => Color::Grey,
                    4 => Color::White,
                    8 => Color::Yellow,
                    16 => Color::DarkYellow,
                    32 => Color::Magenta,
                    64 => Color::Red,
                    128 => Color::Blue,
                    256 => Color::Cyan,
                    512 => Color::Green,
                    _ => Color::White,
                };
                execute!(
                    stdout,
                    SetBackgroundColor(Color::Black),
                    SetForegroundColor(color),
                    Print(format!(
                        "|{:>6}|",
                        if val == 0 {
                            ".".to_string()
                        } else {
                            val.to_string()
                        }
                    )),
                    ResetColor,
                )?;
            }
            execute!(stdout, Print("\r\n"))?;
        }

    execute!(
        stdout,
        Print(format!("\nScore: {}\n", game.current_score()))
    )?;
    stdout.flush()?;

    Ok(())
}

pub fn play<const N: usize>() -> io::Result<()> {
    let mut rng = RngPlacement::new();
    let mut game = GameState::<4>::new_random(&mut rng);

    // prepare terminal
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        cursor::Hide
    )?;

    render(&game)?;

    loop {
        if crossterm::event::poll(Duration::from_millis(100))?
            && let Event::Key(key) = crossterm::event::read()? {
                let mv = match key.code {
                    KeyCode::Char('q') => break, // quit
                    KeyCode::Up | KeyCode::Char('w') => Some(Move::Up),
                    KeyCode::Down | KeyCode::Char('s') => Some(Move::Down),
                    KeyCode::Left | KeyCode::Char('a') => Some(Move::Left),
                    KeyCode::Right | KeyCode::Char('d') => Some(Move::Right),
                    _ => None,
                };

                if let Some(mv) = mv {
                    if let Ok(new_state) = game.apply_move(mv, &mut rng) {
                        game = new_state;
                    }
                    render(&game)?;
                    if game.is_finished() {
                        execute!(stdout, Print("Game over!"))?;
                        break;
                    }
                }
            }
    }

    // cleanup
    execute!(
        stdout,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;
    terminal::disable_raw_mode()?;

    Ok(())
}
