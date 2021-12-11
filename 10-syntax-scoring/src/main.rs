// use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

const IS_TEST: bool = false;

fn main() {
    let file_name = if IS_TEST { "input.test.txt" } else { "input.txt" };
    // open target file
    let file = File::open(&file_name)
        .expect("unable to open file");

    // uses a reader buffer
    let rows: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|s| s.unwrap())
        .collect();

    let mut score = 0;
    for row in rows {
        let mut syntax = vec![];
        for c in row.chars() {
            match c {
                '(' | '[' | '{' | '<' => syntax.push(c),
                ')' | ']' | '}' | '>' => {
                    let last = syntax.pop().unwrap();
                    if last != get_bracket_pair(c) {
                        score += get_corrupt_score(c);
                        break;
                    }
                },
                _ => ()
            }
        }
    }

    println!("Corrupted score is {}", score);
}

fn get_bracket_pair(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => c
    }
}

fn get_corrupt_score(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0
    }
}