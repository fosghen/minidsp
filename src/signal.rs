pub const SAMPLE_RATE: u32 = 10000;

use std::f32;
use hound;

pub fn save_wave(signal: &Vec<f64>, filename: &str) -> Result<(), hound::Error> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };

    let mut writer = hound::WavWriter::create(filename, spec).unwrap();
    for value in signal {
        // let amplitude = f32::MAX as f64;
        writer.write_sample(*value as f32)?;
    }
    Ok(())
}

pub fn read_wave(signal: &mut Vec<f64>, filename: &str) -> Result<(), hound::Error> {
    let mut reader = hound::WavReader::open(filename)?;
    
    for sample in reader.samples::<f32>(){
        signal.push(sample? as f64);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::generate;

    use super::*;

    #[test]
    fn test_save_read_wave() {
        let signal = generate::create_sine(133., 0., 0.1, 0.5);
        let fname = "test_o.wav";

        save_wave(&signal, fname).expect("failed to save wav");

        let mut readed_signal: Vec<f64> = Vec::new();

        read_wave(&mut readed_signal, fname).expect("failed to open wav");

        assert!(signal.iter().zip(readed_signal.iter()).all(|(x, y)| (x - y).abs() < 1e-6), "Writed signal not equal readed signal");

        fs::remove_file(fname).ok();
    }
}