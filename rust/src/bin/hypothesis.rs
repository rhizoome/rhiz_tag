use chrono::NaiveDateTime;
use rhiz_tag::to_datetag;
use std::io::{self, BufRead}; // Replace `your_crate_name` with the name of your crate

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(datetime_str) => {
                let datetime_str = datetime_str.trim();
                match NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S") {
                    Ok(datetime) => {
                        let tag = to_datetag(datetime);
                        println!("{}", tag);
                    }
                    Err(e) => {
                        eprintln!("Error parsing datetime '{}': {}", datetime_str, e);
                        break;
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                break;
            }
        }
    }
}
