use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let file_name = "./input.txt";
    let mut bits_sum: [i32; 12] = [0,0,0,0,0,0,0,0,0,0,0,0];
    let mut total_lines = 0;

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

                total_lines += 1;
                let mut line_chars = line.chars();
                for i in 0..12 {   
                    let bit = line_chars.next().unwrap();
                    let bit_digit = bit.to_digit(10)
                        .expect(format!("unknown digit {}", bit).as_str());

                    bits_sum[i] += bit_digit as i32;
                }

                // do not accumulate data
                line.clear();
            }
            Err(err) => {
                println!("{}", err)
            }
        };
    }

    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();
    for i in 0..12 {
        print!("{},", bits_sum[i]);
        if bits_sum[i] > total_lines / 2 {
            gamma_rate.push('1');
            epsilon_rate.push('0');
        } else {
            gamma_rate.push('0');
            epsilon_rate.push('1');
        }
    }
    println!("\ngamma rate = {}, epsilon rate {}", gamma_rate, epsilon_rate);

    let gamma_rate = binary_to_digit(gamma_rate);
    let epsilon_rate = binary_to_digit(epsilon_rate);

    println!("multiplied = {}", gamma_rate * epsilon_rate);
}

fn binary_to_digit(bin: String) -> i32 {
    return i32::from_str_radix(bin.as_str(), 2).unwrap();
}