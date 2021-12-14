use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;


const IS_TEST: bool = false;

fn main() {
    let file_name = if IS_TEST { "input.test.txt" } else { "input.txt" };
    // open target file
    let file = File::open(&file_name)
        .expect("unable to open file");

    // uses a reader buffer
    let mut reader = BufReader::new(file);
    let mut max = (0,0);
    let mut points = {
        // get all boards
        let mut points = HashMap::new();
        loop {
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            if line == "\n" {
                break;
            }

            let point: Vec<u16> = line.trim()
                                     .split(',')
                                     .map(|s| s.parse::<u16>().unwrap())
                                     .collect();
            max.0 = std::cmp::max(point[1], max.0);
            max.1 = std::cmp::max(point[0], max.1);
            points.insert((point[1], point[0]), false);
        }

        points
    };

    let folds = {
        // get all boards
        let mut folds = Vec::new();

        loop {
            let mut line = String::new();
            if reader.read_line(&mut line).unwrap() == 0 {
                break;
            };

            // should really just use regex...
            let equation: Vec<&str> = line.trim()
                                     .split('=')
                                     .collect();
            let coordinate = equation[0].chars().last().unwrap();
            let magnitude = equation[1].parse::<u16>().unwrap();

            folds.push((
                // swap x and y because it's annoying
                if coordinate == 'x' { 'y' } else { 'x' },
                magnitude
            ));
        }

        folds
    };

    print_map(&points, max);

    for eq in folds {
        println!("{} = {}", eq.0, eq.1);
        fold_map(&mut points, eq);
        calculate_new_map_maxs(&mut max, eq);
        print_map(&points, max);
        println!("{} dots are visible", points.len());
    }
}

fn print_map(points: &HashMap<(u16,u16),bool>, max: (u16, u16)) {
    println!();
    for x in 0..=max.0 {
        for y in 0..=max.1 {
            match points.get(&(x,y)) {
                Some(_) => print!("#"),
                None => print!(".")
            }
        }
        println!();
    }
    println!();
}

fn fold_map(points: &mut HashMap<(u16,u16),bool>, equation: (char, u16)) {
    // copy the given point map so it's state is frozen until we're done folding
    for (point, _) in points.to_owned().iter() {
        if equation.0 == 'x' {
            if point.0 > equation.1 {
                let difference = point.0 - equation.1;
                let new_point = (point.0 - (difference * 2), point.1);
                points.remove(point);
                points.insert(new_point, true);
            }
        } else {
            if point.1 > equation.1 {
                let difference = point.1 - equation.1;
                let new_point = (point.0, point.1 - (difference * 2));
                points.remove(point);
                points.insert(new_point, true);
            }
        }
    }
}

fn calculate_new_map_maxs(max: &mut(u16, u16), equation: (char, u16)) {
    let new_bound = equation.1 - 1;
    match equation.0 {
        'x' => max.0 = new_bound,
        'y' => max.1 = new_bound,
        _ => ()
    }
}