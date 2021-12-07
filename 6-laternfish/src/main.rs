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

    let lantern_fish: Vec<usize> = {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        line.trim_end()
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|n| n.parse().unwrap())
            .collect()
    };

    let mut lantern_fish_buckets: Vec<i64> = vec![0,0,0,0,0,0,0,0,0];
    for fish in lantern_fish {
        lantern_fish_buckets[fish] += 1;
    }

    println!("{:?}", lantern_fish_buckets);

    let upper_bound = if IS_TEST { 18 } else { 256 };
    print!("Initial state: ");
    println!("{:?}", lantern_fish_buckets);
    for day in 1..=upper_bound {
        /*
            0 -> 6, dupe into 8
            1 -> 0
            2 -> 1
            3 -> 2
            4 -> 3
            5 -> 4
            6 -> 5
            7 -> 6
            8 -> 7
        */
        let spawned_fish = lantern_fish_buckets[0];
        for i in 1..lantern_fish_buckets.len() {
            lantern_fish_buckets[i - 1] = lantern_fish_buckets[i];
        }
        lantern_fish_buckets[8] = spawned_fish;
        lantern_fish_buckets[6] += spawned_fish;
        
        let day_string = format!("{: >2}", day);
        print!("After {} {}", day_string, if day == 1 { "day:  " } else { "days: " });
        println!("{:?}", lantern_fish_buckets);
    }

    let sum:i64 = lantern_fish_buckets.iter().sum();
    println!("After {} days there will be {} fish", upper_bound, sum)
}