use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let file_name = "./input.txt";
    let mut horizontal = 0;
    let mut depth = 0;

    // open target file
    let file = File::open(&file_name)
        .expect("unable to open file");

    // uses a reader buffer
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    loop {
        match reader.read_line(&mut line) {
            Ok(bytes_read) => {
                // EOF: save last file address to restart from this address for next run
                if bytes_read == 0 {
                    break;
                }

                let dir = line.chars().nth(0).unwrap();
                let mag_char = line.chars().nth(line.len() - 2).unwrap();
                let mag = mag_char.to_digit(10)
                    .expect(format!("unknown digit {}", mag_char).as_str());
                    
                match dir {
                    'f' => horizontal += mag,
                    'd' => depth += mag,
                    'u' => depth -= mag,
                    _ => ()
                }

                // do not accumulate data
                line.clear();
            }
            Err(err) => {
                println!("{}", err)
            }
        };
    }

    println!("depth {} x horizontal {} = {}", depth, horizontal, depth * horizontal);
}