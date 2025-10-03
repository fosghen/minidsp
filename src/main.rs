mod args;

use clap::Parser;
use args::{Cli, Commands, GenCommands};



fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Gen(signal) => {
            match signal.command {
                Some(GenCommands::Sine {
                     freq: _,
                     phase: _,
                     duration: _,
                     amplitude: _ }) => {
                    println!("Generate sinus");
                },
                Some(GenCommands::Noise {
                     std: _,
                     mu: _,
                     amplitude: _ }) => {
                    println!("Genearate noise!!");
                },
                Some(GenCommands::Sweep { f0: _,
                     f1: _,
                     t1: _,
                     method: _,
                     vertex_zero: _,}) => {
                    println!("Genearate sweep!!");    
                }
                _ => {
                    println!("Do nothing");
                },
            }
        }
    }
}