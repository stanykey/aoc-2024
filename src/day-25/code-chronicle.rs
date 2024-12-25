use std::str::FromStr;

#[derive(Debug)]
struct Schematic {
    is_lock: bool,
    heights: Vec<usize>,
}

impl Schematic {
    fn fits_lock_as_key(&self, lock: &Schematic) -> bool {
        self.heights
            .iter()
            .zip(&lock.heights)
            .all(|(&key_height, &lock_height)| key_height + lock_height <= 5)
    }
}

impl FromStr for Schematic {
    type Err = std::convert::Infallible;

    fn from_str(section: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = section.lines().collect();

        let is_lock = lines.first().unwrap().chars().all(|symbol| symbol == '#');

        let mut heights = vec![0; 5];
        for line in lines.iter().skip(1).take(5) {
            for col in 0..5 {
                if line.chars().nth(col).unwrap() == '#' {
                    heights[col] += 1;
                }
            }
        }

        Ok(Schematic { is_lock, heights })
    }
}

#[derive(Debug)]
struct LockSystem {
    locks: Vec<Schematic>,
    keys: Vec<Schematic>,
}

impl LockSystem {
    fn count_fitting_pairs(&self) -> usize {
        self.locks
            .iter()
            .flat_map(|lock| {
                self.keys
                    .iter()
                    .filter(move |key| key.fits_lock_as_key(lock))
            })
            .count()
    }
}

impl FromStr for LockSystem {
    type Err = std::convert::Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut locks = Vec::new();
        let mut keys = Vec::new();

        let normalized_input = input.replace("\r\n", "\n");
        for section in normalized_input.split("\n\n") {
            let schematic = Schematic::from_str(section).expect("Failed to parse schematic");
            if schematic.is_lock {
                locks.push(schematic);
            } else {
                keys.push(schematic);
            }
        }

        Ok(LockSystem { locks, keys })
    }
}

fn main() {
    let input = include_str!("input.data");
    match input.parse::<LockSystem>() {
        Ok(system) => {
            let timer = std::time::Instant::now();
            let count = system.count_fitting_pairs();
            println!("Number of unique lock/key pairs: {}", count);
            println!("The time spent is {:?}", timer.elapsed());
        }
        Err(err) => {
            eprintln!("Error parsing input: {}", err);
        }
    }
}
