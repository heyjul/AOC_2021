use std::{fs::File, io::{self, BufRead}};

fn main() -> std::io::Result<()> {
    let file = File::open("./src/day1.input")?;
    let depths: Vec<u32> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mut increases = 0;
    let mut prev = &depths[..3];
    for i in 1..=(depths.len() - 3) {
        let curr = &depths[i..i+3];
        if prev.iter().sum::<u32>() < curr.iter().sum::<u32>() {
            increases += 1;
        }
        prev = curr;
    }

    println!("The number of times a depth measurement increases is {}", increases);
    
    Ok(())
}
