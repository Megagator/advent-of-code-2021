use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;


const IS_TEST: bool = false;

fn main() {
    let file_name = if IS_TEST { "input.test.txt" } else { "input.txt" };
    // open target file
    let file = File::open(&file_name)
        .expect("unable to open file");

    // uses a reader buffer
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let poly_template = line.trim();

    let pairs = {
        let mut pairs = HashMap::new();
        loop {
            let mut line = String::new();
            if reader.read_line(&mut line).unwrap() == 0 {
                break;
            }
            if line.trim().is_empty() { continue; }

            let pair: Vec<&str> = line.trim()
                                    .split(" -> ")
                                    .collect();

            pairs.insert(pair[0].to_owned(), pair[1].to_owned());
        }
        pairs
    };

    // println!("1. {}", poly_template);
    // println!("{:?}", pairs);

    let mut input: Vec<char> = poly_template.chars().collect();
    for step in 1..=10 {
        let mut output = vec![];

        for i in 0..input.len() - 1 {
            let lookup_pair = format!("{}{}", input[i], input[i+1]);
            match pairs.get(&lookup_pair) {
                Some(c) => {
                    output.pop();
                    output.push(input[i]);
                    output.push(c.chars().last().unwrap());
                    output.push(input[i+1]);
                }
                None => ()
            }
        }

        // let result: String = output.iter().collect();
        // println!("{}. {}", step, result);

        input = output;
        println!("{}. size is {}", step, input.len());
    }


    // after 10 evolutions, count characters:
    let mut char_counter = HashMap::new();
    for ch in input {
        let counter = char_counter.entry(ch).or_insert(0);
        *counter += 1;
    }
    // println!("{:?}", char_counter);

    let mut lo = u32::MAX;
    let mut hi = 0;
    for (_,n) in char_counter {
        lo = std::cmp::min(lo, n);
        hi = std::cmp::max(hi, n);
    }

    println!("Most common minus the least common is {}", hi - lo);
}