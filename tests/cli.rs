use assert_cmd::prelude::*;
use std::fs;
use std::process::Command;
use std::sync::{Mutex, OnceLock};

static SERIAL_TEST_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

fn serial_test_guard() -> std::sync::MutexGuard<'static, ()> {
    SERIAL_TEST_MUTEX
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap()
}

#[test]
fn test_general_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("minidsp")?;
    cmd.arg("-h");
    cmd.assert().success().stdout(
        r#"Make some dsp with .wav files

Usage: minidsp <COMMAND>

Commands:
  gen          Generare signal
  add          Sum of two signals
  sub          Substraction of two signals
  mux          Multiplex of two signals
  scale        Scaling of signal
  mov-average  Moving average
  help         Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
"#,
    );

    Ok(())
}

#[test]
fn test_gen_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("minidsp")?;
    cmd.arg("gen").arg("-h");
    cmd.assert()
        .success()
        .stdout(
r#"Generare signal

Usage: minidsp gen
       minidsp gen sine [OPTIONS]
       minidsp gen noise [OPTIONS]
       minidsp gen sweep [OPTIONS]
       minidsp gen help [COMMAND]...

Options:
  -h, --help  Print help

minidsp gen sine:
  -f, --freq <FREQ>                  frequency in Hz [default: 50]
  -p, --phase <PHASE>                phase in degrees [default: 0]
  -d, --duration <DURATION>          duration in seconds [default: 1]
  -a, --amplitude <AMPLITUDE>        amplitude of sinus [default: 1]
  -o, --out-filename <OUT_FILENAME>  filename [default: ]
  -h, --help                         Print help

minidsp gen noise:
  -d, --duration <DURATION>          duration [default: 1]
  -s, --std <STD>                    standart deviation [default: 1]
  -m, --mu <MU>                      mean of noise [default: 1]
  -o, --out-filename <OUT_FILENAME>  filename [default: ]
  -h, --help                         Print help

minidsp gen sweep:
      --f0 <F0>                      start frequency [default: 1]
      --f1 <F1>                      frequency at t1 [default: 1]
      --t1 <T1>                      time for stop sweep [default: 1]
  -m, --method <METHOD>              type of sweep: linear, quadratic, logarithmic, hyperbolic [default: linear]
  -v, --vertex-zero                  only for quadratic, vertex of the parabola
  -o, --out-filename <OUT_FILENAME>  filename [default: ]
  -h, --help                         Print help

minidsp gen help:
Print this message or the help of the given subcommand(s)
  [COMMAND]...  Print help for the subcommand(s)
"#);

    Ok(())
}

#[test]
fn test_add_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("minidsp")?;
    cmd.arg("add").arg("-h");
    cmd.assert().success().stdout(
        r#"Sum of two signals

Usage: minidsp add [OPTIONS] --signal1 <SIGNAL1> --signal2 <SIGNAL2>

Options:
  -1, --signal1 <SIGNAL1>        first signal
  -2, --signal2 <SIGNAL2>        second signal
  -o, --out-signal <OUT_SIGNAL>  fname of output signal [default: sum_of_signals.wav]
  -h, --help                     Print help
"#,
    );

    Ok(())
}

#[test]
fn test_sub_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("minidsp")?;
    cmd.arg("sub").arg("-h");
    cmd.assert().success().stdout(
        r#"Substraction of two signals

Usage: minidsp sub [OPTIONS] --signal1 <SIGNAL1> --signal2 <SIGNAL2>

Options:
  -1, --signal1 <SIGNAL1>        first signal
  -2, --signal2 <SIGNAL2>        second signal
  -o, --out-signal <OUT_SIGNAL>  fname of output signal [default: sub_of_signals.wav]
  -h, --help                     Print help
"#,
    );

    Ok(())
}

#[test]
fn test_mux_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("minidsp")?;
    cmd.arg("mux").arg("-h");
    cmd.assert().success().stdout(
        r#"Multiplex of two signals

Usage: minidsp mux [OPTIONS] --signal1 <SIGNAL1> --signal2 <SIGNAL2>

Options:
  -1, --signal1 <SIGNAL1>        first signal
  -2, --signal2 <SIGNAL2>        second signal
  -o, --out-signal <OUT_SIGNAL>  fname of output signal [default: mux_of_signals.wav]
  -h, --help                     Print help
"#,
    );

    Ok(())
}

#[test]
fn test_scale_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("minidsp")?;
    cmd.arg("scale").arg("-h");
    cmd.assert().success().stdout(
        r#"Scaling of signal

Usage: minidsp scale [OPTIONS] --signal <SIGNAL>

Options:
  -s, --signal <SIGNAL>          signal
  -a, --amplitude <AMPLITUDE>    second signal [default: 1]
  -o, --out-signal <OUT_SIGNAL>  fname of output signal [default: scaled_signal.wav]
  -h, --help                     Print help
"#,
    );

    Ok(())
}

#[test]
fn test_move_avarage_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("minidsp")?;
    cmd.arg("mov-average").arg("-h");
    cmd.assert().success().stdout(
        r#"Moving average

Usage: minidsp mov-average [OPTIONS] --signal <SIGNAL>

Options:
  -s, --signal <SIGNAL>                signal
  -k, --kernel-length <KERNEL_LENGTH>  length of window for average [default: 1]
  -o, --out-signal <OUT_SIGNAL>        fname of output signal [default: scaled_signal.wav]
  -h, --help                           Print help
"#,
    );

    Ok(())
}

#[test]
fn test_gen_sine() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("minidsp")?;
    cmd.arg("gen").arg("sine");
    cmd.assert().success().stdout("Generate sinus\n");

    fs::remove_file("sine_50hz.wav").ok();

    Ok(())
}

#[test]
fn test_gen_noise() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("minidsp")?;
    cmd.arg("gen").arg("noise");
    cmd.assert().success().stdout("Genearate noise!!\n");

    fs::remove_file("noise_1mu_1std.wav").ok();

    Ok(())
}

#[test]
fn test_gen_linear_sweep() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("minidsp")?;
    cmd.arg("gen").arg("sweep");
    cmd.assert().success().stdout("Genearate sweep!!\n");

    fs::remove_file("sweep_1_1_linear.wav").ok();

    Ok(())
}

#[test]
fn test_gen_quadratic_sweep() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("minidsp")?;
    cmd.arg("gen").arg("sweep").arg("-m").arg("quadratic");
    cmd.assert().success().stdout("Genearate sweep!!\n");

    fs::remove_file("sweep_1_1_quadratic.wav").ok();

    Ok(())
}

#[test]
fn test_gen_hyperbolic_sweep() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("minidsp")?;
    cmd.arg("gen").arg("sweep").arg("-m").arg("hyperbolic");
    cmd.assert().success().stdout("Genearate sweep!!\n");

    fs::remove_file("sweep_1_1_hyperbolic.wav").ok();

    Ok(())
}

#[test]
fn test_dsp_add() -> Result<(), Box<dyn std::error::Error>> {
    let _guard = serial_test_guard();
    let mut cmd = Command::cargo_bin("minidsp")?;
    cmd.arg("gen")
        .arg("sine")
        .arg("-f")
        .arg("30")
        .arg("-d")
        .arg("3")
        .arg("-o")
        .arg("sine1.wav");

    cmd.assert().success().stdout("Generate sinus\n");

    let mut cmd2 = Command::cargo_bin("minidsp")?;
    cmd2.arg("gen")
        .arg("sine")
        .arg("-f")
        .arg("30")
        .arg("-d")
        .arg("2")
        .arg("-p")
        .arg("180")
        .arg("-o")
        .arg("sine2.wav");

    cmd2.assert().success().stdout("Generate sinus\n");

    let mut cmd3 = Command::cargo_bin("minidsp")?;
    cmd3.arg("add")
        .arg("-1")
        .arg("sine1.wav")
        .arg("-2")
        .arg("sine2.wav")
        .arg("-o")
        .arg("sine3.wav");

    cmd3.assert().success();

    let mut reader = hound::WavReader::open("sine3.wav")?;

    for sample in reader.samples::<f32>().flatten() {
        assert!(sample.abs() < 1e-6, "Summ not equal zero, it's {sample}");
    }

    fs::remove_file("sine1.wav").ok();
    fs::remove_file("sine2.wav").ok();
    fs::remove_file("sine3.wav").ok();

    Ok(())
}

#[test]
fn test_dsp_sub() -> Result<(), Box<dyn std::error::Error>> {
    let _guard = serial_test_guard();
    let mut cmd = Command::cargo_bin("minidsp")?;
    cmd.arg("gen")
        .arg("sine")
        .arg("-f")
        .arg("30")
        .arg("-d")
        .arg("3")
        .arg("-o")
        .arg("sine1.wav");

    cmd.assert().success().stdout("Generate sinus\n");

    let mut cmd2 = Command::cargo_bin("minidsp")?;
    cmd2.arg("gen")
        .arg("sine")
        .arg("-f")
        .arg("30")
        .arg("-d")
        .arg("2")
        .arg("-o")
        .arg("sine2.wav");

    cmd2.assert().success().stdout("Generate sinus\n");

    let mut cmd3 = Command::cargo_bin("minidsp")?;
    cmd3.arg("sub")
        .arg("-1")
        .arg("sine1.wav")
        .arg("-2")
        .arg("sine2.wav")
        .arg("-o")
        .arg("sine3.wav");

    cmd3.assert().success();

    let mut reader = hound::WavReader::open("sine3.wav")?;

    for sample in reader.samples::<f32>().flatten() {
        assert!(sample.abs() < 1e-6, "Sub not equal zero, it's {sample}");
    }

    fs::remove_file("sine1.wav").ok();
    fs::remove_file("sine2.wav").ok();
    fs::remove_file("sine3.wav").ok();

    Ok(())
}

#[test]
fn test_dsp_mux() -> Result<(), Box<dyn std::error::Error>> {
    let _guard = serial_test_guard();
    let mut cmd = Command::cargo_bin("minidsp")?;
    cmd.arg("gen")
        .arg("sine")
        .arg("-f")
        .arg("30")
        .arg("-d")
        .arg("3")
        .arg("-o")
        .arg("sine1.wav");

    cmd.assert().success().stdout("Generate sinus\n");

    let mut cmd2 = Command::cargo_bin("minidsp")?;
    cmd2.arg("gen")
        .arg("sine")
        .arg("-f")
        .arg("30")
        .arg("-d")
        .arg("2")
        .arg("-a")
        .arg("10")
        .arg("-o")
        .arg("sine2.wav");

    cmd2.assert().success().stdout("Generate sinus\n");

    let mut cmd3 = Command::cargo_bin("minidsp")?;
    cmd3.arg("mux")
        .arg("-1")
        .arg("sine1.wav")
        .arg("-2")
        .arg("sine2.wav")
        .arg("-o")
        .arg("sine3.wav");

    cmd3.assert().success();

    let mut reader = hound::WavReader::open("sine3.wav")?;
    let mut max_val = 0.0;
    for sample_real in reader.samples::<f32>().flatten() {
        if sample_real > max_val {
            max_val = sample_real;
        }
    }

    assert!(
        (max_val - 10.).abs() < 1e-6,
        "Amplitede should be around 10, but it's {max_val}"
    );

    fs::remove_file("sine1.wav").ok();
    fs::remove_file("sine2.wav").ok();
    fs::remove_file("sine3.wav").ok();

    Ok(())
}

#[test]
fn test_dsp_scale() -> Result<(), Box<dyn std::error::Error>> {
    let _guard = serial_test_guard();
    let mut cmd = Command::cargo_bin("minidsp")?;
    cmd.arg("gen")
        .arg("sine")
        .arg("-f")
        .arg("30")
        .arg("-d")
        .arg("3")
        .arg("-o")
        .arg("sine1.wav");

    cmd.assert().success().stdout("Generate sinus\n");

    let mut cmd2 = Command::cargo_bin("minidsp")?;
    cmd2.arg("scale")
        .arg("-s")
        .arg("sine1.wav")
        .arg("-a")
        .arg("10")
        .arg("-o")
        .arg("sine2.wav");

    cmd2.assert().success();

    let mut reader = hound::WavReader::open("sine2.wav")?;
    let mut max_val = 0.0;
    for sample_real in reader.samples::<f32>().flatten() {
        if sample_real > max_val {
            max_val = sample_real;
        }
    }

    assert!(
        (max_val - 10.).abs() < 1e-6,
        "Amplitede should be around 10, but it's {max_val}"
    );

    fs::remove_file("sine1.wav").ok();
    fs::remove_file("sine2.wav").ok();

    Ok(())
}

#[test]
fn test_dsp_move_average() -> Result<(), Box<dyn std::error::Error>> {
    let _guard = serial_test_guard();
    let mut cmd = Command::cargo_bin("minidsp")?;
    cmd.arg("gen")
        .arg("noise")
        .arg("-d")
        .arg("3")
        .arg("-o")
        .arg("noise.wav");

    cmd.assert().success();

    let mut cmd2 = Command::cargo_bin("minidsp")?;
    cmd2.arg("mov-average")
        .arg("-s")
        .arg("noise.wav")
        .arg("-k")
        .arg("13")
        .arg("-o")
        .arg("noise_meaned.wav");

    cmd2.assert().success();

    let mut reader = hound::WavReader::open("noise_meaned.wav")?;
    let mut sig: Vec<f64> = Vec::new();

    for sample_real in reader.samples::<f32>().flatten() {
        if sample_real.is_nan() {
            panic!("{sample_real}")
        }
        sig.push(sample_real as f64);
    }

    let n = sig.len() as f64;
    let mean = sig.iter().sum::<f64>() / n;
    let variance = sig.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n;
    let std = variance.sqrt();

    assert!(
        (mean - 1.).abs() < 0.5,
        "Mean should be around 1, but it's {mean}"
    );

    assert!(std < 0.5, "Std should be around 1, but it's {std}");

    // fs::remove_file("noise.wav").ok();
    // fs::remove_file("noise_meaned.wav").ok();

    Ok(())
}
