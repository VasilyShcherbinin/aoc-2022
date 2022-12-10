use std::{io, path::Path, fs};

fn main() -> io::Result<()> {

    let path = Path::new("file1.txt");
    let contents = fs::read_to_string(path).expect("Failed to read file");
    let char_vec: Vec<char> = contents.chars().collect();
    let mut answer = 0;
    let window_size = 4;

    let iter = char_vec.windows(window_size);
    let iter_slice_vec = iter.collect::<Vec<_>>();
    for (index,vec_slice) in iter_slice_vec.iter().enumerate() {
        let mut result_vec = vec_slice.to_vec();
        result_vec.sort();
        result_vec.dedup();
        if result_vec.len() == vec_slice.len() {
            println!("Found unique result slice {:?}, index {}", vec_slice, index);
            answer = index+window_size;
            break;
        }
    }

    println!("{}",answer);

    Ok(())
}

