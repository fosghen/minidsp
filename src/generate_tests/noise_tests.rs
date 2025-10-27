use crate::generate;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_noise_basic() {
        // Test basic noise generation
        let duration = 0.1; // 100ms
        let std = 1.0;
        let mu = 0.0;
        
        let result = generate::create_noise(duration, std, mu);
        assert!(result.is_ok(), "Noise generation should succeed");
        
        let signal = result.unwrap();
        let expected_samples = (duration * crate::signal::SAMPLE_RATE as f64) as usize;
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
            let result = generate::create_noise(duration, std, mu);
            assert!(result.is_ok(), "Noise generation should succeed for duration {}", duration);
            
            let signal = result.unwrap();
            let expected_samples = (duration * crate::signal::SAMPLE_RATE as f64) as usize;
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
            let result = generate::create_noise(duration, std, mu);
            assert!(result.is_ok(), "Noise generation should succeed for std={}, mu={}", std, mu);
            
            let signal = result.unwrap();
            assert_eq!(signal.len(), (duration * crate::signal::SAMPLE_RATE as f64) as usize);
        }
    }

    #[test]
    fn test_create_noise_statistical_properties() {
        // Test statistical properties of the generated noise
        let duration = 1.0; // 1 second for better statistics
        let std = 1.0;
        let mu = 0.0;
        
        let result = generate::create_noise(duration, std, mu);
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
        let result = generate::create_noise(0.0001, 1.0, 0.0);
        assert!(result.is_ok());
        let signal = result.unwrap();
        assert!(!signal.is_empty(), "Should have at least one sample even for very short duration");
        
        // Test zero std (should fail or return constant values)
        let result = generate::create_noise(0.01, 0.0, 1.0);
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
                panic!("Zero std is expected to fail");
            }
        }
        
        // Test negative std (might fail or be handled gracefully)
        let result = generate::create_noise(0.01, -1.0, 0.0);
        // The behavior depends on the rand_distr implementation
        // It might fail or convert to positive std
        match result {
            Ok(_) => {
                // If it succeeds, that's also acceptable behavior
            },
            Err(_) => {
                // If it fails, that's also acceptable
                panic!("Negative std correctly caused an error");
            }
        }
    }

    #[test]
    fn test_create_noise_reproducibility() {
        // Test that multiple calls produce different results (randomness)
        let duration = 0.01;
        let std = 1.0;
        let mu = 0.0;
        
        let signal1 = generate::create_noise(duration, std, mu).unwrap();
        let signal2 = generate::create_noise(duration, std, mu).unwrap();
        
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
        
        let signal = generate::create_noise(duration, std, mu).unwrap();
        
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
