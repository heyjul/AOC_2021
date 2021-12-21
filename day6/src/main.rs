use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("./src/day6.input")?;
    let buffer = BufReader::new(file);
    let mut lanternfishes = [0; 10];
    buffer
        .lines()
        .next()
        .unwrap()?
        .split(',')
        .map(|day| day.parse::<u32>().unwrap())
        .for_each(|day| lanternfishes[day as usize] += 1);

    for _ in 0..256 {
        for i in 0..10 {
            match i {
                0 => {
                    lanternfishes[9] = lanternfishes[0];
                    lanternfishes[7] += lanternfishes[0];
                    lanternfishes[i] = lanternfishes[i + 1];
                }
                9 => lanternfishes[9] = 0,
                _ => lanternfishes[i] = lanternfishes[i + 1],
            }
        }
    }

    println!(
        "There are {} lanternfishes after 256 days",
        lanternfishes.iter().sum::<u64>()
    );
    Ok(())
}
