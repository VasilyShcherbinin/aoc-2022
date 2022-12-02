use std::fs::File;
use std::io::{self, prelude::*, BufReader};

//A-X->Rock->1
//B-Y->Paper->2
//C-Z->Scissors->3
//Loss -> 0
//Draw -> 3
//Win -> 6

//X->Lose
//Y->Draw
//Z->Win
fn main() -> io::Result<()> {
    let file = File::open("file1.txt")?;
    let reader = BufReader::new(file);
    let mut score = 0;

    const WIN_VALUE: i32 = 6;
    const DRAW_VALUE: i32 = 3;
    const LOSE_VALUE: i32 = 0;
    const ROCK_VALUE: i32 = 1;
    const PAPER_VALUE: i32 = 2;
    const SCISSORS_VALUE: i32 = 3;


    for line in reader.lines() {
        let line_string = line.unwrap();
        let mut iter = line_string.split_whitespace();
        let key = iter.next().unwrap();
        let value = iter.next().unwrap();
        match key {
            "A" => 
                match value {
                    "X" => score += SCISSORS_VALUE + LOSE_VALUE,
                    "Y" => score += ROCK_VALUE + DRAW_VALUE,
                    "Z" => score += PAPER_VALUE + WIN_VALUE,
                    _=> todo!()
                }
            "B" => 
                match value {
                    "X" => score += ROCK_VALUE + LOSE_VALUE,
                    "Y" => score += PAPER_VALUE + DRAW_VALUE,
                    "Z" => score += SCISSORS_VALUE + WIN_VALUE,
                    _=> todo!()
                }
            "C" => 
                match value {
                    "X" => score += PAPER_VALUE + LOSE_VALUE,
                    "Y" => score += SCISSORS_VALUE + DRAW_VALUE,
                    "Z" => score += ROCK_VALUE + WIN_VALUE,
                    _=> todo!()
                }
            _=> todo!()
        }
    }
    println!("{}", score);

    Ok(())
}
