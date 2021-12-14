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

            pairs.insert(pair[0].to_owned(), pair[1].chars().next().unwrap());
        }
        pairs
    };

    // println!("1. {}", poly_template);
    // println!("{:?}", pairs);

    // create initial pairs and letter counts
    let mut output_pair_counts: HashMap<String, u64> = HashMap::new();
    let mut letter_counts: HashMap<char, u64> = HashMap::new();

    let input: Vec<char> = poly_template.chars().collect();
    for i in 0..input.len() {
        let letter_counter = letter_counts.entry(input[i]).or_insert(0);
        *letter_counter += 1;

        if i < input.len() - 1 {
            let lookup_pair = format!("{}{}", input[i], input[i+1]);
            let pair_counter = output_pair_counts.entry(lookup_pair).or_insert(0);
            *pair_counter += 1;
        }
    }

    println!("Step: 0");
    println!("pairs  : {:?}", output_pair_counts);
    println!("letters: {:?}", letter_counts);

    for step in 1..=40 {
        let mut new_pair_counts: HashMap<String, u64> = HashMap::new();

        for (pair, count) in output_pair_counts.iter_mut() {
            match pairs.get(pair) {
                Some(c) => {
                    // add two new pairs
                    let new_pair_a = format!("{}{}", pair.chars().next().unwrap(), c);
                    let new_pair_b = format!("{}{}", c, pair.chars().last().unwrap());
                    
                    let pair_counter = new_pair_counts.entry(new_pair_a).or_insert(0);
                    *pair_counter += *count;
                    let pair_counter = new_pair_counts.entry(new_pair_b).or_insert(0);
                    *pair_counter += *count;
                    
                    // update new character's count
                    let letter_counter = letter_counts.entry(*c).or_insert(0);
                    *letter_counter += *count;

                    // drop old pair
                    *count = 0;
                }
                None => ()
            }
        }

        // update pair counts
        for (pair, count) in new_pair_counts {
            let pair_counter = output_pair_counts.entry(pair).or_insert(0);
            *pair_counter += count;
        }

        println!();
        println!("Step: {}", step);
        println!("pairs  : {:?}", output_pair_counts);
        println!("letters: {:?}", letter_counts);
    }

    let mut lo = u64::MAX;
    let mut hi = 0;
    for (_,n) in letter_counts {
        lo = std::cmp::min(lo, n);
        hi = std::cmp::max(hi, n);
    }

    println!();
    println!("Most common minus the least common is {}", hi - lo);
}