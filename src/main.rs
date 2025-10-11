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
                Some(GenCommands::Sweep { 
                     f0,
                     f1,
                     t1,
                     method,
                     vertex_zero,}) => {
                    match method.as_str() {
                        "linear" => {
                            let signal = generate::create_linear_sweep(f0, f1, t1);
                            let filename = format!("sweep_{f0}_{f1}_linear.csv");
                            let _ = signal::save_csv(&signal, &filename);
                            println!("Genearate sweep!!");
                        },
                        "hyperbolic" => {
                            let signal = generate::create_hyperbolic_sweep(f0, f1, t1);
                            let filename = format!("sweep_{f0}_{f1}_hyperbolic.csv");
                            let _ = signal::save_csv(&signal, &filename);
                            println!("Genearate sweep!!");
                        },
                        "quadratic" => {
                            let signal = generate::create_quadratic_sweep(f0, f1, t1, vertex_zero);
                            let filename = format!("sweep_{f0}_{f1}_quadratic.csv");
                            let _ = signal::save_csv(&signal, &filename);
                            println!("Genearate sweep!!");
                        },
                        _ => {},
                    }
                }
                _ => {
                    println!("Do nothing");
                },
            }
        }
    }
}