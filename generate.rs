mod signal;

use libm;

fn create_sine(freq: f64, phase: f64, duration: f64, amplitude: f64) -> Vec<f64> {
    let sample_num = (duration * signal::SAMPLE_RATE as f64) as usize;
    let mut signal: [f64; sample_num];

    for i in 0..sample_num {
        signal[i] = libm::sine(2. * libm::PI * freq * i as f64 / signal::SAMPLE_RATE as f64 + phase) * amplitude;
    }

    signal.to_vec()
}