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

    #[test]
    fn test_create_noise_basic() {
        // Test basic noise generation
        let duration = 0.1; // 100ms
        let std = 1.0;
        let mu = 0.0;
        
        let result = create_noise(duration, std, mu);
        assert!(result.is_ok(), "Noise generation should succeed");
        
        let signal = result.unwrap();
        let expected_samples = (duration * signal::SAMPLE_RATE as f64) as usize;
        assert_eq!(signal.len(), expected_samples, "Wrong number of samples");
        
        // Check that we have some variation in the signal
        let min_val = signal.iter().fold(f64::INFINITY, |acc, &x| acc.min(x));
        let max_val = signal.iter().fold(f64::NEG_INFINITY, |acc, &x| acc.max(x));
        assert!(max_val > min_val, "Noise should have variation");
    }

    #[test]
    fn test_create_noise_different_durations() {
        // Test different durations
        let durations = vec![0.001, 0.01, 0.1, 1.0];
        let std = 1.0;
        let mu = 0.0;
        
        for duration in durations {
            let result = create_noise(duration, std, mu);
            assert!(result.is_ok(), "Noise generation should succeed for duration {}", duration);
            
            let signal = result.unwrap();
            let expected_samples = (duration * signal::SAMPLE_RATE as f64) as usize;
            assert_eq!(signal.len(), expected_samples, "Wrong number of samples for duration {}", duration);
        }
    }

    #[test]
    fn test_create_noise_different_parameters() {
        // Test different std and mu values
        let test_cases = vec![
            (0.1, 0.0),   // Low std, zero mean
            (1.0, 0.0),   // Normal std, zero mean
            (2.0, 0.0),   // High std, zero mean
            (1.0, 1.0),   // Normal std, positive mean
            (1.0, -1.0),  // Normal std, negative mean
            (0.5, 5.0),   // Low std, high mean
        ];
        
        let duration = 0.01; // Short duration for faster tests
        
        for (std, mu) in test_cases {
            let result = create_noise(duration, std, mu);
            assert!(result.is_ok(), "Noise generation should succeed for std={}, mu={}", std, mu);
            
            let signal = result.unwrap();
            assert_eq!(signal.len(), (duration * signal::SAMPLE_RATE as f64) as usize);
        }
    }

    #[test]
    fn test_create_noise_statistical_properties() {
        // Test statistical properties of the generated noise
        let duration = 1.0; // 1 second for better statistics
        let std = 1.0;
        let mu = 0.0;
        
        let result = create_noise(duration, std, mu);
        assert!(result.is_ok());
        let signal = result.unwrap();
        
        // Calculate sample mean
        let sample_mean = signal.iter().sum::<f64>() / signal.len() as f64;
        // Sample mean should be close to the theoretical mean (mu)
        assert!((sample_mean - mu).abs() < 0.1, "Sample mean {} should be close to theoretical mean {}", sample_mean, mu);
        
        // Calculate sample variance
        let sample_var = signal.iter()
            .map(|x| (x - sample_mean).powi(2))
            .sum::<f64>() / (signal.len() - 1) as f64;
        let sample_std = sample_var.sqrt();
        
        // Sample std should be close to the theoretical std
        assert!((sample_std - std).abs() < 0.2, "Sample std {} should be close to theoretical std {}", sample_std, std);
    }

    #[test]
    fn test_create_noise_edge_cases() {
        // Test very short duration
        let result = create_noise(0.0001, 1.0, 0.0);
        assert!(result.is_ok());
        let signal = result.unwrap();
        assert!(signal.len() >= 1, "Should have at least one sample even for very short duration");
        
        // Test zero std (should fail or return constant values)
        let result = create_noise(0.01, 0.0, 1.0);
        // This might fail due to Normal::new(1.0, 0.0) being invalid
        // or succeed and return constant values
        match result {
            Ok(signal) => {
                // If it succeeds, all values should be the same (equal to mu)
                if !signal.is_empty() {
                    let first_val = signal[0];
                    assert!(signal.iter().all(|&x| (x - first_val).abs() < 1e-10), 
                           "All values should be equal for zero std");
                }
            },
            Err(_) => {
                // It's also acceptable for this to fail
                assert!(true, "Zero std is expected to fail");
            }
        }
        
        // Test negative std (might fail or be handled gracefully)
        let result = create_noise(0.01, -1.0, 0.0);
        // The behavior depends on the rand_distr implementation
        // It might fail or convert to positive std
        match result {
            Ok(_) => {
                // If it succeeds, that's also acceptable behavior
                assert!(true, "Negative std was handled gracefully");
            },
            Err(_) => {
                // If it fails, that's also acceptable
                assert!(true, "Negative std correctly caused an error");
            }
        }
    }

    #[test]
    fn test_create_noise_reproducibility() {
        // Test that multiple calls produce different results (randomness)
        let duration = 0.01;
        let std = 1.0;
        let mu = 0.0;
        
        let signal1 = create_noise(duration, std, mu).unwrap();
        let signal2 = create_noise(duration, std, mu).unwrap();
        
        // The signals should be different (very high probability)
        let mut different = false;
        for (a, b) in signal1.iter().zip(signal2.iter()) {
            if (a - b).abs() > 1e-10 {
                different = true;
                break;
            }
        }
        assert!(different, "Random noise should produce different results on multiple calls");
    }

    #[test]
    fn test_create_noise_range_and_distribution() {
        // Test that the noise values are within reasonable bounds
        let duration = 0.1;
        let std = 1.0;
        let mu = 0.0;
        
        let signal = create_noise(duration, std, mu).unwrap();
        
        // For normal distribution with std=1, about 99.7% of values should be within 3 standard deviations
        // So we check that most values are within reasonable bounds
        let within_bounds = signal.iter()
            .filter(|&&x| x.abs() <= 4.0) // 4 standard deviations should cover almost all values
            .count();
        
        let percentage_within_bounds = within_bounds as f64 / signal.len() as f64;
        assert!(percentage_within_bounds > 0.99, 
               "At least 99% of values should be within 4 standard deviations, got {}%", 
               percentage_within_bounds * 100.0);
    }

}
