use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::cmp;
use std::collections::HashMap;
use colored::*;

const FIELD_MAX: usize = 999;

fn main() {
    let file_name = "./input.txt";
    // open target file
    let file = File::open(&file_name)
        .expect("unable to open file");

    // uses a reader buffer
    let mut reader = BufReader::new(file);

    let vent_lines = {
        // get all boards
        let mut vent_lines: Vec<Vec<usize>> = Vec::new();
        loop {
            let mut line = String::new();
            if reader.read_line(&mut line).unwrap() == 0 {
                break;
            }
            let points = line.trim_end()
                             .split(&[',',' ','-','>'][..])
                             .filter(|s| !s.is_empty())
                             .map(|n| n.parse().unwrap())
                             .collect();

            vent_lines.push(points)
        }

        vent_lines
    };

    let mut field: HashMap<(usize, usize), usize> = HashMap::new();

    for vent in vent_lines.iter() {
        // only consider horizontal or vertical vent lines
        if vent[0] != vent[2] && vent[1] != vent[3] {
            continue;
        }
        // println!("{}, {}, {}, {}", vent[0], vent[1], vent[2], vent[3]);
        log_vent_line(&mut field, vent[0], vent[1], vent[2], vent[3]);
    }

    print_field(&field);
}

fn log_vent_line(field: &mut HashMap<(usize, usize), usize>, x1: usize, y1: usize, x2: usize, y2: usize) {
    let lo_x = cmp::min(x1, x2);
    let hi_x = cmp::max(x1, x2);
    let lo_y = cmp::min(y1, y2);
    let hi_y = cmp::max(y1, y2);

    let mut x = lo_x;
    let mut y = lo_y;
    loop {
        let key = (x,y);
        let mut value = 0;
        if field.contains_key(&key) {
            value = *field.get(&key).unwrap();
        }
        field.insert(key, value + 1);

        if x == hi_x && y == hi_y {
            break;
        }

        if x < hi_x {
            x += 1;
        }
        if y < hi_y {
            y += 1;
        }
    }
}

fn print_field(field: &HashMap<(usize, usize), usize>) {
    let mut dangerous_points = 0;
    for x in 0..=FIELD_MAX {
        for y in 0..=FIELD_MAX {
            match field.get(&(x,y)) {
                Some(v) => {
                    if *v > 1 {
                        dangerous_points += 1;
                        print!("{} ", v.to_string().green())
                    }
                    else { print!("{} ", v) }
                },
                None => print!(". ")
            }
        }
        println!();
    }

    println!("number of dangerous vent points: {}", dangerous_points);
}