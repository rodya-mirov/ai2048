//! Module for visualizing a 2048 game in crossterm.
//!
//! Note this is more or less completely AI-generated because I don't care very much about this
//! part of the project.
use crate::game_structs::GameState;
use crate::game_traits::FullGame;
use crossterm::{
    cursor, execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal,
};
use std::io;
use std::io::Write;

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
