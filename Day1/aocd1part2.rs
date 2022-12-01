use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("file1.txt")?;
    let reader = BufReader::new(file);
    let mut counter: i32 = 0;
    let mut max_count: i32 = 0;
    let mut elf_list: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line_string = line.unwrap();
        if line_string.is_empty() {
            elf_list.push(counter);
            if max_count < counter {
                max_count = counter;
            }
            counter = 0;
        }
        else {
            counter += &line_string.parse().unwrap();
        }
    }

    elf_list.sort();
    elf_list.reverse();

    let mut sum = 0;
    for i in 0..3 {
        sum += elf_list.get(i).unwrap();
    }
    println!("Sum: {}", sum);
    println!("MaxCount: {}", max_count);

    Ok(())
}
