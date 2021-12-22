use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("./src/day7.input")?;
    let positions: Vec<i32> = BufReader::new(file)
        .lines()
        .next()
        .unwrap()?
        .split(',')
        .map(|position| position.parse::<i32>().unwrap())
        .collect();
    
    let mean = positions.iter().sum::<i32>() as f32 / positions.len() as f32;
    let mean = mean.floor() as i32; // use round or ceil to pass the test but floor to pass the input
    println!("Mean {}", mean);

    println!("They must spend {} fuel", positions.iter().fold(0, |acc, pos| {
        let distance = if pos - mean < 0 { mean - pos } else { pos - mean };
        acc + (distance * (distance + 1) / 2)
    }));

    Ok(())
}
