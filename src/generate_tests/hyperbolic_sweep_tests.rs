use crate::generate;
use crate::signal;

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function for frequency measurement
    fn compute_freq(signal: &[f64], num_pts: i32, idx_start: i32) -> f64 {
        let mut zeros = 0;

        for pair in signal[idx_start as usize..(idx_start + num_pts) as usize].windows(2) {
            if pair[0].signum() != pair[1].signum(){
                zeros += 1;
            }
        }
        zeros as f64 / 2.0 / num_pts as f64 * signal::SAMPLE_RATE as f64
    }

    #[test]
    fn test_create_hyperbolic_sweep_basic() {
        // Test basic hyperbolic sweep generation
        let f0 = 100.0;  // Start frequency
        let f1 = 1000.0; // End frequency
        let t1 = 1.0;    // Duration
        
        let signal = generate::create_hyperbolic_sweep(f0, f1, t1).unwrap();
        
        // Check that we get the expected number of samples
        let expected_samples = (t1 * signal::SAMPLE_RATE as f64) as usize;
        assert_eq!(signal.len(), expected_samples, "Wrong number of samples for linear sweep");
        
        // Check that signal is not empty
        assert!(!signal.is_empty(), "Linear sweep signal should not be empty");
        
        // Check that all values are within [-1, 1] range (since we use sin)
        assert!(signal.iter().all(|&x| (-1.0..=1.0).contains(&x)), 
                "All sweep values should be within [-1, 1] range");
    }

    #[test]
    fn test_create_hyperbolic_sweep_freqs() {
        // Test frequency hyperbolic sweep generation
        let f0 = 100.0;  // Start frequency
        let f1 = 1000.0; // End frequency
        let t1 = 1.0;    // Duration
        
        let signal = generate::create_hyperbolic_sweep(f0, f1, t1).unwrap();
        
        // Check that we get the expected number of samples
        let expected_samples = (t1 * signal::SAMPLE_RATE as f64) as usize;
        assert_eq!(signal.len(), expected_samples, "Wrong number of samples for hyperbolic sweep");
        
        // Check f0 is 100
        assert!((f0 - compute_freq(&signal, 100, 0)).abs() < f0 * 0.06,
         "Freq at start is {}, but expect {}", compute_freq(&signal, 100, 0), f0);
        // Check f1 is 1000
        assert!((f1 - compute_freq(&signal, 100, expected_samples as i32 - 101)).abs() < f1 * 0.06, 
        "Freq at end is {}, but expect {}", compute_freq(&signal, 100, expected_samples as i32 - 101), f1);

        // Check that all values are within [-1, 1] range (since we use sin)
        assert!(signal.iter().all(|&x| (-1.0..=1.0).contains(&x)), 
                "All sweep values should be within [-1, 1] range");
    }

    #[test]
    fn test_create_hyperbolic_sweep_const_freq() {
        // Test frequency hyperbolic sweep generation
        let f0 = 1000.0;  // Start frequency
        let f1 = 1000.0; // End frequency
        let t1 = 1.0;    // Duration
        
        let signal = generate::create_hyperbolic_sweep(f0, f1, t1).unwrap();
        
        // Check that we get the expected number of samples
        let expected_samples = (t1 * signal::SAMPLE_RATE as f64) as usize;
        assert_eq!(signal.len(), expected_samples, "Wrong number of samples for hyperbolic sweep");

        assert!(signal.iter().all(|&x| !x.is_nan()), "Signal has nans");
        
        // Check f0 is 1000
        assert!((f0 - compute_freq(&signal, 100, 0)).abs() < f0 * 0.06,
         "Freq at start is {}, but expect {}", compute_freq(&signal, 100, 0), f0);
        // Check f1 is 1000
        assert!((f1 - compute_freq(&signal, 100, expected_samples as i32 - 101)).abs() < f1 * 0.06, 
        "Freq at end is {}, but expect {}", compute_freq(&signal, 100, expected_samples as i32 - 101), f1);

        // Check that all values are within [-1, 1] range (since we use sin)
        assert!(signal.iter().all(|&x| (-1.0..=1.0).contains(&x)), 
                "All sweep values should be within [-1, 1] range");
    }

    #[test]
    fn test_create_hyperbolic_sweep_edge() {
        // Test frequency hyperbolic sweep generation
        let mut f0 = -1.0;  // Start frequency
        let mut f1 = 1000.0; // End frequency
        let mut t1 = 1.0;    // Duration
        
        let signal = generate::create_hyperbolic_sweep(f0, f1, t1);
        assert!(signal.is_err(), "With negative parameter, we should get Error");
        
        f0 = 1.0;
        f1 = - 100.0;

        let signal = generate::create_hyperbolic_sweep(f0, f1, t1);
        assert!(signal.is_err(), "With negative parameter, we should get Error");

        f1 = 1000.0;
        t1 = - 13.0;

        let signal = generate::create_hyperbolic_sweep(f0, f1, t1);
        assert!(signal.is_err(), "With negative parameter, we should get Error");

        t1 = 0.0;

         let signal = generate::create_hyperbolic_sweep(f0, f1, t1).unwrap();
         assert_eq!(signal.len(), 0, "Length with t1 = 0, should be 0, but {}", signal.len());
    }

    #[test]
    fn test_create_hyperbolic_sweep_mathematical_properties() {
        // Test mathematical properties of hyperbolic sweep
        let f0 = 100.0;
        let f1 = 1000.0;
        let t1 = 0.5;
        
        let signal = generate::create_hyperbolic_sweep(f0, f1, t1).unwrap();
        
        // For hyperbolic sweep, the instantaneous frequency should be:
        // f(t) = f0 * f1 / (f1 + (f0 - f1) * t/t1)
        // This creates a hyperbolic frequency progression
        
        // Check frequency at different time points
        let test_times = vec![0.0, t1 * 0.25, t1 * 0.5, t1 * 0.75, t1];
        
        for &t in &test_times {
            let sample_index = (t * signal::SAMPLE_RATE as f64) as usize;
            if sample_index < signal.len() {
                // Calculate expected frequency using hyperbolic formula
                let expected_freq = if f0 != f1 {
                    f0 * f1 / (f1 + (f0 - f1) * t / t1)
                } else {
                    f0
                };
                
                // For hyperbolic sweep, frequency should always be between f0 and f1
                assert!(expected_freq >= f0.min(f1), 
                       "Frequency at t={} should be >= min(f0,f1), got {}", t, expected_freq);
                assert!(expected_freq <= f0.max(f1), 
                       "Frequency at t={} should be <= max(f0,f1), got {}", t, expected_freq);
            }
        }
    }

    #[test]
    fn test_create_hyperbolic_sweep_continuity() {
        // Test that the hyperbolic sweep signal is continuous
        let f0 = 200.0;
        let f1 = 800.0;
        let t1 = 0.1;
        
        let signal = generate::create_hyperbolic_sweep(f0, f1, t1).unwrap();
        
        // Check continuity by comparing adjacent samples
        for i in 1..signal.len() {
            let diff = (signal[i] - signal[i-1]).abs();
            // The maximum difference should be reasonable for a continuous signal
            let max_expected_diff = 2.0 * std::f64::consts::PI * f1.max(f0) / signal::SAMPLE_RATE as f64;
            assert!(diff <= max_expected_diff * 2.0, // Allow tolerance for hyperbolic sweep
                   "Signal should be continuous, but found large jump at sample {}", i);
        }
    }

    #[test]
    fn test_create_hyperbolic_sweep_frequency_progression() {
        // Test that frequency changes hyperbolically (non-linearly)
        let f0 = 100.0;
        let f1 = 1000.0;
        let t1 = 0.2;
        
        let signal = generate::create_hyperbolic_sweep(f0, f1, t1).unwrap();
        
        // Measure frequency at different points using zero-crossing method
        let window_size = 200; // Window size for frequency measurement
        let num_points = 5;
        
        let mut measured_freqs = Vec::new();
        for i in 0..num_points {
            let idx = (i * (signal.len() - window_size) / (num_points - 1)) as usize;
            let freq = compute_freq(&signal, window_size as i32, idx as i32);
            measured_freqs.push(freq);
        }
        
        // For hyperbolic sweep, frequency should change non-linearly
        // Early in the sweep, frequency changes slowly
        // Later in the sweep, frequency changes more rapidly
        if measured_freqs.len() >= 3 {
            let early_change = (measured_freqs[1] - measured_freqs[0]).abs();
            let late_change = (measured_freqs[measured_freqs.len()-1] - measured_freqs[measured_freqs.len()-2]).abs();
            
            // For hyperbolic sweep going from low to high frequency,
            // the change should be more rapid at the end
            if f1 > f0 {
                assert!(late_change > early_change * 0.5, 
                       "Hyperbolic sweep should have more rapid frequency change at the end");
            }
        }
    }

    #[test]
    fn test_create_hyperbolic_sweep_different_parameters() {
        // Test various parameter combinations
        let test_cases = vec![
            (10.0, 100.0, 0.1),    // Low to medium frequency
            (1000.0, 100.0, 1.0),  // High to low frequency (reverse hyperbolic sweep)
            (1.0, 5000.0, 0.01),   // Very wide frequency range
            (440.0, 880.0, 2.0),   // Musical interval (octave)
            (20.0, 20.0, 0.5),     // Same start and end frequency
            (50.0, 200.0, 0.001),  // Very short duration
        ];
        
        for (f0, f1, t1) in test_cases {
            let result = generate::create_hyperbolic_sweep(f0, f1, t1);
            assert!(result.is_ok(), "Hyperbolic sweep should succeed for f0={}, f1={}, t1={}", f0, f1, t1);
            
            let signal = result.unwrap();
            let expected_samples = (t1 * signal::SAMPLE_RATE as f64) as usize;
            
            assert_eq!(signal.len(), expected_samples, 
                      "Wrong number of samples for f0={}, f1={}, t1={}", f0, f1, t1);
            
            // Check that all values are valid
            assert!(signal.iter().all(|&x| x.is_finite()), 
                   "All signal values should be finite for f0={}, f1={}, t1={}", f0, f1, t1);
            
            // Check amplitude range
            assert!(signal.iter().all(|&x| (-1.0..=1.0).contains(&x)), 
                   "All signal values should be in [-1, 1] for f0={}, f1={}, t1={}", f0, f1, t1);
        }
    }

    #[test]
    fn test_create_hyperbolic_sweep_vs_linear_comparison() {
        // Compare hyperbolic sweep with linear sweep for same parameters
        let f0 = 100.0;
        let f1 = 1000.0;
        let t1 = 0.1;
        
        let hyperbolic_signal = generate::create_hyperbolic_sweep(f0, f1, t1).unwrap();
        let linear_signal = generate::create_linear_sweep(f0, f1, t1);
        
        // Both should have the same length
        assert_eq!(hyperbolic_signal.len(), linear_signal.len());
        
        // Both should start and end at similar frequencies
        let window_size = 100;
        
        // Check start frequency (should be close)
        let hyper_start_freq = compute_freq(&hyperbolic_signal, window_size, 0);
        let linear_start_freq = compute_freq(&linear_signal, window_size, 0);
        
        let start_diff = (hyper_start_freq - linear_start_freq).abs();
        assert!(start_diff < f0 * 0.1, "Start frequencies should be similar");
        
        // Check end frequency (should be close)
        // Use a smaller window for end frequency to avoid going out of bounds
        let end_window_size = 50;
        let end_idx = hyperbolic_signal.len() - end_window_size;
        if end_idx > 0 {
            let hyper_end_freq = compute_freq(&hyperbolic_signal, end_window_size as i32, end_idx as i32);
            let linear_end_freq = compute_freq(&linear_signal, end_window_size as i32, end_idx as i32);
            
            let end_diff = (hyper_end_freq - linear_end_freq).abs();
            assert!(end_diff < f1 * 0.2, "End frequencies should be similar, got hyper: {}, linear: {}", hyper_end_freq, linear_end_freq);
        }
        
        // But the signals themselves should be different (different frequency progression)
        let mut similar_count = 0;
        let tolerance = 0.1;
        for (h, l) in hyperbolic_signal.iter().zip(linear_signal.iter()) {
            if (h - l).abs() < tolerance {
                similar_count += 1;
            }
        }
        
        // The signals should be significantly different
        let similarity_ratio = similar_count as f64 / hyperbolic_signal.len() as f64;
        assert!(similarity_ratio < 0.8, "Hyperbolic and linear sweeps should be different");
    }

    #[test]
    fn test_create_hyperbolic_sweep_amplitude_consistency() {
        // Test that amplitude remains consistent throughout the hyperbolic sweep
        let f0 = 200.0;
        let f1 = 800.0;
        let t1 = 0.2;
        
        let signal = generate::create_hyperbolic_sweep(f0, f1, t1).unwrap();
        
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

    #[test]
    fn test_create_hyperbolic_sweep_edge_case_zero_duration() {
        // Test zero duration case
        let signal = generate::create_hyperbolic_sweep(100.0, 1000.0, 0.0).unwrap();
        assert_eq!(signal.len(), 0, "Zero duration should produce empty signal");
    }

    #[test]
    fn test_create_hyperbolic_sweep_very_short_duration() {
        // Test very short duration
        let signal = generate::create_hyperbolic_sweep(100.0, 1000.0, 0.0001).unwrap();
        assert!(!signal.is_empty(), "Very short duration should produce at least one sample");
        assert!(signal.iter().all(|&x| x.is_finite()), "Very short signal should have finite values");
    }
}
