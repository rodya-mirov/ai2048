use std::io;
use clap::Parser;
use crate::cli::{Cli, Commands};

mod game_structs;
mod game_traits;

mod cli;
mod tui;

/// Currently, main is just "run 2048 in the terminal"
/// It will be replaced by something more sophisticated in the future
fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Play { seed } => {
            println!("Starting interactive 2048...");
            if let Some (s) = seed {
                println!("Using PRNG seed {s}");
            }

            tui::play::<4>(seed)?;
        }

        Commands::Train { iterations, output, learning_rate } => {
            println!("Starting model training with {iterations} and learning rate of {learning_rate}");
            println!("Model will be saved in {output}");

            unimplemented!()
        }
    }

    Ok(())
}
