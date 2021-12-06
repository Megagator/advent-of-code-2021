use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
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
        // if vent[0] != vent[2] && vent[1] != vent[3] {
        //     continue;
        // }
        // println!("{}, {}, {}, {}", vent[0], vent[1], vent[2], vent[3]);
        log_vent_line(&mut field, vent[0], vent[1], vent[2], vent[3]);
    }

    print_field(&field);
}

fn log_vent_line(field: &mut HashMap<(usize, usize), usize>, x1: usize, y1: usize, x2: usize, y2: usize) {
    let mut x = x1;
    let mut y = y1;

    loop {
        let key = (x,y);
        let mut value = 0;
        if field.contains_key(&key) {
            value = *field.get(&key).unwrap();
        }
        field.insert(key, value + 1);

        if x == x2 && y == y2 {
            break;
        }

        if x != x2 {
            x = if x > x2 { x - 1 } else { x + 1 };
        }

        if y != y2 {
            y = if y > y2 { y - 1 } else { y + 1 };
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
                        print!("{}", v.to_string().green())
                    }
                    else { print!("{}", v) }
                },
                None => print!(".")
            }
        }
        println!();
    }

    println!("number of dangerous vent points: {}", dangerous_points);
}