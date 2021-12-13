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
    let paths: Vec<(String, String)> = io::BufReader::new(file)
        .lines()
        .map(|s| s.unwrap())
        .map(|str| {
            let s: Vec<&str> = str.split("-").collect();
            (s[0].to_owned(), s[1].to_owned())
        })
        .collect();
    // silly, but gets us string references
    let paths: Vec<(&str, &str)> = paths
        .iter()
        .map(|tup| (tup.0.as_str(), tup.1.as_str()))
        .collect();

    let mut caves: HashMap<&str, Vec<&str>> = HashMap::new();
    // for relationship A-B
    for path in paths.iter() {
        // map that A has B
        if let Some(cave) = caves.get_mut(path.0) {
            cave.push(path.1);
        } else {
            caves.insert(path.0, vec!(path.1));
        }

        // map that B has A
        if let Some(cave) = caves.get_mut(path.1) {
            cave.push(path.0);
        } else {
            caves.insert(path.1, vec!(path.0));
        }
    }

    // for (cave_name, cave_list) in caves {
    //     println!("{} is connected to {:?}", cave_name, cave_list);
    // }

    let mut unique_path = vec!["start"];
    let mut possible_paths = 0;
    explore_path(&caves, &mut unique_path, &mut possible_paths);
    println!("The number of possible paths through caves is {}", possible_paths);
}


fn explore_path(
    caves: & HashMap<&str, Vec<&str>>,
    current_path: &mut Vec<&str>,
    possible_paths: &mut u32
) {
    let current_cave = current_path.last().unwrap();
    if let Some((cave_key, connected_caves)) = caves.get_key_value(current_cave) {
        if *cave_key == "end" {
            *possible_paths += 1;
            println!("{:?}", current_path);
        } else {
            for cave in connected_caves {
                let mut new_path = current_path.to_owned();
                new_path.push(*cave);
                if cave.to_lowercase() == *cave {
                    // it is lowercase
                    if !current_path.contains(cave) {
                        // has not yet been visited
                        explore_path(caves, &mut new_path, possible_paths)
                    }
                } else {
                    explore_path(caves, &mut new_path, possible_paths)
                }
            }
        }
    }
}