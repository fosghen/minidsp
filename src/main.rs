mod args;
mod generate;
mod signal;
mod dsp;

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
                     amplitude,
                     out_filename }) => {
                    let signal = generate::create_sine(freq, phase, duration, amplitude);
                    let filename = if out_filename.len() == 0 {format!("sine_{}hz.wav", freq)} else {out_filename};
                    let _ = signal::save_wave(&signal, &filename);
                    println!("Generate sinus");
                },
                Some(GenCommands::Noise {
                     std,
                     mu,
                     duration,
                     out_filename}) => {
                    match generate::create_noise(duration, std, mu) {
                        Ok(signal) => {
                            let filename = if out_filename.len() == 0 {format!("noise_{}mu_{}std.wav", mu, std)} else {out_filename};
                            let _ = signal::save_wave(&signal, &filename);
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
                     vertex_zero,
                     out_filename}) => {
                    match method.as_str() {
                        "linear" => {
                            let signal = generate::create_linear_sweep(f0, f1, t1);
                            let filename = if out_filename.len() == 0 {format!("sweep_{f0}_{f1}_linear.wav")} else {out_filename};
                            let _ = signal::save_wave(&signal, &filename);
                            println!("Genearate sweep!!");
                        },
                        "hyperbolic" => {
                            let result = generate::create_hyperbolic_sweep(f0, f1, t1);
                            let filename = if out_filename.len() == 0 {format!("sweep_{f0}_{f1}_hyperbolic.wav")} else {out_filename};
                            if let Ok(signal) = result {
                                let _ = signal::save_wave(&signal, &filename);
                                println!("Genearate sweep!!");
                            } else {
                                println!("Fail to generate {filename}");
                            }
                        },
                        "quadratic" => {
                            let signal = generate::create_quadratic_sweep(f0, f1, t1, vertex_zero);
                            let filename = if out_filename.len() == 0 {format!("sweep_{f0}_{f1}_quadratic.wav")} else {out_filename};
                            let _ = signal::save_wave(&signal, &filename);
                            println!("Genearate sweep!!");
                        },
                        _ => {},
                    }
                }
                _ => {
                    println!("Do nothing");
                },
            }
        },
        Commands::Add{signal1, signal2, out_signal} => {
            let mut sig1: Vec<f64> = Vec::new();
            let mut sig2: Vec<f64> = Vec::new();

            let _ = signal::read_wave(&mut sig1, &signal1);
            let _ = signal::read_wave(&mut sig2, &signal2);

            let result = dsp::add_sgnals(&sig1, &sig2);

            let _ = signal::save_wave(&result, &out_signal);
        },
        Commands::Sub{signal1, signal2, out_signal} => {
            let mut sig1: Vec<f64> = Vec::new();
            let mut sig2: Vec<f64> = Vec::new();

            let _ = signal::read_wave(&mut sig1, &signal1);
            let _ = signal::read_wave(&mut sig2, &signal2);

            let result = dsp::sub_sgnals(&sig1, &sig2);

            let _ = signal::save_wave(&result, &out_signal);
        },
    }
}