pub fn add_signal(sig1: &[f64], sig2: &[f64]) -> Vec<f64> {
    let mut sig_new: Vec<f64> = Vec::new();
    let length = if sig1.len() < sig2.len() {
        sig1.len()
    } else {
        sig2.len()
    };

    for i in 0..length {
        sig_new.push(sig1[i] + sig2[i]);
    }

    sig_new
}

pub fn sub_signal(sig1: &[f64], sig2: &[f64]) -> Vec<f64> {
    let mut sig_new: Vec<f64> = Vec::new();
    let length = if sig1.len() < sig2.len() {
        sig1.len()
    } else {
        sig2.len()
    };

    for i in 0..length {
        sig_new.push(sig1[i] - sig2[i]);
    }

    sig_new
}

pub fn mux_signal(sig1: &[f64], sig2: &[f64]) -> Vec<f64> {
    let mut sig_new: Vec<f64> = Vec::new();
    let length = if sig1.len() < sig2.len() {
        sig1.len()
    } else {
        sig2.len()
    };

    for i in 0..length {
        sig_new.push(sig1[i] * sig2[i]);
    }

    sig_new
}

pub fn scaling(sig1: &[f64], amplitude: f64) -> Vec<f64> {
    let mut sig_new: Vec<f64> = Vec::new();

    for element in sig1.iter() {
        sig_new.push(element * amplitude);
    }

    sig_new
}

pub fn move_average(sig1: &[f64], kernel_length: i32) -> Vec<f64> {
    let mut sig_new: Vec<f64> = Vec::new();

    if sig1.len() < kernel_length as usize {
        return sig_new;
    }

    for i in 0..=sig1.len() - kernel_length as usize {
        let mean = sig1[i..i + kernel_length as usize].iter().sum::<f64>() / kernel_length as f64;
        sig_new.push(mean);
    }

    sig_new
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_signals_basic() {
        let sig1 = vec![1.0, 2.0, 3.0];
        let sig2 = vec![4.0, 5.0, 6.0];
        let result = add_signal(&sig1, &sig2);
        assert_eq!(result, vec![5.0, 7.0, 9.0]);
    }

    #[test]
    fn test_add_signals_different_lengths() {
        let sig1 = vec![1.0, 2.0];
        let sig2 = vec![3.0, 4.0, 5.0];
        let result = add_signal(&sig1, &sig2);
        // результат должен иметь длину меньшего вектора
        assert_eq!(result, vec![4.0, 6.0]);
    }

    #[test]
    fn test_add_signals_negative_values() {
        let sig1 = vec![-1.0, -2.0, -3.0];
        let sig2 = vec![1.0, 2.0, 3.0];
        let result = add_signal(&sig1, &sig2);
        assert_eq!(result, vec![0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_sub_signals_basic() {
        let sig1 = vec![5.0, 7.0, 9.0];
        let sig2 = vec![1.0, 2.0, 3.0];
        let result = sub_signal(&sig1, &sig2);
        assert_eq!(result, vec![4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_sub_signals_different_lengths() {
        let sig1 = vec![1.0, 2.0, 3.0];
        let sig2 = vec![1.0];
        let result = sub_signal(&sig1, &sig2);
        assert_eq!(result, vec![0.0]);
    }

    #[test]
    fn test_sub_signals_negative_values() {
        let sig1 = vec![-1.0, -2.0, -3.0];
        let sig2 = vec![1.0, 1.0, 1.0];
        let result = sub_signal(&sig1, &sig2);
        assert_eq!(result, vec![-2.0, -3.0, -4.0]);
    }

    #[test]
    fn test_mux_signal_same_length() {
        let sig1 = vec![1.0, 2.0, 3.0];
        let sig2 = vec![4.0, 5.0, 6.0];
        let expected = vec![4.0, 10.0, 18.0];
        assert_eq!(mux_signal(&sig1, &sig2), expected);
    }

    #[test]
    fn test_mux_signal_first_shorter() {
        let sig1 = vec![2.0, 3.0];
        let sig2 = vec![1.0, 2.0, 3.0, 4.0];
        let expected = vec![2.0, 6.0];
        assert_eq!(mux_signal(&sig1, &sig2), expected);
    }

    #[test]
    fn test_mux_signal_second_shorter() {
        let sig1 = vec![1.0, 2.0, 3.0, 4.0];
        let sig2 = vec![0.5, 2.0];
        let expected = vec![0.5, 4.0];
        assert_eq!(mux_signal(&sig1, &sig2), expected);
    }

    #[test]
    fn test_mux_signal_one_empty() {
        let sig1 = vec![];
        let sig2 = vec![1.0, 2.0, 3.0];
        let expected: Vec<f64> = vec![];
        assert_eq!(mux_signal(&sig1, &sig2), expected);
    }

    #[test]
    fn test_mux_signal_both_empty() {
        let sig1: Vec<f64> = vec![];
        let sig2: Vec<f64> = vec![];
        let expected: Vec<f64> = vec![];
        assert_eq!(mux_signal(&sig1, &sig2), expected);
    }

    #[test]
    fn test_mux_signal_with_zeros() {
        let sig1 = vec![0.0, 2.0, -3.0];
        let sig2 = vec![5.0, 0.0, 4.0];
        let expected = vec![0.0, 0.0, -12.0];
        assert_eq!(mux_signal(&sig1, &sig2), expected);
    }

    #[test]
    fn test_mux_signal_with_negatives() {
        let sig1 = vec![-1.0, -2.0];
        let sig2 = vec![3.0, -4.0];
        let expected = vec![-3.0, 8.0];
        assert_eq!(mux_signal(&sig1, &sig2), expected);
    }

    #[test]
    fn test_scaling_normal_values() {
        let sig = vec![1.0, 2.0, 3.0];
        let amplitude = 2.5;
        let expected = vec![2.5, 5.0, 7.5];
        assert_eq!(scaling(&sig, amplitude), expected);
    }

    #[test]
    fn test_scaling_with_zero_amplitude() {
        let sig = vec![1.0, -2.0, 3.5];
        let amplitude = 0.0;
        let expected = vec![0.0, 0.0, 0.0];
        assert_eq!(scaling(&sig, amplitude), expected);
    }

    #[test]
    fn test_scaling_with_negative_amplitude() {
        let sig = vec![1.0, -2.0, 3.0];
        let amplitude = -1.0;
        let expected = vec![-1.0, 2.0, -3.0];
        assert_eq!(scaling(&sig, amplitude), expected);
    }

    #[test]
    fn test_scaling_empty_signal() {
        let sig: Vec<f64> = vec![];
        let amplitude = 5.0;
        let expected: Vec<f64> = vec![];
        assert_eq!(scaling(&sig, amplitude), expected);
    }

    #[test]
    fn test_scaling_with_fractional_amplitude() {
        let sig = vec![4.0, 8.0, 12.0];
        let amplitude = 0.5;
        let expected = vec![2.0, 4.0, 6.0];
        assert_eq!(scaling(&sig, amplitude), expected);
    }

    #[test]
    fn test_scaling_with_nan() {
        let sig = vec![1.0, f64::NAN, 3.0];
        let amplitude = 2.0;
        let result = scaling(&sig, amplitude);
        assert!(result[0].is_finite());
        assert!(result[1].is_nan());
        assert!(result[2].is_finite());
    }

    #[test]
    fn test_scaling_with_infinity() {
        let sig = vec![f64::INFINITY, -2.0];
        let amplitude = 3.0;
        let result = scaling(&sig, amplitude);
        assert!(result[0].is_infinite());
        assert_eq!(result[1], -6.0);
    }

    #[test]
    fn test_move_average_basic() {
        let sig = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let kernel = 3;
        let result = move_average(&sig, kernel);
        let expected = vec![2.0, 3.0, 4.0]; // (1+2+3)/3, (2+3+4)/3, (3+4+5)/3
        assert_eq!(result, expected);
    }

    #[test]
    fn test_move_average_kernel_1() {
        let sig = vec![1.0, 2.0, 3.0];
        let kernel = 1;
        let result = move_average(&sig, kernel);
        // Должен вернуть все элементы (усреднение по одному элементу)
        assert_eq!(result, vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_move_average_kernel_equals_signal_length() {
        let sig = vec![1.0, 2.0, 3.0];
        let kernel = 3;
        let result = move_average(&sig, kernel);
        // Только одно окно: [0..3]
        assert_eq!(result, vec![2.0]);
    }

    #[test]
    fn test_move_average_kernel_larger_than_signal() {
        let sig = vec![1.0, 2.0];
        let kernel = 5;
        let result = move_average(&sig, kernel);
        // Нет ни одного полного окна → пустой результат
        assert_eq!(result, Vec::<f64>::new());
    }

    #[test]
    fn test_move_average_empty_signal() {
        let sig: Vec<f64> = vec![];
        let kernel = 3;
        let result = move_average(&sig, kernel);
        assert_eq!(result, Vec::<f64>::new());
    }

    #[test]
    fn test_move_average_with_negative_and_fractional() {
        let sig = vec![-1.0, 0.0, 1.0, 2.0];
        let kernel = 2;
        let result = move_average(&sig, kernel);
        let expected = vec![-0.5, 0.5, 1.5]; // (-1+0)/2, (0+1)/2, (1+2)/2
        assert_eq!(result, expected);
    }

    #[test]
    fn test_move_average_kernel_zero() {
        // kernel_length = 0 → деление на ноль!
        // В текущей реализации: kernel_length as usize = 0 → sig1.len() - 0 = len
        // Затем в цикле: sig1[i..i+0] — пустой срез → sum = 0.0, деление на 0.0 → NaN
        let sig = vec![1.0, 2.0, 3.0];
        let result = move_average(&sig, 0);
        // Поведение: цикл выполняется sig1.len() раз, каждый раз push(NaN)
        assert!(result.iter().all(|&x| x.is_nan()));
    }

    #[test]
    fn test_move_average_kernel_negative() {
        // kernel_length < 0 → as usize даёт большое положительное число
        // sig1.len() - (большое число) → переполнение (panic в debug, wrap в release)
        // В debug-режиме это panic! "attempt to subtract with overflow"
        // Поэтому такой вызов — ошибка программиста.
        // Тест можно пропустить или проверить панику:
        let sig = vec![1.0, 2.0, 3.0];
        let _ = std::panic::catch_unwind(|| {
            move_average(&sig, -1);
        });
        // В production лучше валидировать входные данные
    }
}
