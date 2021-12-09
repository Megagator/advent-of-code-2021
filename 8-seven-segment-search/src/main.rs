use std::fs::File;
use std::io::{self, BufRead};

const IS_TEST: bool = false;

fn main() {
    let file_name = if IS_TEST { "input.test.txt" } else { "input.txt" };
    // open target file
    let file = File::open(&file_name)
        .expect("unable to open file");

    // uses a reader buffer
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|s| s.unwrap())
        .collect();

    let mut num_unique_segments = 0;
    for note in lines {
        let segments: Vec<usize> = {
            note.split(' ')
                .map(|s| s.trim().len() )
                .collect()
        };

        let size = segments.len();
        for i in ((size - 4)..size).rev() {
            match segments[i] {
                2 | 3 | 4 | 7 => num_unique_segments += 1,
                _ => ()
            }
        }
    }

    println!("number of unique segments in output is {}", num_unique_segments);
}
