mod args;
mod generate;
mod signal;

use clap::Parser;
use args::{Cli, Commands, GenCommands};

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Gen(signal) => {
            match signal.command {
                Some(GenCommands::Sine {
                     freq,
                     phase,
                     duration,
                     amplitude }) => {
                    let signal = generate::create_sine(freq, phase, duration, amplitude);
                    let filename = format!("sine_{}hz.csv", freq);
                    let _ = signal::save_csv(&signal, &filename);
                    println!("Generate sinus");
                },
                Some(GenCommands::Noise {
                     std,
                     mu,
                     duration}) => {
                    match generate::create_noise(duration, std, mu) {
                        Ok(signal) => {
                            let filename = format!("noise_{}mu_{}std.csv", mu, std);
                            let _ = signal::save_csv(&signal, &filename);
                            println!("Genearate noise!!");
                        }
                        Err(e) => {println!("Error in generation noise: {e}");}
                    }
                    

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