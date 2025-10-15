use crate::cli::{Cli, Commands};
use crate::model_structs::PolicyNet;
use burn::backend::NdArray;
use clap::Parser;
use std::io;

mod game_structs;
mod game_traits;

mod model_structs;
mod model_traits;

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
        
        Commands::AutoPlay { seed } => {
            println!("Starting automatic 2048...");
            if let Some (s) = seed {
                println!("Using PRNG seed {s}");
            }

            let model: PolicyNet<4, NdArray> = PolicyNet::new();

            tui::simulate::<4>(seed, &model)?;
        }

        Commands::Train { iterations, output, learning_rate } => {
            println!("Starting model training with {iterations} and learning rate of {learning_rate}");
            println!("Model will be saved in {output}");

            unimplemented!()
        }
    }

    Ok(())
}
