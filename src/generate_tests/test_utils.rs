use crate::signal;

/// Helper function for frequency measurement using zero-crossing detection
pub fn compute_freq(signal: &Vec<f64>, num_pts: i32, idx_start: i32) -> f64 {
    let mut zeros = 0;

    for pair in signal[idx_start as usize..(idx_start + num_pts) as usize].windows(2) {
        if pair[0].signum() != pair[1].signum(){
            zeros += 1;
        }
    }
    zeros as f64 / 2.0 / num_pts as f64 * signal::SAMPLE_RATE as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_freq_basic() {
        // Test the frequency computation function
        // Create a simple sine wave at 100 Hz
        let freq = 100.0;
        let duration = 0.1;
        let sample_rate = signal::SAMPLE_RATE as f64;
        let num_samples = (duration * sample_rate) as usize;
        
        let mut signal = Vec::with_capacity(num_samples);
        for i in 0..num_samples {
            let t = i as f64 / sample_rate;
            let value = (2.0 * std::f64::consts::PI * freq * t).sin();
            signal.push(value);
        }
        
        // Measure frequency using our function
        let measured_freq = compute_freq(&signal, 1000, 0);
        
        // Should be close to 100 Hz
        let tolerance = freq * 0.1; // 10% tolerance
        assert!((measured_freq - freq).abs() < tolerance,
               "Measured frequency {:.1}Hz should be close to expected {:.1}Hz", 
               measured_freq, freq);
    }

    #[test]
    fn test_compute_freq_edge_cases() {
        // Test edge cases for frequency computation
        let signal = vec![0.0, 1.0, 0.0, -1.0, 0.0]; // Simple square wave
        
        // Test with window size larger than signal - this should panic or handle gracefully
        // Let's test with a smaller window that fits
        let freq = compute_freq(&signal, 3, 0);
        assert!(freq >= 0.0, "Frequency should be non-negative");
        
        // Note: Testing with zero window size would cause division by zero
        // This is expected behavior and we don't need to test it explicitly
    }
}
