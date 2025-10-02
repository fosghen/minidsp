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
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Generare signal
    Gen(GenArgs),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
struct GenArgs{
    #[command(subcommand)]
    command: Option<GenCommands>
}

#[derive(Debug, Subcommand)]
enum GenCommands {
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