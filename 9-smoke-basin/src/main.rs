use std::fs::File;
use std::io::{self, BufRead};

const IS_TEST: bool = false;

fn main() {
    let file_name = if IS_TEST { "input.test.txt" } else { "input.txt" };
    // open target file
    let file = File::open(&file_name)
        .expect("unable to open file");

    // uses a reader buffer
    let rows: Vec<Vec<u32>> = io::BufReader::new(file)
        .lines()
        .map(|s| {
            s.unwrap()
             .chars()
             .map(|c| c.to_digit(10).unwrap())
             .collect()
        })
        .collect();

    let mut low_points = Vec::new();
    for i in 0..rows.len() {
        for j in 0..rows[i].len() {
            let mut north_is_higher = false;
            if i <= 0 || rows[i][j] < rows[i-1][j] {
                north_is_higher = true;
            }
            
            let mut south_is_higher = false;
            if i == rows.len() - 1 || rows[i][j] < rows[i+1][j] {
                south_is_higher = true;
            }

            let mut west_is_higher = false;
            if j == 0 || rows[i][j] < rows[i][j-1] {
                west_is_higher = true;
            }
            
            let mut east_is_higher = false;
            if j == rows[i].len() - 1 || rows[i][j] < rows[i][j+1] {
                east_is_higher = true;
            }

            if north_is_higher && south_is_higher && west_is_higher && east_is_higher {
                low_points.push(rows[i][j]);
            }
        }
    }

    let risk = low_points.iter().sum::<u32>() + low_points.len() as u32;
    println!("Risk level is {}", risk);
}
