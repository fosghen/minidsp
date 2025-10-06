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