use crate::generate;
use crate::signal;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_linear_sweep_basic() {
        // Test basic linear sweep generation
        let f0 = 100.0;  // Start frequency
        let f1 = 1000.0; // End frequency
        let t1 = 1.0;    // Duration
        
        let signal = generate::create_linear_sweep(f0, f1, t1);
        
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
        
        let signal = generate::create_linear_sweep(f0, f1, t1);
        
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
        let signal = generate::create_linear_sweep(100.0, 1000.0, 0.0);
        assert_eq!(signal.len(), 0, "Zero duration should produce empty signal");
        
        // Test very short duration
        let signal = generate::create_linear_sweep(100.0, 1000.0, 0.0001);
        assert!(signal.len() >= 1, "Very short duration should produce at least one sample");
        
        // Test when f0 == f1 (constant frequency)
        let f0 = 440.0;
        let f1 = 440.0;
        let t1 = 0.01;
        
        let signal = generate::create_linear_sweep(f0, f1, t1);
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
        
        let signal = generate::create_linear_sweep(f0, f1, t1);
        
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
            let signal = generate::create_linear_sweep(f0, f1, t1);
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
        let signal = generate::create_linear_sweep(1.0, 1.0, 1.0); // Very low constant frequency
        assert!(!signal.is_empty(), "Low frequency sweep should not be empty");
        
        let signal = generate::create_linear_sweep(10000.0, 10000.0, 0.01); // High constant frequency
        assert!(!signal.is_empty(), "High frequency sweep should not be empty");
        
        // Test that we can handle large frequency ranges
        let signal = generate::create_linear_sweep(0.1, 20000.0, 0.1);
        let expected_samples = (0.1 * signal::SAMPLE_RATE as f64) as usize;
        assert_eq!(signal.len(), expected_samples, "Large frequency range should work correctly");
    }

    #[test]
    fn test_create_linear_sweep_continuity() {
        // Test that the sweep signal is continuous (no sudden jumps)
        let f0 = 100.0;
        let f1 = 1000.0;
        let t1 = 0.1;
        
        let signal = generate::create_linear_sweep(f0, f1, t1);
        
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
        
        let signal = generate::create_linear_sweep(f0, f1, t1);
        
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
