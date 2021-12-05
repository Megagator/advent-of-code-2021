use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use colored::*;

fn main() {
    let file_name = "./input.txt";
    // open target file
    let file = File::open(&file_name)
        .expect("unable to open file");

    // uses a reader buffer
    let mut reader = BufReader::new(file);
    
    // get the random draw
    let random_draw: Vec<usize> = {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        line.trim_end()
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|n| n.parse().unwrap())
            .collect()
    };

    let mut boards = {
        // get all boards
        let mut boards = Vec::new();
        loop {
            let mut line = String::new();
            if reader.read_line(&mut line).unwrap() == 0 {
                break;
            }
            boards.push(make_board(&mut reader));
        }

        boards
    };

    for n in random_draw {
        for board in boards.iter_mut() {
            if apply_new_draw(n, board) {
                // print_board(board);
                // println!("-----");
                if winning_board(board) {
                    print_board(board);
                    print_board_score(n, board);
                    return
                }
            }
        }
    }

}

fn make_board(mut reader: impl BufRead) -> Vec<Vec<(usize, bool)>> {
    let mut board = Vec::new();
    for _ in 0..5 {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        let row = line.trim_end()
                      .split(' ')
                      .filter(|s| !s.is_empty())
                      .map(|n| {
                        (
                            n.parse().unwrap(),
                            false
                        )
                      })
                      .collect();
        board.push(row)
    }

    board
}

fn apply_new_draw(num: usize, board: &mut Vec<Vec<(usize, bool)>>) -> bool {
    for row in 0..board.len() {
        for col in 0..board[row].len() {
            if board[row][col].0 == num {
                board[row][col].1 = true;
                return true;
            }
        }
    }

    false
}

fn winning_board(board: & Vec<Vec<(usize, bool)>>) -> bool {
    // by row
    for row in 0..board.len() {
        for col in 0..board[row].len() {
            if !board[row][col].1 {
                break;
            }
            if col == board.len() - 1 {
                return true
            }
        }
    }
    
    // by column
    for col in 0..board.len() {
        for row in 0..board[col].len() {
            if !board[col][row].1 {
                break;
            }
            if row == board[0].len() - 1 {
                return true
            }
        }
    }

    false
}

fn print_board(board: & Vec<Vec<(usize, bool)>>) {
    for row in 0..board.len() {
        for col in 0..board[row].len() {
            if board[row][col].1 {
                print!("{} ", board[row][col].0.to_string().cyan())
            } else {
                print!("{} ", board[row][col].0)
            }
        }
        println!();
    }
}

fn print_board_score(winning_num: usize, board: & Vec<Vec<(usize, bool)>>) {
    let mut score = 0;
    for row in 0..board.len() {
        for col in 0..board[row].len() {
            if !board[row][col].1 {
                score += board[row][col].0
            }
        }
    }

    println!("winning board score is {}", score * winning_num)
}