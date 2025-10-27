use crate::signal;
use rand_distr::{Distribution, Normal};

pub fn create_sine(freq: f64, phase: f64, duration: f64, amplitude: f64) -> Vec<f64> {
    let sample_num = (duration * signal::SAMPLE_RATE as f64) as usize;
    let mut out = Vec::with_capacity(sample_num);

    for i in 0..sample_num {
        let t = i as f64 / signal::SAMPLE_RATE as f64;
        let value = amplitude * (2.0 * std::f64::consts::PI * freq * t + phase).sin();
        out.push(value);
    }

    out
}

pub fn create_noise(
    duration: f64,
    std: f64,
    mu: f64,
) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let sample_num = (duration * signal::SAMPLE_RATE as f64) as usize;
    let mut out = Vec::with_capacity(sample_num);

    let normal = Normal::new(mu, std)?;
    let mut rng = rand::rng();

    for _ in 0..sample_num {
        out.push(normal.sample(&mut rng));
    }

    Ok(out)
}

pub fn create_linear_sweep(f0: f64, f1: f64, t1: f64) -> Vec<f64> {
    let sample_num = (t1 * signal::SAMPLE_RATE as f64) as usize;
    let mut out = Vec::with_capacity(sample_num);
    let df = (f1 - f0) / t1;

    for i in 0..sample_num {
        let t = i as f64 / signal::SAMPLE_RATE as f64;
        let freq = f0 * t + 0.5 * df * t.powf(2.0);
        let value = (2.0 * std::f64::consts::PI * freq).sin();
        out.push(value);
    }
    out
}

pub fn create_hyperbolic_sweep(f0: f64, f1: f64, t1: f64) -> Result<Vec<f64>, String> {
    if f0 < 0.0 || f1 < 0.0 || t1 < 0.0 {
        return Err("Parameters f0, f1, t1 must be positive".to_string());
    }
    let sample_num = (t1 * signal::SAMPLE_RATE as f64) as usize;
    let mut out = Vec::with_capacity(sample_num);

    for i in 0..sample_num {
        let t = i as f64 / signal::SAMPLE_RATE as f64;
        let freq = if f0 != f1 {
            (f0 * f1 * t1 / (f0 - f1)) * (f0 / (f1 + (f0 - f1) * t / t1)).ln()
        } else {
            f0 * t
        };
        let value = (2.0 * std::f64::consts::PI * freq).sin();
        out.push(value);
    }

    Ok(out)
}

pub fn create_quadratic_sweep(f0: f64, f1: f64, t1: f64, vertex_zero: bool) -> Vec<f64> {
    let sample_num = (t1 * signal::SAMPLE_RATE as f64) as usize;
    let mut out = Vec::with_capacity(sample_num);

    let k = (f1 - f0) / t1.powf(2.0);

    for i in 0..sample_num {
        let t = i as f64 / signal::SAMPLE_RATE as f64;
        let freq = if vertex_zero {
            f0 * t + k / 3.0 * t.powf(3.0)
        } else {
            f0 * t - k / 3.0 * t.powf(3.0) + 2.0 * k * t.powf(2.0)
        };
        let value = (2.0 * std::f64::consts::PI * freq).sin();
        out.push(value);
    }

    out
}

// Include test modules
#[cfg(test)]
mod tests {
    // Include all test modules
    include!("generate_tests/mod.rs");
}
