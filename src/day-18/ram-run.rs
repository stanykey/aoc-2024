use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl FromStr for Point {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let values: Vec<&str> = input.split(',').collect();
        let x = values[0].parse::<isize>();
        let y = values[1].parse::<isize>();

        if let (Ok(x), Ok(y)) = (x, y) {
            Ok(Point { x, y })
        } else {
            Err("Invalid point record".to_string())
        }
    }
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }

    // find all the neighbors of the given point that satisfy the filter.
    fn neighbors<F>(&self, filter: F) -> Vec<Point>
    where
        F: Fn(&Point) -> bool,
    {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .map(|(x, y)| Point {
                x: self.x + x,
                y: self.y + y,
            })
            .filter(filter)
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct State {
    point: Point,
    cost: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl State {
    fn new(point: Point, cost: usize) -> Self {
        State { point, cost }
    }
}

fn dijkstra<Filter>(start: &Point, destination: &Point, filter: Filter) -> Option<usize>
where
    Filter: Fn(&Point) -> Vec<Point>,
{
    let mut visited = HashMap::new();
    let mut frontier = BinaryHeap::new();
    frontier.push(State::new(start.clone(), 0));

    while let Some(State { point, cost }) = frontier.pop() {
        if &point == destination {
            return Some(cost);
        }

        if let Some(&prev_cost) = visited.get(&point) {
            if cost >= prev_cost {
                continue;
            }
        } else {
            visited.insert(point.clone(), cost);
        }

        for next in filter(&point) {
            frontier.push(State::new(next.clone(), cost + 1));
        }
    }

    // no path found.
    None
}

struct MemoryGrid {
    width: usize,
    height: usize,
    corruptions: Vec<Point>,
}

impl MemoryGrid {
    fn new(width: usize, height: usize, corruptions: Vec<Point>) -> Self {
        Self {
            width,
            height,
            corruptions,
        }
    }

    fn print(&self) {
        let mut grid = vec![vec!['.'; self.width]; self.height];
        for point in &self.corruptions {
            grid[point.y as usize][point.x as usize] = '#';
        }

        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", grid[y][x]);
            }
            println!();
        }
    }

    fn find_shortest_path(&self, start: &Point, end: &Point, bytes_count: usize) -> Option<usize> {
        if start == end {
            return Some(0);
        }

        let corruptions = self
            .corruptions
            .iter()
            .take(bytes_count)
            .map(|point| point.clone())
            .collect::<HashSet<Point>>();

        let allowed = |point: &Point| {
            point
                .neighbors(|point: &Point| {
                    point.x >= 0
                        && point.x < self.width as isize
                        && point.y >= 0
                        && point.y < self.height as isize
                        && !corruptions.contains(point)
                })
                .into_iter()
                .collect::<Vec<Point>>()
        };

        dijkstra(&start, &end, allowed)
    }

    fn find_first_blocker(&self, start: &Point, end: &Point, offset: usize) -> Option<Point> {
        for i in offset..self.corruptions.len() {
            match self.find_shortest_path(&start, &end, i) {
                Some(_) => continue,
                None => return Some(self.corruptions[i - 1]),
            }
        }

        None
    }
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| Point::from_str(line).unwrap())
        .collect()
}

fn main() {
    let input = include_str!("input.data");
    const MEMORY_GRID_WIDTH: usize = 71;
    const MEMORY_GRID_HEIGHT: usize = 71;
    const CORRUPTIONS_TO_PROCESS: usize = 1024;

    let corruptions = parse_input(input);
    let memory = MemoryGrid::new(MEMORY_GRID_WIDTH, MEMORY_GRID_HEIGHT, corruptions);
    memory.print();

    let start = Point::new(0, 0);
    let end = Point::new(
        (MEMORY_GRID_WIDTH - 1) as isize,
        (MEMORY_GRID_HEIGHT - 1) as isize,
    );
    match memory.find_shortest_path(&start, &end, CORRUPTIONS_TO_PROCESS) {
        Some(steps) => {
            println!(
                "The minimum number of steps needed to reach the exit is {}",
                steps
            );
        }
        None => {
            println!("No path found");
        }
    }

    match memory.find_first_blocker(&start, &end, CORRUPTIONS_TO_PROCESS) {
        Some(point) => {
            println!(
                "The minimum number of steps needed to reach the exit is ({}, {})",
                point.x, point.y
            );
        }
        None => {
            println!("No path found");
        }
    }
}
