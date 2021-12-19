use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(PartialEq, Eq, Hash, Debug)]
struct Coordinates {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Pair {
    start: Coordinates,
    end: Coordinates,
}

impl Pair {
    fn new(vec: &Vec<&str>) -> Self {
        let coord1: Vec<&str> = vec.get(0).unwrap().split(',').collect();
        let coord2: Vec<&str> = vec.get(1).unwrap().split(',').collect();

        Self {
            start: Coordinates {
                x: coord1[0].trim().parse::<u32>().unwrap(),
                y: coord1[1].trim().parse::<u32>().unwrap(),
            },
            end: Coordinates {
                x: coord2[0].trim().parse::<u32>().unwrap(),
                y: coord2[1].trim().parse::<u32>().unwrap(),
            },
        }
    }

    fn is_horizontal_or_vertical_or_diagonal(&self) -> bool {
        self.is_horizontal() || self.is_vertical() || self.is_diagonal()
    }

    fn is_horizontal(&self) -> bool {
        self.start.x == self.end.x
    }

    fn is_vertical(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_diagonal(&self) -> bool {
        self.start.x.max(self.end.x) - self.start.x.min(self.end.x) == self.start.y.max(self.end.y) - self.start.y.min(self.end.y)
    }

    fn cover(&self) -> Vec<Coordinates> {
        if self.is_diagonal() {
            let mut range_x: Box<dyn DoubleEndedIterator<Item = u32>> =
                Box::new(self.start.x.min(self.end.x)..=self.start.x.max(self.end.x));
            let mut range_y: Box<dyn DoubleEndedIterator<Item = u32>> =
                Box::new(self.start.y.min(self.end.y)..=self.start.y.max(self.end.y));

            if self.start.x > self.end.x {
                range_x = Box::new(range_x.rev());
            }
            if self.start.y > self.end.y {
                range_y = Box::new(range_y.rev());
            }
            return range_x
                .zip(range_y)
                .map(|coord| Coordinates {
                    x: coord.0,
                    y: coord.1,
                })
                .collect();
        }
        let x = if self.is_horizontal() {
            (self.start.y, self.end.y, self.start.x)
        } else {
            (self.start.x, self.end.x, self.start.y)
        };
        (x.0.min(x.1)..=x.0.max(x.1))
            .map(|val| Coordinates {
                x: if self.is_horizontal() { x.2 } else { val },
                y: if self.is_vertical() { x.2 } else { val },
            })
            .collect()
    }
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("./src/day5.input")?;
    let mut hash = HashMap::new();
    BufReader::new(file)
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let coords: Vec<&str> = line.split("->").collect();
            Pair::new(&coords)
        })
        .filter(|pair| pair.is_horizontal_or_vertical_or_diagonal())
        .for_each(|pair| {
            pair.cover().into_iter().for_each(|coord| {
                let entry = hash.entry(coord).or_insert(0);
                *entry += 1;
            });
        });

    let count = hash.values().into_iter().filter(|&&val| val > 1).count();

    print!("Two lines are overlapped at {} points", count);

    Ok(())
}
