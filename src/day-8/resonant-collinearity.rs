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

fn load_antennas_map(file_path: &Path) -> io::Result<Map> {
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

fn get_antinodes(lhs: Coordinate, rhs: Coordinate) -> (Coordinate, Coordinate) {
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

    (first, second)
}

fn find_all_antinodes(coordinates: &Vec<Coordinate>, map: &Map) -> HashSet<Coordinate> {
    coordinates
        .iter()
        .enumerate()
        .flat_map(move |(i, &lhs)| {
            coordinates.iter().skip(i + 1).flat_map(move |&rhs| {
                let (first, second) = get_antinodes(lhs, rhs);
                vec![first, second].into_iter().filter(move |coord| {
                    coord.x >= 0 && coord.y >= 0 && coord.x < map.width && coord.y < map.height
                })
            })
        })
        .collect::<HashSet<_>>()
}

fn count_unique_antinodes(map: &Map) -> usize {
    map.antennas
        .iter()
        .flat_map(|(_, coordinates)| find_all_antinodes(coordinates, map))
        .collect::<HashSet<_>>()
        .len()
}

fn main() -> io::Result<()> {
    let file_path = Path::new("input.data");
    let antennas_map = load_antennas_map(file_path)?;

    let timer = Instant::now();
    let unique_antinodes = count_unique_antinodes(&antennas_map);
    println!(
        "There are {} antinodes within the bounds of the map",
        unique_antinodes
    );
    println!("Time elapsed: {:?}", timer.elapsed());

    Ok(())
}
