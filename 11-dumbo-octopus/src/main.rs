use colored::*;
use std::fs::File;
use std::io::{self, BufRead};

const IS_TEST: bool = false;

fn main() {
    let file_name = if IS_TEST { "input.test.txt" } else { "input.txt" };
    // open target file
    let file = File::open(&file_name)
        .expect("unable to open file");

    // uses a reader buffer
    let mut rows: Vec<Vec<(u8, bool)>> = io::BufReader::new(file)
        .lines()
        .map(|s| {
            s.unwrap()
             .chars()
             .map(|n| (n.to_digit(10).unwrap() as u8, false))
             .collect()
        })
        .collect();

    let mut step = 0;
    loop {
        step += 1;
        println!();
        println!("Step {}", step);
        increment(&mut rows);
        flash_mob(&mut rows);

        let step_score = get_number_of_flashes(&mut rows);
        
        reset_flash_status(&mut rows);
        print_octo_rows(&rows);

        if step_score == rows.len() * rows[0].len() {
            break;
        }
    }

    println!("Total flash happens at step {}", step);
}

fn increment(rows: &mut Vec<Vec<(u8, bool)>>) {
    for i in 0..rows.len() {
        for j in 0..rows[i].len() {
            rows[i][j].0 += 1
        }
    }
}

fn flash_mob(rows: &mut Vec<Vec<(u8, bool)>>) {
    for i in 0..rows.len() {
        for j in 0..rows[i].len() {
            if rows[i][j].0 >= 10 && !rows[i][j].1 {
                flash(i, j, rows);
            }
        }
    }
}

fn flash(i: usize, j: usize, rows: &mut Vec<Vec<(u8, bool)>>) {
    rows[i][j].0 += 1;

    if rows[i][j].0 < 10 || rows[i][j].1 {
        return
    }

    rows[i][j].1 = true;

    // NW
    if i > 0 && j > 0 {
        flash(i - 1, j - 1, rows);
    }
    // N
    if i > 0 {
        flash(i - 1, j, rows);
    }
    // NE
    if i > 0 && j < rows[i].len() - 1 {
        flash(i - 1, j + 1, rows);
    }
    // W
    if j > 0 {
        flash(i, j - 1, rows);
    }
    // E
    if j < rows[i].len() - 1 {
        flash(i, j + 1, rows);
    }
    // SW
    if i < rows.len() - 1 && j > 0 {
        flash(i + 1, j - 1, rows);
    }
    // S
    if i < rows.len() - 1 {
        flash(i + 1, j, rows);
    }
    // SE
    if i < rows.len() - 1 && j < rows[i].len() - 1 {
        flash(i + 1, j + 1, rows);
    }
}

fn get_number_of_flashes(rows: &Vec<Vec<(u8, bool)>>) -> usize {
    let mut flash_count = 0;

    for row in rows {
        for octo in row {
            if octo.1 {
                flash_count += 1;
            }
        }
    }

    flash_count
}

fn reset_flash_status(rows: &mut Vec<Vec<(u8, bool)>>) {
    for i in 0..rows.len() {
        for j in 0..rows[i].len() {
            if rows[i][j].1 {
                rows[i][j].0 = 0;
                rows[i][j].1 = false;
            }
        }
    }
}

fn print_octo_rows(rows: &Vec<Vec<(u8, bool)>>) {
    for row in rows {
        for octo in row {
            if octo.0 == 0 {
                print!("{}", octo.0.to_string().blue());
            } else {
                print!("{}", octo.0);
            }
        }
        println!();
    }
}
