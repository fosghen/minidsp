pub fn add_sgnals(sig1: &Vec<f64>, sig2: &Vec<f64>) -> Vec<f64>{
    let mut sig_new: Vec<f64> = Vec::new();
    let length = if sig1.len() < sig2.len() {sig1.len()} else {sig2.len()};
    
    for i in 0..length {
        sig_new.push(sig1[i] + sig2[i]);
    }

    sig_new
}