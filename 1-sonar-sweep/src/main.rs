use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let file_name = "./input.txt";
    let mut last_depth = 0;
    let mut total_increases = 0;

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

                let depth: i32 = line
                    .trim()
                    .parse()
                    .expect(
                        format!("failed to read \"{}\" as a depth", line).as_str()
                    );
                
                if depth > last_depth {
                    total_increases += 1;
                }
                last_depth = depth;

                // do not accumulate data
                line.clear();
            }
            Err(err) => {
                println!("{}", err)
            }
        };
    }

    println!("the depth increased {} times", total_increases);
}