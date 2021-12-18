use std::{fs::File, io::{self, BufRead}};

fn main() -> std::io::Result<()> {
    let file = File::open("./src/day3.input")?;
    let binaries: Vec<Vec<char>> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    let digit_count = binaries.get(0).unwrap().len();
    
    let mut binaries_copy = binaries.clone();
    for i in 0..digit_count {
        let binaries_count = binaries_copy.len();
        let mut count = 0;
        for binary in binaries_copy.iter() {
            if binary[i] == '1' {
                count += 1;
            }
        }

        let value = if binaries_count - count > count { '0' } else { '1' };

        binaries_copy = binaries_copy
            .into_iter()
            .filter(|x| x[i] == value)
            .collect();

        if binaries_copy.len() == 1 {
            break;
        }
    }

    let oxygen_generator_rating: String = (&binaries_copy[0]).iter().collect();
    let oxygen_generator_rating = u32::from_str_radix(&oxygen_generator_rating, 2).unwrap();

    binaries_copy = binaries.clone();

    for i in 0..digit_count {
        let binaries_count = binaries_copy.len();
        let mut count = 0;
        for binary in binaries_copy.iter() {
            if binary[i] == '1' {
                count += 1;
            }
        }

        let value = if binaries_count - count > count { '1' } else { '0' };

        binaries_copy = binaries_copy
            .into_iter()
            .filter(|x| x[i] == value)
            .collect();

        if binaries_copy.len() == 1 {
            break;
        }
    }

    let co2_scrubber_ratting: String = (&binaries_copy[0]).iter().collect();
    let co2_scrubber_ratting = u32::from_str_radix(&co2_scrubber_ratting, 2).unwrap();

    println!("The life support rating of the submarine is {}", oxygen_generator_rating * co2_scrubber_ratting);

    Ok(())
}