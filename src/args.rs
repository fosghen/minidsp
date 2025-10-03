use clap::{Args, Parser, Subcommand};

// Делаем парсер командной строки
// структура такая: [operation] [operation args]
// gen 
//     sine -freq -phase -ampl -duration
//     noise -ampl -std -mu
#[derive(Parser)]
#[command(name = "minidsp")]
#[command(version = "0.1")]
#[command(about = "Make some dsp with .wav files", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Generare signal
    Gen(GenArgs),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct GenArgs{
    #[command(subcommand)]
    pub command: Option<GenCommands>
}

#[derive(Debug, Subcommand)]
pub enum GenCommands {
    Sine {
        #[arg(short, long, default_value_t = 50., help = "frequency in Hz")]
        freq: f64,
        #[arg(short, long, default_value_t = 0., help = "phase in radians")]
        phase: f64,
        #[arg(short, long, default_value_t = 1., help = "duration in seconds")]
        duration: f64,
        #[arg(short, long, default_value_t = 1., help = "amplitude of sinus")]
        amplitude: f64,
    },

    Noise {
        #[arg(short, long, default_value_t = 1., help = "amplitude")]
        amplitude: f64,
        #[arg(short, long, default_value_t = 1., help = "standart deviation")]
        std: f64,
        #[arg(short, long, default_value_t = 1., help = "mean of noise")]
        mu: f64,
    },

    Sweep {
        #[arg(long, default_value_t = 1., help = "start frequency")]
        f0: f64,
        #[arg(long, default_value_t = 1., help = "frequency at t1")]
        f1: f64,
        #[arg(long, default_value_t = 1., help = "time for stop sweep")]
        t1: f64,
        #[arg(short, long, default_value = "linear", help = "type of sweep: linear, quadratic, logarithmic, hyperbolic")]
        method: String,
        #[arg(short, long, help = "only for quadratic, vertex of the parabola")]
        vertex_zero: bool,
    }
}
