use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl fmt::Debug for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Map {
    width: i32,
    height: i32,
    antennas: HashMap<char, Vec<Coordinate>>,
}

impl Map {
    fn load(file_path: &Path) -> io::Result<Map> {
        let file = File::open(file_path)?;
        let lines: Vec<_> = io::BufReader::new(file).lines().collect::<Result<_, _>>()?;

        let mut antennas: HashMap<char, Vec<Coordinate>> = HashMap::new();
        let height = lines.len() as i32;
        let width = lines[0].len() as i32;

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch != '.' {
                    antennas
                        .entry(ch)
                        .or_insert_with(Vec::new)
                        .push(Coordinate {
                            x: x as i32,
                            y: y as i32,
                        });
                }
            }
        }

        Ok(Map {
            width,
            height,
            antennas,
        })
    }

    fn is_in_bound(&self, position: &Coordinate) -> bool {
        position.x >= 0 && position.y >= 0 && position.x < self.width && position.y < self.height
    }
}

fn get_first_antinodes(map: &Map, lhs: Coordinate, rhs: Coordinate) -> HashSet<Coordinate> {
    let dx = rhs.x - lhs.x;
    let dy = rhs.y - lhs.y;

    let first = Coordinate {
        x: lhs.x - dx,
        y: lhs.y - dy,
    };

    let second = Coordinate {
        x: rhs.x + dx,
        y: rhs.y + dy,
    };

    [first, second]
        .into_iter()
        .filter(|pos| map.is_in_bound(pos))
        .collect()
}

fn get_all_antinodes(map: &Map, lhs: Coordinate, rhs: Coordinate) -> HashSet<Coordinate> {
    let mut antinodes = HashSet::new();

    // Calculate direction vector
    let dx = rhs.x - lhs.x;
    let dy = rhs.y - lhs.y;

    // Forward antinodes (beyond rhs)
    let mut current = rhs;
    while map.is_in_bound(&current) {
        antinodes.insert(current);
        current = Coordinate {
            x: current.x + dx,
            y: current.y + dy,
        };
    }

    // Backward antinodes (before lhs)
    current = lhs;
    while map.is_in_bound(&current) {
        antinodes.insert(current);
        current = Coordinate {
            x: current.x - dx,
            y: current.y - dy,
        };
    }

    antinodes
}

fn find_all_antinodes<F>(
    coordinates: &Vec<Coordinate>,
    map: &Map,
    generator: &F,
) -> HashSet<Coordinate>
where
    F: Fn(&Map, Coordinate, Coordinate) -> HashSet<Coordinate>,
{
    coordinates
        .iter()
        .enumerate()
        .flat_map(move |(i, &lhs)| {
            coordinates
                .iter()
                .skip(i + 1)
                .flat_map(move |&rhs| generator(map, lhs, rhs))
        })
        .collect::<HashSet<_>>()
}

fn count_all_antinodes(map: &Map) -> usize {
    map.antennas
        .iter()
        .flat_map(|(_, coordinates)| find_all_antinodes(coordinates, map, &get_first_antinodes))
        .collect::<HashSet<_>>()
        .len()
}

fn count_all_antinodes_with_resonant_harmonics(map: &Map) -> usize {
    map.antennas
        .iter()
        .flat_map(|(_, coordinates)| find_all_antinodes(coordinates, map, &get_all_antinodes))
        .collect::<HashSet<_>>()
        .len()
}

fn main() -> io::Result<()> {
    let file_path = Path::new("input.data");
    let antennas_map = Map::load(file_path)?;

    let timer = Instant::now();
    let antinode_count = count_all_antinodes(&antennas_map);
    println!(
        "There are {} antinodes within the bounds of the map",
        antinode_count
    );
    println!("Time elapsed: {:?}", timer.elapsed());

    let timer = Instant::now();
    let antinode_count = count_all_antinodes_with_resonant_harmonics(&antennas_map);
    println!(
        "There are {} antinodes within the bounds of the map if resonant harmonics takes",
        antinode_count
    );
    println!("Time elapsed: {:?}", timer.elapsed());

    Ok(())
}
