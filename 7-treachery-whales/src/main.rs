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

    let count = crab_positions.len() as f32;
    let sum: i32 = crab_positions.iter().sum();
    let mean = (sum as f32 / count).floor() as i32;
    // println!("sum {} count {} mean {}", sum, count, mean);

    let mut fuel_cost = 0;
    for crab in crab_positions.iter() {
        let raw_cost = (crab - mean).abs();
        fuel_cost += triangular(raw_cost);
    }

    // println!("{:?}", crab_positions);
    println!("Cheapest position is {} and costs {} fuel", mean, fuel_cost);
}

fn triangular(num: i32) -> i32 {
    (num * num + num) / 2
}