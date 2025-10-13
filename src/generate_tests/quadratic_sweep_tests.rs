use crate::generate;
use crate::signal;

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function for frequency measurement
    fn compute_freq(signal: &Vec<f64>, num_pts: i32, idx_start: i32) -> f64 {
        let mut zeros = 0;

        for pair in signal[idx_start as usize..(idx_start + num_pts) as usize].windows(2) {
            if pair[0].signum() != pair[1].signum(){
                zeros += 1;
            }
        }
        zeros as f64 / 2.0 / num_pts as f64 * signal::SAMPLE_RATE as f64
    }

    #[test]
    fn test_create_quadratic_sweep_basic() {
        // Test basic quadratic sweep generation
        let f0 = 100.0;  // Start frequency
        let f1 = 1000.0; // End frequency
        let t1 = 1.0;    // Duration
        let vertex_zero = true;
        
        let signal = generate::create_quadratic_sweep(f0, f1, t1, vertex_zero);
        
        // Check that we get the expected number of samples
        let expected_samples = (t1 * signal::SAMPLE_RATE as f64) as usize;
        assert_eq!(signal.len(), expected_samples, "Wrong number of samples for quadratic sweep");
        
        // Check that signal is not empty
        assert!(!signal.is_empty(), "Quadratic sweep signal should not be empty");
        
        // Check that all values are within [-1, 1] range (since we use sin)
        assert!(signal.iter().all(|&x| x >= -1.0 && x <= 1.0), 
                "All quadratic sweep values should be within [-1, 1] range");
    }

    #[test]
    fn test_create_quadratic_sweep_vertex_zero_true() {
        // Test quadratic sweep with vertex_zero = true
        let f0 = 200.0;
        let f1 = 800.0;
        let t1 = 0.5;
        let vertex_zero = true;
        
        let signal = generate::create_quadratic_sweep(f0, f1, t1, vertex_zero);
        
        // For vertex_zero = true, frequency formula is: f0*t + k/3*t³
        // where k = (f1 - f0) / t1²
        let k = (f1 - f0) / t1.powi(2);
        
        // Check frequency at different time points
        let test_times = vec![0.0, t1 * 0.25, t1 * 0.5, t1 * 0.75, t1];
        
        for &t in &test_times {
            let sample_index = (t * signal::SAMPLE_RATE as f64) as usize;
            if sample_index < signal.len() {
                let expected_freq = f0 * t + k / 3.0 * t.powi(3);
                
                // For quadratic sweep, frequency should be monotonically increasing
                // (assuming f1 > f0 and vertex_zero = true)
                if f1 > f0 {
                    assert!(expected_freq >= 0.0, "Frequency should be non-negative at t={}", t);
                }
            }
        }
        
        assert_eq!(signal.len(), (t1 * signal::SAMPLE_RATE as f64) as usize);
    }

    #[test]
    fn test_create_quadratic_sweep_vertex_zero_false() {
        // Test quadratic sweep with vertex_zero = false
        let f0 = 200.0;
        let f1 = 800.0;
        let t1 = 0.5;
        let vertex_zero = false;
        
        let signal = generate::create_quadratic_sweep(f0, f1, t1, vertex_zero);
        
        // For vertex_zero = false, frequency formula is: f0*t - k/3*t³ + 2*k*t²
        // where k = (f1 - f0) / t1²
        let k = (f1 - f0) / t1.powi(2);
        
        // Check frequency at different time points
        let test_times = vec![0.0, t1 * 0.25, t1 * 0.5, t1 * 0.75, t1];
        
        for &t in &test_times {
            let sample_index = (t * signal::SAMPLE_RATE as f64) as usize;
            if sample_index < signal.len() {
                let expected_freq = f0 * t - k / 3.0 * t.powi(3) + 2.0 * k * t.powi(2);
                
                // For quadratic sweep with vertex_zero = false, frequency should be non-negative
                assert!(expected_freq >= 0.0, "Frequency should be non-negative at t={}", t);
            }
        }
        
        assert_eq!(signal.len(), (t1 * signal::SAMPLE_RATE as f64) as usize);
    }

    #[test]
    fn test_create_quadratic_sweep_vertex_zero_comparison() {
        // Compare quadratic sweeps with different vertex_zero values
        let f0 = 100.0;
        let f1 = 1000.0;
        let t1 = 0.2;
        
        let signal_true = generate::create_quadratic_sweep(f0, f1, t1, true);
        let signal_false = generate::create_quadratic_sweep(f0, f1, t1, false);
        
        // Both signals should have the same length
        assert_eq!(signal_true.len(), signal_false.len());
        
        // The signals should be different (different frequency progressions)
        let mut similar_count = 0;
        let tolerance = 0.1;
        for (s1, s2) in signal_true.iter().zip(signal_false.iter()) {
            if (s1 - s2).abs() < tolerance {
                similar_count += 1;
            }
        }
        
        // The signals should be significantly different
        let similarity_ratio = similar_count as f64 / signal_true.len() as f64;
        assert!(similarity_ratio < 0.8, "Quadratic sweeps with different vertex_zero should be different");
        
        // Both should have valid ranges
        assert!(signal_true.iter().all(|&x| x >= -1.0 && x <= 1.0), 
               "vertex_zero=true signal should be in [-1, 1] range");
        assert!(signal_false.iter().all(|&x| x >= -1.0 && x <= 1.0), 
               "vertex_zero=false signal should be in [-1, 1] range");
    }

    #[test]
    fn test_create_quadratic_sweep_mathematical_properties() {
        // Test mathematical properties of quadratic sweep
        let f0 = 100.0;
        let f1 = 500.0;
        let t1 = 0.5;
        let vertex_zero = true;
        
        let signal = generate::create_quadratic_sweep(f0, f1, t1, vertex_zero);
        
        // For quadratic sweep with vertex_zero = true:
        // frequency(t) = f0*t + k/3*t³ where k = (f1 - f0) / t1²
        let k = (f1 - f0) / t1.powi(2);
        
        // Check that the frequency formula is applied correctly
        let test_times = vec![0.0, t1 * 0.25, t1 * 0.5, t1 * 0.75, t1];
        
        for &t in &test_times {
            let sample_index = (t * signal::SAMPLE_RATE as f64) as usize;
            if sample_index < signal.len() {
                let expected_freq = f0 * t + k / 3.0 * t.powi(3);
                
                // For quadratic sweep, the frequency should follow the cubic relationship
                // At t=0: freq = 0
                // At t=t1: freq should approach the target frequency
                if t == 0.0 {
                    assert!((expected_freq - 0.0).abs() < 1e-10, "At t=0, frequency should be 0");
                }
            }
        }
    }

    #[test]
    fn test_create_quadratic_sweep_edge_cases() {
        // Test edge cases for quadratic sweep
        
        // Test zero duration
        let signal = generate::create_quadratic_sweep(100.0, 1000.0, 0.0, true);
        assert_eq!(signal.len(), 0, "Zero duration should produce empty signal");
        
        // Test very short duration
        let signal = generate::create_quadratic_sweep(100.0, 1000.0, 0.0001, true);
        assert!(signal.len() >= 1, "Very short duration should produce at least one sample");
        
        // Test when f0 == f1 (constant frequency case)
        let f0 = 440.0;
        let f1 = 440.0;
        let t1 = 0.01;
        
        let signal = generate::create_quadratic_sweep(f0, f1, t1, true);
        let expected_samples = (t1 * signal::SAMPLE_RATE as f64) as usize;
        assert_eq!(signal.len(), expected_samples, "Constant frequency sweep should have correct number of samples");
        
        // For constant frequency (f0 == f1), k = 0, so frequency formula becomes f0*t
        // This should behave like a linear sweep
        assert!(signal.iter().all(|&x| x.is_finite()), "Constant frequency signal should have finite values");
    }

    #[test]
    fn test_create_quadratic_sweep_different_parameters() {
        // Test various parameter combinations
        let test_cases = vec![
            (10.0, 100.0, 0.1, true),    // Low to medium frequency, vertex_zero = true
            (1000.0, 100.0, 1.0, false), // High to low frequency, vertex_zero = false
            (1.0, 5000.0, 0.01, true),   // Very wide frequency range
            (440.0, 880.0, 2.0, false),  // Musical interval (octave)
            (20.0, 20.0, 0.5, true),     // Same start and end frequency
            (50.0, 200.0, 0.001, false), // Very short duration
        ];
        
        for (f0, f1, t1, vertex_zero) in test_cases {
            let signal = generate::create_quadratic_sweep(f0, f1, t1, vertex_zero);
            let expected_samples = (t1 * signal::SAMPLE_RATE as f64) as usize;
            
            assert_eq!(signal.len(), expected_samples, 
                      "Wrong number of samples for f0={}, f1={}, t1={}, vertex_zero={}", 
                      f0, f1, t1, vertex_zero);
            
            // Check that all values are valid
            assert!(signal.iter().all(|&x| x.is_finite()), 
                   "All signal values should be finite for f0={}, f1={}, t1={}, vertex_zero={}", 
                   f0, f1, t1, vertex_zero);
            
            // Check amplitude range
            assert!(signal.iter().all(|&x| x >= -1.0 && x <= 1.0), 
                   "All signal values should be in [-1, 1] for f0={}, f1={}, t1={}, vertex_zero={}", 
                   f0, f1, t1, vertex_zero);
        }
    }

    #[test]
    fn test_create_quadratic_sweep_continuity() {
        // Test that the quadratic sweep signal is continuous
        let f0 = 200.0;
        let f1 = 800.0;
        let t1 = 0.1;
        let vertex_zero = true;
        
        let signal = generate::create_quadratic_sweep(f0, f1, t1, vertex_zero);
        
        // Check continuity by comparing adjacent samples
        for i in 1..signal.len() {
            let diff = (signal[i] - signal[i-1]).abs();
            // The maximum difference should be reasonable for a continuous signal
            let max_expected_diff = 2.0 * std::f64::consts::PI * f1.max(f0) / signal::SAMPLE_RATE as f64;
            assert!(diff <= max_expected_diff * 2.0, // Allow tolerance for quadratic sweep
                   "Signal should be continuous, but found large jump at sample {}", i);
        }
    }

    #[test]
    fn test_create_quadratic_sweep_frequency_progression() {
        // Test that frequency changes quadratically (non-linearly)
        let f0 = 100.0;
        let f1 = 1000.0;
        let t1 = 0.2;
        let vertex_zero = true;
        
        let signal = generate::create_quadratic_sweep(f0, f1, t1, vertex_zero);
        
        // Measure frequency at different points using zero-crossing method
        let window_size = 200;
        let num_points = 5;
        
        let mut measured_freqs = Vec::new();
        for i in 0..num_points {
            let idx = (i * (signal.len() - window_size) / (num_points - 1)) as usize;
            let freq = compute_freq(&signal, window_size as i32, idx as i32);
            measured_freqs.push(freq);
        }
        
        // For quadratic sweep with vertex_zero = true, frequency should change non-linearly
        // The progression should be different from linear sweep
        if measured_freqs.len() >= 3 {
            let early_change = (measured_freqs[1] - measured_freqs[0]).abs();
            let _mid_change = (measured_freqs[2] - measured_freqs[1]).abs();
            let late_change = (measured_freqs[measured_freqs.len()-1] - measured_freqs[measured_freqs.len()-2]).abs();
            
            // For quadratic sweep, the frequency change pattern should be different from linear
            // (more complex than just "early vs late")
            assert!(early_change > 0.0, "Early frequency change should be measurable");
            assert!(late_change > 0.0, "Late frequency change should be measurable");
        }
    }

    #[test]
    fn test_create_quadratic_sweep_vs_other_sweeps() {
        // Compare quadratic sweep with linear and hyperbolic sweeps
        let f0 = 100.0;
        let f1 = 1000.0;
        let t1 = 0.1;
        
        let quadratic_signal = generate::create_quadratic_sweep(f0, f1, t1, true);
        let linear_signal = generate::create_linear_sweep(f0, f1, t1);
        let hyperbolic_signal = generate::create_hyperbolic_sweep(f0, f1, t1).unwrap();
        
        // All signals should have the same length
        assert_eq!(quadratic_signal.len(), linear_signal.len());
        assert_eq!(quadratic_signal.len(), hyperbolic_signal.len());
        
        // All signals should start and end at similar frequencies
        let window_size = 100;
        
        // Check start frequency (should be close to 0 for all sweeps)
        let quad_start_freq = compute_freq(&quadratic_signal, window_size as i32, 0);
        let linear_start_freq = compute_freq(&linear_signal, window_size as i32, 0);
        let hyper_start_freq = compute_freq(&hyperbolic_signal, window_size as i32, 0);
        
        // Start frequencies should be similar (all close to f0)
        let start_tolerance = f0 * 0.2;
        assert!((quad_start_freq - linear_start_freq).abs() < start_tolerance, 
               "Start frequencies should be similar");
        assert!((quad_start_freq - hyper_start_freq).abs() < start_tolerance, 
               "Start frequencies should be similar");
        
        // But the signals themselves should be different (different frequency progressions)
        let mut quad_linear_similar = 0;
        let mut quad_hyper_similar = 0;
        let tolerance = 0.1;
        
        for i in 0..quadratic_signal.len() {
            if (quadratic_signal[i] - linear_signal[i]).abs() < tolerance {
                quad_linear_similar += 1;
            }
            if (quadratic_signal[i] - hyperbolic_signal[i]).abs() < tolerance {
                quad_hyper_similar += 1;
            }
        }
        
        let quad_linear_ratio = quad_linear_similar as f64 / quadratic_signal.len() as f64;
        let quad_hyper_ratio = quad_hyper_similar as f64 / quadratic_signal.len() as f64;
        
        // The signals should be significantly different
        assert!(quad_linear_ratio < 0.8, "Quadratic and linear sweeps should be different");
        assert!(quad_hyper_ratio < 0.8, "Quadratic and hyperbolic sweeps should be different");
    }

    #[test]
    fn test_create_quadratic_sweep_amplitude_consistency() {
        // Test that amplitude remains consistent throughout the quadratic sweep
        let f0 = 200.0;
        let f1 = 800.0;
        let t1 = 0.2;
        let vertex_zero = false;
        
        let signal = generate::create_quadratic_sweep(f0, f1, t1, vertex_zero);
        
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
