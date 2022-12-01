use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("file1.txt")?;
    let reader = BufReader::new(file);
    let mut counter: i32 = 0;
    let mut max_count: i32 = 0;

    for line in reader.lines() {
        let line_string = line.unwrap();
        if line_string.is_empty() {
            if max_count < counter {
                max_count = counter;
            }
            counter = 0;
        }
        else {
            counter += &line_string.parse().unwrap();
        }

    }

    println!("{}", max_count);

    Ok(())
}
