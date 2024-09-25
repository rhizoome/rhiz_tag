use std::str;

use chrono::NaiveDateTime;
use rhiz_tag::{to_datetag_array, TagBuf};
use std::io::{self, BufRead};

fn main() {
    let mut buf = TagBuf::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(datetime_str) => {
                let datetime_str = datetime_str.trim();
                match NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S") {
                    Ok(datetime) => {
                        to_datetag_array(&mut buf, datetime).unwrap();
                        println!("{}", str::from_utf8(buf.as_ref()).unwrap());
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
