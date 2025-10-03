use crate::signal;

pub fn create_sine(freq: f64, phase: f64, duration: f64, amplitude: f64) -> Vec<f64> {
    let sample_num = (duration * signal::SAMPLE_RATE as f64) as usize;
    let mut out = Vec::with_capacity(sample_num);
    for i in 0..sample_num {
        let t = i as f64 / signal::SAMPLE_RATE as f64;
        let value = (2.0 * std::f64::consts::PI * freq * t + phase).sin() * amplitude;
        out.push(value);
    }
    out
}