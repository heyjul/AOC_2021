use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("./src/day7.input")?;
    let mut positions: Vec<i32> = BufReader::new(file)
        .lines()
        .next()
        .unwrap()?
        .split(',')
        .map(|position| position.parse::<i32>().unwrap())
        .collect();
    
    positions.sort_unstable();
    let median;
    let i = positions.len() / 2;
    if positions.len() % 2 == 0 {
        median = (positions.get(i).unwrap() + positions.get(i - 1).unwrap()) / 2;
    } else {
        median = *positions.get(i).unwrap();
    }

    println!("They must spend {} fuel", positions.iter().fold(0, |mut acc, pos| {
        acc += if pos - median < 0 { median - pos } else { pos - median };
        acc
    }));

    Ok(())
}
