#![allow(clippy::let_and_return)]

use std::io;

use burn::backend::Autodiff;
use burn::backend::NdArray;
use burn::backend::ndarray::NdArrayDevice;
use clap::Parser;

use crate::cli::Cli;
use crate::cli::Commands;
use crate::model_structs::PolicyNet;
use crate::model_structs::PolicyNetConfig;

mod game_structs;
mod game_traits;

mod model_structs;
mod model_traits;

mod training;

mod cli;
mod tui;

/// Currently, main is just "run 2048 in the terminal"
/// It will be replaced by something more sophisticated in the future
fn main() -> io::Result<()> {
    let cli = Cli::parse();

    println!("Received command {:?}", cli.command);

    match cli.command {
        Commands::Play { seed } => {
            println!("Starting interactive 2048...");
            if let Some(s) = seed {
                println!("Using PRNG seed {s}");
            }

            tui::play::<4>(seed)?;
        }

        Commands::AutoPlay { seed } => {
            println!("Starting automatic 2048...");
            if let Some(s) = seed {
                println!("Using PRNG seed {s}");
            }

            let device = NdArrayDevice::default();
            let model: PolicyNet<4, NdArray> = PolicyNetConfig::new().init(&device);

            tui::simulate(seed, &model, &device)?;
        }

        Commands::Train {
            max_time,
            output,
            learning_rate,
            games_per_batch,
            learning_steps_per_batch,
            discount_factor,
            l2_reg,
        } => {
            println!("Starting model training");
            println!("Model will be saved in {output}");

            let device = NdArrayDevice::default();
            let mut model: PolicyNet<4, Autodiff<NdArray>> = PolicyNetConfig::new().init(&device);

            training::train(
                &mut model,
                max_time,
                learning_rate,
                games_per_batch,
                learning_steps_per_batch,
                discount_factor,
                l2_reg,
            );

            tui::simulate(None, &model, &device)?;
        }
    }

    Ok(())
}
