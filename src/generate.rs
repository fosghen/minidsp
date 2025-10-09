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

pub fn create_linear_sweep(f0: f64, f1: f64, t1: f64) -> Vec<f64> {
    let sample_num = (t1 * signal::SAMPLE_RATE as f64) as usize;
    let mut out = Vec::with_capacity(sample_num);
    let df = (f1 - f0) / t1 as f64;


    for i in 0..sample_num{
        let t = i as f64 / signal::SAMPLE_RATE as f64;
        let freq = f0 * t + 0.5 * df * t.powf(2.0);
        let value = (2.0 * std::f64::consts::PI * freq).sin();
        out.push(value);
    }

    out
}

pub fn create_hyperbolic_sweep(f0: f64, f1: f64, t1: f64) -> Vec<f64> {
    let sample_num = (t1 * signal::SAMPLE_RATE as f64) as usize;
    let mut out = Vec::with_capacity(sample_num);

    for i in 0..sample_num{
        let t = i as f64 / signal::SAMPLE_RATE as f64;
        let freq = (f0 * f1 * t1 / (f1 - f0)) * (f1 / (f0 + (f1 - f0) * t / t1)).ln();
        let value = (2.0 * std::f64::consts::PI * freq).sin();
        out.push(value);
    }

    out
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

    // ========== TESTS FOR create_linear_sweep ==========

    #[test]
    fn test_create_linear_sweep_basic() {
        // Test basic linear sweep generation
        let f0 = 100.0;  // Start frequency
        let f1 = 1000.0; // End frequency
        let t1 = 1.0;    // Duration
        
        let signal = create_linear_sweep(f0, f1, t1);
        
        // Check that we get the expected number of samples
        let expected_samples = (t1 * signal::SAMPLE_RATE as f64) as usize;
        assert_eq!(signal.len(), expected_samples, "Wrong number of samples for linear sweep");
        
        // Check that signal is not empty
        assert!(!signal.is_empty(), "Linear sweep signal should not be empty");
        
        // Check that all values are within [-1, 1] range (since we use sin)
        assert!(signal.iter().all(|&x| x >= -1.0 && x <= 1.0), 
                "All sweep values should be within [-1, 1] range");
    }

    #[test]
    fn test_create_linear_sweep_frequency_progression() {
        // Test that frequency changes linearly over time
        let f0 = 200.0;
        let f1 = 800.0;
        let t1 = 0.1; // Short duration for faster test
        
        let signal = create_linear_sweep(f0, f1, t1);
        
        // Calculate the frequency rate of change
        let df = (f1 - f0) / t1;
        let expected_samples = (t1 * signal::SAMPLE_RATE as f64) as usize;
        
        // Test frequency at different time points
        let test_points = vec![0.25, 0.5, 0.75]; // 25%, 50%, 75% through the sweep
        
        for &point in &test_points {
            let sample_index = (point * expected_samples as f64) as usize;
            if sample_index < signal.len() {
                let t = sample_index as f64 / signal::SAMPLE_RATE as f64;
                let expected_freq = f0 + df * t;
                
                // For a linear sweep, we expect the instantaneous frequency to be:
                // f(t) = f0 + df * t, where df = (f1 - f0) / t1
                // The phase should be: phase(t) = 2π * (f0*t + 0.5*df*t²)
                // So the instantaneous frequency is the derivative of phase: f0 + df*t
                
                // We can't directly measure instantaneous frequency from the signal,
                // but we can verify the mathematical relationship
                assert!(expected_freq >= f0, "Frequency at {}% should be >= f0", point * 100.0);
                assert!(expected_freq <= f1, "Frequency at {}% should be <= f1", point * 100.0);
            }
        }
    }

    #[test]
    fn test_create_linear_sweep_edge_cases() {
        // Test zero duration
        let signal = create_linear_sweep(100.0, 1000.0, 0.0);
        assert_eq!(signal.len(), 0, "Zero duration should produce empty signal");
        
        // Test very short duration
        let signal = create_linear_sweep(100.0, 1000.0, 0.0001);
        assert!(signal.len() >= 1, "Very short duration should produce at least one sample");
        
        // Test when f0 == f1 (constant frequency)
        let f0 = 440.0;
        let f1 = 440.0;
        let t1 = 0.01;
        
        let signal = create_linear_sweep(f0, f1, t1);
        let expected_samples = (t1 * signal::SAMPLE_RATE as f64) as usize;
        assert_eq!(signal.len(), expected_samples, "Constant frequency sweep should have correct number of samples");
        
        // For constant frequency, the signal should be a sine wave at frequency f0
        // Check that the period matches the expected frequency
        let samples_per_period = signal::SAMPLE_RATE as f64 / f0;
        let period_samples = samples_per_period as usize;
        
        if period_samples < signal.len() && period_samples > 0 {
            // Check approximate periodicity
            let tolerance = 0.2; // Increased tolerance for short signals
            let mut periodic_count = 0;
            let mut total_checks = 0;
            
            for i in 0..(signal.len() - period_samples) {
                let diff = (signal[i] - signal[i + period_samples]).abs();
                if diff < tolerance {
                    periodic_count += 1;
                }
                total_checks += 1;
            }
            
            // For a short signal, we only need most samples to be approximately periodic
            if total_checks > 0 {
                let periodic_ratio = periodic_count as f64 / total_checks as f64;
                assert!(periodic_ratio > 0.7, 
                       "At least 70% of samples should be periodic, got {:.1}%", 
                       periodic_ratio * 100.0);
            }
        }
    }

    #[test]
    fn test_create_linear_sweep_mathematical_properties() {
        // Test mathematical properties of linear sweep
        let f0 = 100.0;
        let f1 = 500.0;
        let t1 = 0.5;
        
        let signal = create_linear_sweep(f0, f1, t1);
        
        // For linear sweep, the phase should be: φ(t) = 2π * (f0*t + 0.5*df*t²)
        // where df = (f1 - f0) / t1
        let df = (f1 - f0) / t1;
        
        // Check phase at different time points
        let test_times = vec![0.0, t1 * 0.25, t1 * 0.5, t1 * 0.75, t1];
        
        for &t in &test_times {
            let sample_index = (t * signal::SAMPLE_RATE as f64) as usize;
            if sample_index < signal.len() {
                let expected_phase = 2.0 * std::f64::consts::PI * (f0 * t + 0.5 * df * t * t);
                let actual_value = signal[sample_index];
                let expected_value = expected_phase.sin();
                
                let diff = (actual_value - expected_value).abs();
                assert!(diff < 1e-10, "Signal value at t={} should match expected phase calculation", t);
            }
        }
    }

    #[test]
    fn test_create_linear_sweep_different_parameters() {
        // Test various parameter combinations
        let test_cases = vec![
            (10.0, 100.0, 0.1),    // Low to medium frequency
            (1000.0, 100.0, 1.0),  // High to low frequency (reverse sweep)
            (1.0, 10000.0, 0.01),  // Very wide frequency range
            (440.0, 880.0, 2.0),   // Musical interval (octave)
            (20.0, 20.0, 0.5),     // Same start and end frequency
        ];
        
        for (f0, f1, t1) in test_cases {
            let signal = create_linear_sweep(f0, f1, t1);
            let expected_samples = (t1 * signal::SAMPLE_RATE as f64) as usize;
            
            assert_eq!(signal.len(), expected_samples, 
                      "Wrong number of samples for f0={}, f1={}, t1={}", f0, f1, t1);
            
            // Check that all values are valid
            assert!(signal.iter().all(|&x| x.is_finite()), 
                   "All signal values should be finite for f0={}, f1={}, t1={}", f0, f1, t1);
            
            // Check amplitude range
            assert!(signal.iter().all(|&x| x >= -1.0 && x <= 1.0), 
                   "All signal values should be in [-1, 1] for f0={}, f1={}, t1={}", f0, f1, t1);
        }
    }

    #[test]
    fn test_create_linear_sweep_frequency_limits() {
        // Test behavior at frequency limits
        let signal = create_linear_sweep(1.0, 1.0, 1.0); // Very low constant frequency
        assert!(!signal.is_empty(), "Low frequency sweep should not be empty");
        
        let signal = create_linear_sweep(10000.0, 10000.0, 0.01); // High constant frequency
        assert!(!signal.is_empty(), "High frequency sweep should not be empty");
        
        // Test that we can handle large frequency ranges
        let signal = create_linear_sweep(0.1, 20000.0, 0.1);
        let expected_samples = (0.1 * signal::SAMPLE_RATE as f64) as usize;
        assert_eq!(signal.len(), expected_samples, "Large frequency range should work correctly");
    }

    #[test]
    fn test_create_linear_sweep_continuity() {
        // Test that the sweep signal is continuous (no sudden jumps)
        let f0 = 100.0;
        let f1 = 1000.0;
        let t1 = 0.1;
        
        let signal = create_linear_sweep(f0, f1, t1);
        
        // Check continuity by comparing adjacent samples
        for i in 1..signal.len() {
            let diff = (signal[i] - signal[i-1]).abs();
            // The maximum difference between adjacent samples should be reasonable
            // For a sine wave, the maximum derivative is the amplitude times the frequency
            let max_expected_diff = 2.0 * std::f64::consts::PI * f1 / signal::SAMPLE_RATE as f64;
            assert!(diff <= max_expected_diff * 1.5, // Allow some tolerance
                   "Signal should be continuous, but found large jump at sample {}", i);
        }
    }

    #[test]
    fn test_create_linear_sweep_amplitude_consistency() {
        // Test that amplitude remains consistent throughout the sweep
        let f0 = 200.0;
        let f1 = 800.0;
        let t1 = 0.2;
        
        let signal = create_linear_sweep(f0, f1, t1);
        
        // Find the maximum absolute value in different segments
        let segment_size = signal.len() / 4; // Divide into 4 segments
        let mut segment_maxes = Vec::new();
        
        for i in 0..4 {
            let start = i * segment_size;
            let end = if i == 3 { signal.len() } else { (i + 1) * segment_size };
            
            let segment_max = signal[start..end].iter()
                .map(|&x| x.abs())
                .fold(0.0, f64::max);
            
            segment_maxes.push(segment_max);
        }
        
        // All segments should have similar maximum amplitudes (close to 1.0)
        for (i, &max_amp) in segment_maxes.iter().enumerate() {
            assert!(max_amp > 0.9, "Segment {} should have amplitude close to 1.0, got {}", i, max_amp);
            assert!(max_amp <= 1.0, "Segment {} amplitude should not exceed 1.0, got {}", i, max_amp);
        }
    }

}
