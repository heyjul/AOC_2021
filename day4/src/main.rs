use std::{
    fs::File,
    io::{self, BufRead},
};

#[derive(Debug)]
struct Cell {
    value: u32,
    found: bool,
}

impl Cell {
    fn new(value: u32) -> Self {
        Self {
            value,
            found: false,
        }
    }
}

#[derive(Debug)]
struct Board {
    cells: Vec<Cell>,
    completed: bool,
}

impl Board {
    fn new() -> Self {
        Self { cells: vec![], completed: false }
    }

    fn len(&self) -> u32 {
        (self.cells.len() as f64).sqrt() as u32
    }

    fn completed_line(&self) -> bool {
        for i in 0..self.len() {
            let mut valid = true;
            for y in 0..self.len() {
                if !self.cells[(i * self.len() + y) as usize].found {
                    valid = false;
                    break;
                }
            }
            if valid {
                return true;
            }
        }

        for i in 0..self.len() {
            let mut valid = true;
            for y in 0..self.len() {
                if !self.cells[(y * self.len() + i) as usize].found {
                    valid = false;
                    break;
                }
            }
            if valid {
                return true;
            }
        }
        false
    }

    fn found(&mut self, value: u32) -> bool {
        if let Some(x) = self.cells.iter_mut().find(|cell| cell.value == value) {
            x.found = true;
            return true;
        }
        false
    }

    fn not_found_sum(&self) -> u32 {
        self.cells.iter().filter(|&cell| !cell.found).map(|cell| cell.value).sum::<u32>()
    }
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("./src/day4.input")?;
    let mut lines = io::BufReader::new(file).lines();
    let draw = lines
        .next()
        .unwrap()?;

    lines.next();

    let mut game: Vec<Board> = Vec::new();
    let mut board = Board::new();
    for line in lines {
        let line = line.unwrap();
        if line.is_empty() {
            game.push(board);
            board = Board::new();
        }
        let cells: Vec<Cell> = line.split(' ').filter(|&value| !value.is_empty()).map(|value| Cell::new(value.parse::<u32>().unwrap())).collect();
        board.cells.extend(cells);
    }
    game.push(board);

    draw.split(',').map(|val| val.parse::<u32>().unwrap()).for_each(|draw| {
        game.iter_mut().for_each(|board| {
            if !board.completed && board.found(draw) {
                /* println!("FOUND {} on board {:?}", draw, board); */
                if board.completed_line() {
                    board.completed = true;
                    println!("BINGO : {}", board.not_found_sum() * draw);
                }
            }
        })
    });
    
    Ok(())
}
