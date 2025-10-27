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
    /// Sum of two signals
    Add {
        #[arg(short('1'), long, help = "first signal")]
        signal1: String,
        #[arg(short('2'), long, help = "second signal")]
        signal2: String,
        #[arg(
            short,
            long,
            default_value = "sum_of_signals.wav",
            help = "fname of output signal"
        )]
        out_signal: String,
    },
    /// Substraction of two signals
    Sub {
        #[arg(short('1'), long, help = "first signal")]
        signal1: String,
        #[arg(short('2'), long, help = "second signal")]
        signal2: String,
        #[arg(
            short,
            long,
            default_value = "sub_of_signals.wav",
            help = "fname of output signal"
        )]
        out_signal: String,
    },
    /// Multiplex of two signals
    Mux {
        #[arg(short('1'), long, help = "first signal")]
        signal1: String,
        #[arg(short('2'), long, help = "second signal")]
        signal2: String,
        #[arg(
            short,
            long,
            default_value = "mux_of_signals.wav",
            help = "fname of output signal"
        )]
        out_signal: String,
    },
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct GenArgs {
    #[command(subcommand)]
    pub command: Option<GenCommands>,
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
        #[arg(short, long, default_value = "", help = "filename")]
        out_filename: String,
    },

    Noise {
        #[arg(short, long, default_value_t = 1., help = "duration")]
        duration: f64,
        #[arg(short, long, default_value_t = 1., help = "standart deviation")]
        std: f64,
        #[arg(short, long, default_value_t = 1., help = "mean of noise")]
        mu: f64,
        #[arg(short, long, default_value = "", help = "filename")]
        out_filename: String,
    },

    Sweep {
        #[arg(long, default_value_t = 1., help = "start frequency")]
        f0: f64,
        #[arg(long, default_value_t = 1., help = "frequency at t1")]
        f1: f64,
        #[arg(long, default_value_t = 1., help = "time for stop sweep")]
        t1: f64,
        #[arg(
            short,
            long,
            default_value = "linear",
            help = "type of sweep: linear, quadratic, logarithmic, hyperbolic"
        )]
        method: String,
        #[arg(short, long, help = "only for quadratic, vertex of the parabola")]
        vertex_zero: bool,
        #[arg(short, long, default_value = "", help = "filename")]
        out_filename: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_parses_gen_sine_with_long_flags() {
        let cli = Cli::try_parse_from([
            "minidsp",
            "gen",
            "sine",
            "--freq",
            "440",
            "--phase",
            "0.0",
            "--duration",
            "2.0",
            "--amplitude",
            "0.5",
        ])
        .expect("should parse");

        if let Commands::Gen(gen_args) = cli.command {
            match gen_args.command {
                Some(GenCommands::Sine {
                    freq,
                    phase,
                    duration,
                    amplitude,
                    out_filename,
                }) => {
                    assert_eq!(freq, 440.0);
                    assert_eq!(phase, 0.0);
                    assert_eq!(duration, 2.0);
                    assert_eq!(amplitude, 0.5);
                    assert_eq!(out_filename, "");
                }
                other => panic!("expected Sine, got {:?}", other),
            }
        }
    }

    #[test]
    fn test_parses_gen_noise_with_short_flags() {
        // short flags по именам полей: duration -> -a, std -> -s, mu -> -m
        let cli = Cli::try_parse_from([
            "minidsp", "gen", "noise", "-d", "0.2", "-s", "0.1", "-m", "0.0",
        ])
        .expect("should parse");

        if let Commands::Gen(gen_args) = cli.command {
            match gen_args.command {
                Some(GenCommands::Noise {
                    duration,
                    std,
                    mu,
                    out_filename,
                }) => {
                    assert_eq!(duration, 0.2);
                    assert_eq!(std, 0.1);
                    assert_eq!(mu, 0.0);
                    assert_eq!(out_filename, "");
                }
                other => panic!("expected Noise, got {:?}", other),
            }
        }
    }

    #[test]
    fn test_parses_gen_sweep_with_defaults_present() {
        // Можно опустить флаги, у которых есть default_value_t / default_value
        let cli = Cli::try_parse_from(["minidsp", "gen", "sweep"]).expect("should parse");

        if let Commands::Gen(gen_args) = cli.command {
            match gen_args.command {
                Some(GenCommands::Sweep {
                    f0,
                    f1,
                    t1,
                    method,
                    vertex_zero,
                    out_filename,
                }) => {
                    assert_eq!(f0, 1.0);
                    assert_eq!(f1, 1.0);
                    assert_eq!(t1, 1.0);
                    assert_eq!(method, "linear");
                    assert!(!vertex_zero);
                    assert_eq!(out_filename, "");
                }
                other => panic!("expected Sweep, got {:?}", other),
            }
        }
    }

    #[test]
    fn test_parses_gen_without_subcommand_as_none() {
        // subcommand у Gen — Option<GenCommands>, значит без подкоманды вернётся None
        let cli = Cli::try_parse_from(["minidsp", "gen"]).expect("should parse");

        if let Commands::Gen(gen_args) = cli.command {
            assert!(gen_args.command.is_none(), "expected no subcommand");
        }
    }

    #[test]
    fn test_rejects_unknown_subcommand() {
        let parsed = Cli::try_parse_from(["minidsp", "gen", "unknown"]);
        assert!(parsed.is_err(), "unknown subcommand must error");
    }
}
