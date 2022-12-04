use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("file1.txt")?;
    let reader = BufReader::new(file);
    let mut score = 0;
    let mut priorities: HashMap<char, i32> = HashMap::new();
    
    build_priority_hashmap(&mut priorities);

    for line in reader.lines() {
        let line_string = line.unwrap();
        let iter = line_string.split_at(line_string.len()/2);

        let rucksack1 = iter.0;
        let rucksack2 = iter.1;

        for letter in rucksack1.chars() {
            if rucksack2.contains(letter) {
                score += priorities.get(&letter).unwrap();
                break;
            }
        }
    }
    println!("{}", score);

    Ok(())
}

fn build_priority_hashmap(priorities: &mut HashMap<char, i32>) {
    let mut lower_value: i32 = 1;
    let mut higher_value: i32 = 27;
    for m in 'a' as u32..='z' as u32 {
        priorities.insert(char::from_u32(m).unwrap(), lower_value);
        lower_value+=1;
    }
    for m in 'A' as u32..='Z' as u32 {
        priorities.insert(char::from_u32(m).unwrap(), higher_value);
        higher_value+=1;
    }
}