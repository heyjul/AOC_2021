use std::{fs::File, io::{self, BufRead}};

fn main() -> std::io::Result<()> {
    let file = File::open("./src/day2.input")?;
    let commands = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .map(|x| x.split(' ').map(|x| x.to_string()).collect::<Vec<String>>());

    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;
    for command in commands {
        let value = command[1].parse::<i32>().unwrap();
        match &command[0][..] {
            "forward" => {
                horizontal_position += value;
                depth += value * aim;
            },
            "down" => aim += value,
            "up" => aim -= value,
            _ => {},
       }
    }
    
    println!("The final result is {}", depth * horizontal_position);

    Ok(())
}