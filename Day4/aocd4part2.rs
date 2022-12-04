use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use array_tool::vec::Intersect;

fn main() -> io::Result<()> {
    let file = File::open("file1.txt")?;
    let reader = BufReader::new(file);
    let mut score: i32 = 0;

    for line in reader.lines() {
        
        let mut elf1_sectors:Vec<u32> = Vec::new();
        let mut elf2_sectors:Vec<u32> = Vec::new();
        
        let line_string = line.unwrap();
        let mut iter = line_string.split(",");

        let elf1_task = iter.next().unwrap();
        let elf2_task = iter.next().unwrap();

        get_elf_sector(elf1_task, &mut elf1_sectors);
        get_elf_sector(elf2_task, &mut elf2_sectors);

        let same: bool = is_sector_overlap(elf2_sectors, elf1_sectors);
        if same {
            score += 1;
        }
    }
    
    println!("{}", score);

    Ok(())
}

fn is_sector_overlap(elf2_sectors: Vec<u32>, elf1_sectors: Vec<u32>) -> bool {
    let mut result = false;
    for m in elf1_sectors {
        if elf2_sectors.contains(&m) {
            result = true;
            break;
        }
        else {
            result = false;
        }
    }
    return result;

    // Or can do like this as well with a library
    //
    // if !elf2_sectors.intersect(elf1_sectors).is_empty() {
    //     return true;
    // } else {
    //     return false;
    // }
}

fn get_elf_sector(elf_task: &str, elf_sectors: &mut Vec<u32>) {
    let mut get_sector_range = elf_task.split("-");
    let sector_start = get_sector_range.next().unwrap();
    let sector_end = get_sector_range.next().unwrap();
    for i in sector_start.parse().unwrap()..=sector_end.parse().unwrap() {
         elf_sectors.push(i);
    }
}
