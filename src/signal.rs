pub const SAMPLE_RATE: u32 = 10000;

use std::fs::OpenOptions;
use std::io::Write;


pub fn save_csv(signal: &Vec<f64>, filename: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)     
        .create(true)     
        .truncate(true)
        .open(filename)?;

    for value in signal {
        writeln!(file, "{};", value)?;
    }
    Ok(())
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_save_csv_creates_correct_file() {
        let signal = vec![1.0, 2.5, -3.14];
        let filename = "test_output.csv";

        save_csv(&signal, filename).expect("failed to save CSV");

        let content = fs::read_to_string(filename).expect("failed to read file");

        let expected = "1;\n2.5;\n-3.14;\n";
        assert_eq!(content, expected);

        fs::remove_file(filename).ok();
    }
}