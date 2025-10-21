pub fn add_sgnals(sig1: &Vec<f64>, sig2: &Vec<f64>) -> Vec<f64>{
    let mut sig_new: Vec<f64> = Vec::new();
    let length = if sig1.len() < sig2.len() {sig1.len()} else {sig2.len()};
    
    for i in 0..length {
        sig_new.push(sig1[i] + sig2[i]);
    }

    sig_new
}

pub fn sub_sgnals(sig1: &Vec<f64>, sig2: &Vec<f64>) -> Vec<f64>{
    let mut sig_new: Vec<f64> = Vec::new();
    let length = if sig1.len() < sig2.len() {sig1.len()} else {sig2.len()};
    
    for i in 0..length {
        sig_new.push(sig1[i] - sig2[i]);
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
        let result = add_sgnals(&sig1, &sig2);
        assert_eq!(result, vec![5.0, 7.0, 9.0]);
    }

    #[test]
    fn test_add_signals_different_lengths() {
        let sig1 = vec![1.0, 2.0];
        let sig2 = vec![3.0, 4.0, 5.0];
        let result = add_sgnals(&sig1, &sig2);
        // результат должен иметь длину меньшего вектора
        assert_eq!(result, vec![4.0, 6.0]);
    }

    #[test]
    fn test_add_signals_negative_values() {
        let sig1 = vec![-1.0, -2.0, -3.0];
        let sig2 = vec![1.0, 2.0, 3.0];
        let result = add_sgnals(&sig1, &sig2);
        assert_eq!(result, vec![0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_sub_signals_basic() {
        let sig1 = vec![5.0, 7.0, 9.0];
        let sig2 = vec![1.0, 2.0, 3.0];
        let result = sub_sgnals(&sig1, &sig2);
        assert_eq!(result, vec![4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_sub_signals_different_lengths() {
        let sig1 = vec![1.0, 2.0, 3.0];
        let sig2 = vec![1.0];
        let result = sub_sgnals(&sig1, &sig2);
        assert_eq!(result, vec![0.0]);
    }

    #[test]
    fn test_sub_signals_negative_values() {
        let sig1 = vec![-1.0, -2.0, -3.0];
        let sig2 = vec![1.0, 1.0, 1.0];
        let result = sub_sgnals(&sig1, &sig2);
        assert_eq!(result, vec![-2.0, -3.0, -4.0]);
    }
}