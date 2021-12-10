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










// Start by taking each segment of a 7-segment display, and assign it a score based on the number of digits in which that segment is used. For instance, segment "a" is worth 8, because it's used in 8 digits (0, 2, 3, 5, 6, 7, 8, 9), segment "b" is 6, since it's used in 6 digits (0, 4, 5, 6, 8, 9), etc
// Now, take each digit, and add up the scores of all the segments used to create that digit. For example, a "1" uses segment "c", worth 8 points, and segment "f", worth 9 points, for a total of 17. If you do this for every digit, you'll find they yield 10 unique numbers. Armed with these sums, decoding the output is now fairly straightforward:
// Count the number of times each character occurs in the first part of the row, before the "|". Since all 10 digits are present here exactly once, this is equivalent to the first step described above. This is that character's score.
// For each digit in the output, add up the scores for each character contained in that digit.
// Look up the sum in a table of the sums you calculated earlier to find out which digit yields that sum.
// That's it. You've decoded the digit.
// I was pretty stoked to figure this out, mostly because the other ways I could think of seemed like the kind of fussy logic that I have to get wrong 4 or 5 times before I get it right.