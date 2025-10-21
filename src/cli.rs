//! Clap CLI for dispatching what we're gonna do

use clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug)]
#[command(version, about = "2048 AI Playground")]
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

    /// Play the game automatically in the terminal, using an untrained model
    AutoPlay {
        /// Optional seed for the PRNG
        #[arg(short, long)]
        seed: Option<u64>,
    },

    /// Indicate we want to train a new model
    Train {
        /// Number of training iterations (batches)
        #[arg(short, long, default_value_t = 50)]
        iterations: usize,

        /// Number of games per batch
        #[arg(short, long, default_value_t = 5)]
        games_per_batch: usize,

        /// Number of learning steps per batch of training data
        #[arg(short, long, default_value_t = 1)]
        learning_steps_per_batch: usize,

        /// Discount factor for reinforcement learning
        #[arg(short, long, default_value_t = 0.99)]
        discount_factor: f32,

        /// Path to save the trained model
        #[arg(short, long, default_value = "model.bin")]
        output: String,

        /// Learning rate
        #[arg(short = 'r', long, default_value_t = 0.001)]
        learning_rate: f64,

        /// L2 regularization amount

        /// Discount factor for reinforcement learning
        #[arg(short, long, default_value_t = 0.0001)]
        l2_reg: f32,
    },
}
