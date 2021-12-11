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

    let mut corrupted_score = 0;
    let mut autocomplete_scores = vec![];
    for row in rows {
        let mut syntax = vec![];
        let mut is_corrupted = false;
        for c in row.chars() {
            match c {
                '(' | '[' | '{' | '<' => syntax.push(c),
                ')' | ']' | '}' | '>' => {
                    let last = syntax.pop().unwrap();
                    if last != get_bracket_pair(c) {
                        corrupted_score += get_corrupt_score(c);
                        is_corrupted = true;
                        break;
                    }
                },
                _ => ()
            }
        }
        
        // println!("{} {:?}", is_corrupted, syntax);
        if !is_corrupted {
            let mut autocomplete_score = 0;
            while !syntax.is_empty() {
                let next = syntax.pop().unwrap();
                autocomplete_score = get_new_autocomplete_score(
                    next, autocomplete_score
                );
            }
            autocomplete_scores.push(autocomplete_score);
        }
    }

    autocomplete_scores.sort();
    let middle = (autocomplete_scores.len() + 1) / 2 - 1;
    // println!("{:?} {}", autocomplete_scores, middle);

    println!("Corrupted score is {}",  corrupted_score);
    println!("Middle autocomplete score is {}",  autocomplete_scores[middle]);
}

fn get_bracket_pair(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
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

fn get_new_autocomplete_score(c: char, score: u64) -> u64 {
    let new_score = score * 5;
    new_score + match get_bracket_pair(c) {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0
    }
}