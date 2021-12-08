use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

const IS_TEST: bool = false;

fn main() {
    let file_name = if IS_TEST { "input.test.txt" } else { "input.txt" };
    // open target file
    let file = File::open(&file_name)
        .expect("unable to open file");

    // uses a reader buffer
    let mut reader = BufReader::new(file);

    let mut crab_positions: Vec<i32> = {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        line.trim_end()
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|n| n.parse().unwrap())
            .collect()
    };

    crab_positions.sort();

    let count = crab_positions.len();
    let median: i32;
    if count % 2 == 0 {
        median = (crab_positions[count / 2 - 1] + crab_positions[count / 2]) / 2;
    } else {
        median = crab_positions[(count + 1) / 2 - 1]
    }

    let mut fuel_cost = 0;
    for crab in crab_positions.iter() {
        fuel_cost += (crab - median).abs();
    }

    println!("{:?}", crab_positions);
    println!("Cheapest position is {} and costs {} fuel", median, fuel_cost);
}