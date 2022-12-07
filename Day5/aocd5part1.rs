use std::{io::{self, BufReader, BufRead, Lines, Error}, fs::File, path::Path};

use regex::Regex;

fn main() -> io::Result<()> {

    let file = File::open("file1.txt")?;
    let path = Path::new("file1.txt");
    let mut answer: String = String::new();

    let reader = BufReader::new(file);
    let mut reader_lines = reader.lines();
    let mut init_stacks = initialize_stacks(&mut reader_lines, path);

    // reader_lines is now a mutable iterator, so we can use it in the loop
    for lines in reader_lines {
        let move_amount: u32;
        let from_stack: usize;
        let to_stack:usize;
        let line_result = lines.as_ref();
        if line_result.unwrap().contains("move") {
            let re = Regex::new(r"\d+").unwrap();
            let numbers: Vec<u32> = re
                .find_iter(line_result.unwrap())
                .map(|m| m.as_str())
                .map(|s| s.parse().unwrap())
                .collect();
            move_amount = numbers[0];
            from_stack = numbers[1] as usize - 1;
            to_stack = numbers[2] as usize - 1;

            for _ in 0..move_amount {
                let popped_item = init_stacks[from_stack].pop().unwrap();
                init_stacks[to_stack].push(popped_item);
            }
        }
    }
    
    get_final_answer(init_stacks, &mut answer);

    Ok(())
}

fn get_final_answer(init_stacks: Vec<Vec<char>>, answer: &mut String) {
    for vectors in init_stacks {
        answer.push_str(&vectors.last().unwrap().to_string());
    }
    println!("{}", answer);
}

// Update the function to take a mutable reference to an iterator of lines instead of a BufReader
fn initialize_stacks(reader_lines: &mut Lines<BufReader<File>>, path: &Path) -> Vec<Vec<char>> {
    let mut init_stacks: Vec<Vec<char>> = Vec::new();
    let mut init_stacks_def_start_line = 0;
    let re = Regex::new(r"^\s[0-9\s]*(\d)\s$").unwrap();
    // Use the iterator of lines instead of calling lines() on the BufReader
    for (index, entries) in reader_lines.enumerate() {
        if re.is_match(entries.as_ref().unwrap()) {
            build_stack_vec(&re, entries, &mut init_stacks);
            init_stacks_def_start_line = index;
            break;
        }
    }
    for idx in (1..=init_stacks_def_start_line).rev() {
        let line = get_line_at(&path, idx-1).unwrap();
        for (index, characters) in line.chars().enumerate() {
            if characters.is_alphabetic() {
                init_stacks[(index-1)/4].push(characters);
            }
        }
    }
    init_stacks
}

fn build_stack_vec(re: &Regex, entries: Result<String, Error>, init_stacks: &mut Vec<Vec<char>>) {
    let cap = re.captures(entries.as_ref().unwrap()).unwrap();
    let stacks: i32 = cap[1].parse::<i32>().unwrap();
    for _ in 0..stacks {
        let v : Vec<char> = Vec::new();
        init_stacks.push(v);
    }
}

fn get_line_at(path: &Path, line_num: usize) -> Result<String, Error> {
    let file = File::open(path).expect("File not found or cannot be opened");
    let content = BufReader::new(&file);
    let mut lines = content.lines();
    lines.nth(line_num).expect("No line found at that position")
}