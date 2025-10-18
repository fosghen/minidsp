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
        let amplitude = f32::MAX as f64;
        writer.write_sample((value * amplitude) as f32)?;
    }
    Ok(())
}

