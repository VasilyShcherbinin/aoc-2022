use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("file1.txt")?;
    let reader = BufReader::new(file);
    let mut score = 0;
    let mut priorities: HashMap<char, i32> = HashMap::new();
    
    build_priority_hashmap(&mut priorities);

    let lines_vec: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Something went wrong"))
        .collect();

    let three_slice: Vec<Vec<String>> = lines_vec.chunks(3).map(|s| s.into()).collect();
    for vector in three_slice.iter() {
        for letter in vector.get(0).unwrap().chars() {
            if vector.get(1).unwrap().contains(letter) {
                if vector.get(2).unwrap().contains(letter){
                    score += priorities.get(&letter).unwrap();
                    break;
                }
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