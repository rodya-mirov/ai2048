//! Clap CLI for dispatching what we're gonna do

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    version,
    about = "2048 AI Playground",
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Play the game interactively in the terminal
    Play {
        /// Optional seed for the PRNG
        #[arg(short, long)]
        seed: Option<u64>,
    },

    /// Indicate we want to train a new model
    Train {
        /// Number of training iterations
        #[arg(short, long, default_value_t = 1000)]
        iterations: usize,

        /// Path to save the trained model
        #[arg(short, long, default_value = "model.bin")]
        output: String,

        /// Learning rate
        #[arg(short = 'r', long, default_value_t = 0.001)]
        learning_rate: f32,
    }
}