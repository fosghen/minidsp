use crate::signal;
use rand_distr::{Normal, Distribution, NormalError};

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

pub fn create_noise(duration: f64, std: f64, mu: f64) -> Result<Vec<f64>, NormalError> {
    let sample_num = (duration * signal::SAMPLE_RATE as f64) as usize;
    let mut out = Vec::with_capacity(sample_num);

    let normal = Normal::new(mu, std)?;
    let mut rng =  rand::rng();

    for _ in 0..sample_num{
        out.push(normal.sample(&mut rng));
    }

    Ok(out)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_sine_basic() {
        // Test basic sine wave generation
        let freq = 440.0; // A4 note
        let phase = 0.0;
        let duration = 0.1; // 100ms
        let amplitude = 1.0;
        
        let signal = create_sine(freq, phase, duration, amplitude);
        
        // Check that we get the expected number of samples
        let expected_samples = (duration * signal::SAMPLE_RATE as f64) as usize;
        assert_eq!(signal.len(), expected_samples);
        
        // Check that first sample is 0 (since phase = 0)
        assert!((signal[0]).abs() < 1e-10, "First sample should be approximately 0");
        
        // Check that amplitude is correct (find max value)
        match signal.iter().max_by(|x, y| x.partial_cmp(y).unwrap()) {
            Some(max_val) => assert!((max_val - amplitude).abs() < 0.1,
             "Maximum amplitude should be close to {} but got {}", amplitude, max_val),

            None => assert_eq!(duration, 0.0, "Signal is empty, but it should be not"),
        }


    }

    #[test]
    fn test_create_sine_phase() {
        // Test with non-zero phase
        let freq = 100.0;
        let phase = std::f64::consts::PI / 2.0; // 90 degrees
        let duration = 0.01; // 10ms
        let amplitude = 0.5;
        
        let signal = create_sine(freq, phase, duration, amplitude);
        
        // With phase = π/2, the first sample should be amplitude * sin(π/2) = amplitude
        assert!((signal[0] - amplitude).abs() < 1e-10, "First sample with phase π/2 should be amplitude");
    }

    #[test]
    fn test_create_sine_frequency() {
        // Test different frequencies
        let frequencies = vec![1.0, 10.0, 100.0, 1000.0];
        let duration = 0.1;
        
        for freq in frequencies {
            let signal = create_sine(freq, 0.0, duration, 1.0);
            let expected_samples = (duration * signal::SAMPLE_RATE as f64) as usize;
            assert_eq!(signal.len(), expected_samples, "Wrong number of samples for frequency {}", freq);
        }
    }

    #[test]
    fn test_create_sine_amplitude() {
        // Test different amplitudes
        let amplitudes = vec![0.1, 0.5, 1.0, 2.0];
        let freq = 100.0;
        let duration = 0.01;

        for amplitude in amplitudes {
            let signal = create_sine(freq, 0.0, duration, amplitude);
        
            match signal.iter().max_by(|x, y| x.partial_cmp(y).unwrap()) {
                Some(max_val) => assert!((max_val - amplitude).abs() < 0.1,
                 "Maximum amplitude should be close to {} but got {}", amplitude, max_val),
    
                None => assert_eq!(duration, 0.0, "Signal is empty, but it should be not"),
            }
        }
    }

    #[test]
    fn test_create_sine_duration() {
        // Test different durations
        let durations = vec![0.001, 0.01, 0.1, 1.0];
        let freq = 100.0;
        let amplitude = 1.0;
        
        for duration in durations {
            let signal = create_sine(freq, 0.0, duration, amplitude);
            let expected_samples = (duration * signal::SAMPLE_RATE as f64) as usize;
            assert_eq!(signal.len(), expected_samples, "Wrong number of samples for duration {}", duration);
        }
    }

    #[test]
    fn test_create_sine_edge_cases() {
        // Test zero amplitude
        let signal = create_sine(100.0, 0.0, 0.01, 0.0);
        assert!(signal.iter().all(|&x| x.abs() < 1e-10), "All samples should be zero for zero amplitude");
        
        // Test very short duration
        let signal = create_sine(100.0, 0.0, 0.0001, 1.0);
        assert!(signal.len() >= 1, "Should have at least one sample even for very short duration");
        
        // Test very low frequency
        let signal = create_sine(0.1, 0.0, 1.0, 1.0);
        assert_eq!(signal.len(), signal::SAMPLE_RATE as usize, "Wrong number of samples for 1 second");
    }

    #[test]
    fn test_create_sine_mathematical_properties() {
        // Test that the sine wave has the correct mathematical properties
        let freq = 100.0;
        let phase = 0.0;
        let duration = 0.1;
        let amplitude = 1.0;
        
        let signal = create_sine(freq, phase, duration, amplitude);
        
        // Check periodicity: for freq=100Hz, we should have 10 periods in 0.1s
        let samples_per_period = signal::SAMPLE_RATE as f64 / freq;
        let period_samples = samples_per_period as usize;
        
        // Check that the signal is approximately periodic
        for i in 0..(signal.len() - period_samples) {
            let diff = (signal[i] - signal[i + period_samples]).abs();
            assert!(diff < 0.1, "Signal should be periodic, but found large difference at sample {}", i);
        }
    }

}
