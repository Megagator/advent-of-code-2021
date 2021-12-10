use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use colored::*;

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

    let mut low_point_map: HashMap<(usize, usize), HashMap<(usize, usize), bool>> = HashMap::new();
    let mut low_point_list = Vec::new();
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
                low_point_map.insert((i,j), HashMap::new());
                low_point_list.push((i,j));
            }
        }
    }

    for (low_point, basin_points) in low_point_map.iter_mut() {
        add_to_basin(*low_point, &rows, basin_points);
        // println!("{:?}, {:?}", low_point, basin_points);
    }

    for i in 0..rows.len() {
        for j in 0..rows[i].len() {
            let mut found_point = false;
            for (_, basin_points) in &low_point_map {
                if basin_points.get(&(i,j)) != None {
                    found_point = true;
                    print!("{}", rows[i][j].to_string().red());
                    break;
                }
            }
            
            if !found_point { 
                print!("{}", rows[i][j])
            }
        }
        println!();
    }
    
    let mut basin_sizes = Vec::new();
    for (_, basin_points) in &low_point_map {
        basin_sizes.push(basin_points.len());
    }

    basin_sizes.sort();
    let b1 = basin_sizes[basin_sizes.len() - 1];
    let b2 = basin_sizes[basin_sizes.len() - 2];
    let b3 = basin_sizes[basin_sizes.len() - 3];
    println!("Top 3 basin sizes {}, {}, {} multiplied are {}", b1, b2, b3, b1 * b2 * b3);
}

fn add_to_basin(point: (usize, usize), grid: &Vec<Vec<u32>>, basin_point_map: &mut HashMap<(usize, usize), bool>) {
    if grid[point.0][point.1] == 9 {
        return
    }

    basin_point_map.insert(point, true);

    // check north
    if point.0 > 0 && basin_point_map.get(&(point.0 - 1, point.1)) == None {
        add_to_basin((point.0 - 1, point.1), grid, basin_point_map);
    }
    
    // check south
    if point.0 < grid.len() - 1 && basin_point_map.get(&(point.0 + 1, point.1)) == None {
        add_to_basin((point.0 + 1, point.1), grid, basin_point_map);
    }
    
    // check west
    if point.1 > 0 && basin_point_map.get(&(point.0, point.1 - 1)) == None {
        add_to_basin((point.0, point.1 - 1), grid, basin_point_map);
    }
    
    // check east
    if point.1 < grid[point.0].len() - 1 && basin_point_map.get(&(point.0, point.1 + 1)) == None {
        add_to_basin((point.0, point.1 + 1), grid, basin_point_map);
    }
}