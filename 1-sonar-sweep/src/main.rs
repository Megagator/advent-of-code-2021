use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let file_name = "./input.txt";
    let mut last_depths: [i32; 4] = [-1,-1,-1,-1];
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

                last_depths[0] = last_depths[1];
                last_depths[1] = last_depths[2];
                last_depths[2] = last_depths[3];
                last_depths[3] = depth;
                
                if last_depths[0] >= 0 && last_depths[0] < last_depths[3] {
                    total_increases += 1;
                }

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