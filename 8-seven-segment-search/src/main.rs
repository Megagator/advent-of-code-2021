use std::collections::HashMap;
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

    let unique_map = unique_map_to_num();

    let mut value_of_outputs: u32 = 0;
    for note in lines {
        let segments: Vec<&str> = {
            note.split(' ')
                .map(|s| s.trim() )
                .collect()
        };

        let size = segments.len();
        let mut char_count: HashMap<char, usize> = HashMap::new();
        for i in 0..(size - 5) {
            for c in segments[i].chars() {
                let count = char_count.get(&c).or(Some(&0));
                char_count.insert(c, count.unwrap() + 1);
            }
        }
        // print!("{:?}", char_count);

        let mut result_num_string = String::new();
        for i in (size - 4)..size {
            let mut unique_sum = 0;
            for c in segments[i].chars() {
                unique_sum += char_count.get(&c).unwrap();
            }

            result_num_string.push(*unique_map.get(&unique_sum).unwrap());
        }

        value_of_outputs += result_num_string.parse::<u32>().unwrap();
    }

    println!("value of output is {}", value_of_outputs);
}

fn reference_number_of_uses(chars: &str) -> usize {
    let mut sum = 0;
    for c in chars.chars() {
        sum += {
            match c {
                'a' | 'c' => 8,
                'b' => 6,
                'd' | 'g' => 7,
                'e' => 4,
                'f' => 9,
                _ => 0
            }
        }
    }

    sum
}


fn unique_map_to_num() -> HashMap<usize, char> {
    let mut hm: HashMap<usize, char> = HashMap::new();
    // use the Fig. 1 diagram for as a reference assignment
    hm.insert(reference_number_of_uses("abcefg"), '0');
    hm.insert(reference_number_of_uses("cf"), '1');
    hm.insert(reference_number_of_uses("acdeg"), '2');
    hm.insert(reference_number_of_uses("acdfg"), '3');
    hm.insert(reference_number_of_uses("bcdf"), '4');
    hm.insert(reference_number_of_uses("abdfg"), '5');
    hm.insert(reference_number_of_uses("abdefg"), '6');
    hm.insert(reference_number_of_uses("acf"), '7');
    hm.insert(reference_number_of_uses("abcdefg"), '8');
    hm.insert(reference_number_of_uses("abcdfg"), '9');

    hm
}


// Fig. 1
// 0:      1:      2:      3:      4:
//  aaaa    ....    aaaa    aaaa    ....
// b    c  .    c  .    c  .    c  b    c
// b    c  .    c  .    c  .    c  b    c
//  ....    ....    dddd    dddd    dddd
// e    f  .    f  e    .  .    f  .    f
// e    f  .    f  e    .  .    f  .    f
//  gggg    ....    gggg    gggg    ....

// 5:      6:      7:      8:      9:
//  aaaa    aaaa    aaaa    aaaa    aaaa
// b    .  b    .  .    c  b    c  b    c
// b    .  b    .  .    c  b    c  b    c
//  dddd    dddd    ....    dddd    dddd
// .    f  e    f  .    f  e    f  .    f
// .    f  e    f  .    f  e    f  .    f
//  gggg    gggg    ....    gggg    gggg
