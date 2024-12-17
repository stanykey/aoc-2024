use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::str::FromStr;

#[derive(Eq, PartialEq, Hash)]
struct ReindeerState {
    point: Point,
    direction: Direction,
    cost: usize,
    path: Vec<Point>,
}

impl ReindeerState {
    fn new(point: Point, direction: Direction, cost: usize, path: Vec<Point>) -> Self {
        Self {
            point,
            direction,
            path,
            cost,
        }
    }
}

impl Ord for ReindeerState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for ReindeerState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn make_step(&self, point: &Point) -> Point {
        match self {
            Direction::North => Point::new(point.x, point.y - 1),
            Direction::South => Point::new(point.x, point.y + 1),
            Direction::West => Point::new(point.x - 1, point.y),
            Direction::East => Point::new(point.x + 1, point.y),
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn neighbors(&self, direction: Direction) -> Vec<(Point, Direction, usize)> {
        [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ]
        .into_iter()
        .filter(|d| *d != direction.opposite())
        .map(|d| {
            let cost = if direction != d { 1001 } else { 1 };
            (d.make_step(self), d, cost)
        })
        .collect()
    }
}

#[derive(Debug, Clone)]
struct Maze {
    nodes: HashSet<Point>,
    start: Point,
    end: Point,
}

impl FromStr for Maze {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut nodes = HashSet::new();
        let mut start = Point { x: 0, y: 0 };
        let mut end = Point { x: 0, y: 0 };

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == 'S' {
                    start = Point { x, y };
                } else if c == 'E' {
                    end = Point { x, y };
                }

                if c != '#' {
                    nodes.insert(Point { x, y });
                }
            }
        }

        Ok(Self { nodes, start, end })
    }
}

impl Maze {
    fn neighbors(&self, point: Point, direction: Direction) -> Vec<(Point, Direction, usize)> {
        point
            .neighbors(direction)
            .into_iter()
            .filter(|(neighbor, _, _)| self.nodes.contains(neighbor))
            .collect()
    }

    fn shortest_paths(&self) -> (usize, Vec<Vec<Point>>) {
        let mut paths = Vec::new();
        let mut best = usize::MAX;

        // https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
        let mut visited: HashMap<(Point, Direction), usize> = HashMap::new();
        let mut frontier = BinaryHeap::new();
        frontier.push(ReindeerState::new(
            self.start,
            Direction::East,
            0,
            vec![self.start],
        ));

        while let Some(ReindeerState {
            point,
            direction,
            path,
            cost,
        }) = frontier.pop()
        {
            if let Some(&prev_cost) = visited.get(&(point, direction)) {
                if cost > prev_cost {
                    continue;
                }
            } else {
                visited.insert((point, direction), cost);
            }

            if point == self.end && cost <= best {
                paths.push(path.clone());
                best = cost;
            }

            for (neighbor, new_direction, neighbor_cost) in self.neighbors(point, direction) {
                frontier.push(ReindeerState::new(
                    neighbor,
                    new_direction,
                    cost + neighbor_cost,
                    {
                        let mut path = path.clone();
                        path.push(neighbor);
                        path
                    },
                ));
            }
        }

        (best, paths)
    }
}

fn main() {
    let input = include_str!("input.data");

    let map = Maze::from_str(input).unwrap();
    let (shortest_length, paths) = map.shortest_paths();
    println!("The lowest possible score is {}", shortest_length);
    println!(
        "The number of tiles part of at least one best path is {}",
        paths.iter().flatten().unique().count()
    );
}
