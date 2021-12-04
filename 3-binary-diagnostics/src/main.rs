use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let file_name = "./input.txt";
    let mut diag_data = Vec::new();

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

                diag_data.push(line.clone());
                
                // do not accumulate data
                line.clear();
            }
            Err(err) => {
                println!("{}", err)
            }
        };
    }

    let mut oxy_rate_bit_criteria = String::new();
    let mut found_oxy = false;
    let mut co2_rate_bit_criteria = String::new();
    let mut found_co2 = false;
    
    for i in 0..diag_data[0].len() {
        let mut oxy_zeros_count = 0;
        let mut oxy_ones_count = 0;
        let mut co2_zeros_count = 0;
        let mut co2_ones_count = 0;
        let mut oxy_matches = 0;
        let mut last_oxy_match = 0;
        let mut co2_matches = 0;
        let mut last_co2_match = 0;

        // println!("key is {}", oxy_rate_bit_criteria);
        // println!("key is {}", co2_rate_bit_criteria);
        for j in 0..diag_data.len() {
            if !found_oxy && diag_data[j][..oxy_rate_bit_criteria.len()] == oxy_rate_bit_criteria {
                oxy_matches += 1;
                last_oxy_match = j;
                // println!("       {}", diag_data[j].trim_end());
                match diag_data[j].chars().nth(i).unwrap() {
                    '0' => oxy_zeros_count += 1,
                    '1' => oxy_ones_count += 1,
                    _ => ()
                }
            }
            
            if !found_co2 && diag_data[j][..co2_rate_bit_criteria.len()] == co2_rate_bit_criteria {
                co2_matches += 1;
                last_co2_match = j;
                // println!("       {}", diag_data[j].trim_end());
                match diag_data[j].chars().nth(i).unwrap() {
                    '0' => co2_zeros_count += 1,
                    '1' => co2_ones_count += 1,
                    _ => ()
                }
            }
        }

        // keep the most common value, 1 if tied
        if !found_oxy {
            if oxy_ones_count >= oxy_zeros_count {
                oxy_rate_bit_criteria.push('1');
            } else {
                oxy_rate_bit_criteria.push('0');
            }
        }

        // keep the least common value, 0 if tied
        if !found_co2 {
            if co2_ones_count < co2_zeros_count  {
                co2_rate_bit_criteria.push('1');
            } else {
                co2_rate_bit_criteria.push('0');
            }
        }

        if oxy_matches == 1 {
            found_oxy = true;
            oxy_rate_bit_criteria = diag_data[last_oxy_match].clone();
        }
        if co2_matches == 1 {
            found_co2 = true;
            co2_rate_bit_criteria = diag_data[last_co2_match].clone();
        }
    }

    println!("oxy {}, co2 {}", oxy_rate_bit_criteria.trim_end(), co2_rate_bit_criteria.trim_end());

    let oxy_rating = binary_to_digit(oxy_rate_bit_criteria);
    let co2_rating = binary_to_digit(co2_rate_bit_criteria);

    println!("life support rating: {}", oxy_rating * co2_rating);
}

fn binary_to_digit(bin: String) -> i32 {
    return i32::from_str_radix(bin.as_str().trim_end(), 2).unwrap();
}