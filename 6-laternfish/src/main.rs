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

    let mut lantern_fish: Vec<i32> = {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        line.trim_end()
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|n| n.parse().unwrap())
            .collect()
    };

    let upper_bound = if IS_TEST { 18 } else { 80 };
    println!("Initial state: ");
    print_fish(&lantern_fish);
    for day in 1..=upper_bound {
        let mut new_fish: Vec<i32> = vec![];
        lantern_fish = lantern_fish.iter().map(|fish| {
            match fish {
                0 => {
                    new_fish.push(8);
                    6
                },
                _ => fish - 1
            }
        }).collect();
        lantern_fish.append(&mut new_fish);
        
        let day_string = format!("{: >2}", day);
        print!("After {} {}", day_string, if day == 1 { "day:  " } else { "days: " });
        print_fish(&lantern_fish);
    }

    println!("After {} days there will be {} fish", upper_bound, lantern_fish.len())
}

fn print_fish(fish: &Vec<i32>) {
    for fish in fish {
        print!("{},", fish)
    }
    println!();
}