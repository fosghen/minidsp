use assert_cmd::prelude::*;
use std::process::Command;
use std::fs;

#[test]
fn test_general_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("minidsp")?;
    cmd.arg("-h");
    cmd.assert()
        .success()
        .stdout(
r#"Make some dsp with .wav files

Usage: minidsp <COMMAND>

Commands:
  gen   Generare signal
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
"#);

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
  -p, --phase <PHASE>                phase in radians [default: 0]
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
fn test_gen_sine() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("minidsp")?;
    cmd.arg("gen").arg("sine");
    cmd.assert()
        .success()
        .stdout("Generate sinus\n");

    fs::remove_file("sine_50hz.wav").ok();

    Ok(())
}

#[test]
fn test_gen_noise() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("minidsp")?;
    cmd.arg("gen").arg("noise");
    cmd.assert()
        .success()
        .stdout("Genearate noise!!\n");

    fs::remove_file("noise_1mu_1std.wav").ok();

    Ok(())
}

#[test]
fn test_gen_linear_sweep() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("minidsp")?;
    cmd.arg("gen").arg("sweep");
    cmd.assert()
        .success()
        .stdout("Genearate sweep!!\n");

    fs::remove_file("sweep_1_1_linear.wav").ok();

    Ok(())
}

#[test]
fn test_gen_quadratic_sweep() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("minidsp")?;
    cmd.arg("gen").arg("sweep").arg("-m").arg("quadratic");
    cmd.assert()
        .success()
        .stdout("Genearate sweep!!\n");

    fs::remove_file("sweep_1_1_quadratic.wav").ok();

    Ok(())
}

#[test]
fn test_gen_hyperbolic_sweep() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("minidsp")?;
    cmd.arg("gen").arg("sweep").arg("-m").arg("hyperbolic");
    cmd.assert()
        .success()
        .stdout("Genearate sweep!!\n");

    fs::remove_file("sweep_1_1_hyperbolic.wav").ok();

    Ok(())
}