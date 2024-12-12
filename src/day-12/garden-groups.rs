use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

#[derive(Debug)]
struct Group {
    label: char,
    squares: Vec<(usize, usize)>,
}

impl Group {
    fn perimeter(&self) -> usize {
        let squares: HashSet<_> = self.squares.iter().cloned().collect();
        let mut perimeter = 0;

        for &(y, x) in &self.squares {
            for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let neighbor = ((y as isize + dy) as usize, (x as isize + dx) as usize);
                if !squares.contains(&neighbor) {
                    perimeter += 1;
                }
            }
        }

        perimeter
    }

    fn area(&self) -> usize {
        self.squares.len()
    }

    fn price(&self) -> usize {
        self.perimeter() * self.area()
    }
}

#[derive(Debug)]
struct Garden {
    groups: Vec<Group>,
}

impl Default for Garden {
    fn default() -> Garden {
        Self {
            groups: Vec::default(),
        }
    }
}

impl FromStr for Garden {
    type Err = String;
    fn from_str(garden_map: &str) -> Result<Self, Self::Err> {
        let map: Vec<Vec<char>> = garden_map
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        Ok(Garden::create(map))
    }
}

impl Garden {
    fn create(map: Vec<Vec<char>>) -> Garden {
        let mut groups = Vec::new();

        let rows = map.len();
        let cols = map[0].len();
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut visited = vec![vec![false; cols]; rows];
        for y in 0..rows {
            for x in 0..cols {
                if !visited[y][x] {
                    let label = map[y][x];
                    let mut squares = Vec::new();
                    let mut queue = VecDeque::new();

                    queue.push_back((y, x));
                    visited[y][x] = true;
                    while let Some((row, col)) = queue.pop_front() {
                        squares.push((row, col));

                        for &(dy, dx) in &directions {
                            let ny = row as isize + dy;
                            let nx = col as isize + dx;

                            if ny >= 0
                                && ny < rows as isize
                                && nx >= 0
                                && nx < cols as isize
                                && !visited[ny as usize][nx as usize]
                                && map[ny as usize][nx as usize] == label
                            {
                                queue.push_back((ny as usize, nx as usize));
                                visited[ny as usize][nx as usize] = true;
                            }
                        }
                    }

                    groups.push(Group { label, squares });
                }
            }
        }

        Garden { groups }
    }

    fn total_price(&self) -> usize {
        self.groups.iter().map(|group| group.price()).sum()
    }
}

fn main() {
    let puzzle_input = include_str!("input.data");
    let garden = Garden::from_str(puzzle_input).expect("Failed to parse garden map");

    for group in &garden.groups {
        println!(
            "A region of {} plants with price {} * {} = {}",
            group.label,
            group.area(),
            group.perimeter(),
            group.price()
        );
    }
    println!("Total garden price is {}", garden.total_price());
}
