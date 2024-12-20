use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn neighbors(&self) -> Vec<Point> {
        vec![
            Point::new(self.x - 1, self.y),
            Point::new(self.x + 1, self.y),
            Point::new(self.x, self.y - 1),
            Point::new(self.x, self.y + 1),
        ]
    }

    fn distance(&self, other: &Point) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
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

fn dijkstra<Filter>(start: &Point, destination: &Point, filter: Filter) -> Option<Vec<Point>>
where
    Filter: Fn(&Point) -> Vec<Point>,
{
    let mut visited = HashMap::new();
    let mut predecessors = HashMap::new();
    let mut frontier = BinaryHeap::new();
    frontier.push(State::new(start.clone(), 0));

    while let Some(State { point, cost }) = frontier.pop() {
        if let Some(&prev_cost) = visited.get(&point) {
            if cost > prev_cost {
                continue;
            }
        }

        visited.insert(point.clone(), cost);

        if &point == destination {
            let mut path = Vec::new();
            let mut current = Some(point);
            while let Some(p) = current {
                path.push(p);
                current = predecessors.get(&p).cloned();
            }
            path.reverse();
            return Some(path);
        }

        for next in filter(&point) {
            let new_cost = cost + 1;
            if !visited.contains_key(&next) || new_cost < *visited.get(&next).unwrap() {
                visited.insert(next.clone(), new_cost);
                predecessors.insert(next.clone(), point.clone());
                frontier.push(State::new(next.clone(), new_cost));
            }
        }
    }

    None
}

struct Racetrack {
    nodes: HashSet<Point>,
    start: Point,
    end: Point,
}

impl FromStr for Racetrack {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut nodes = HashSet::new();
        let mut start = Point::new(0, 0);
        let mut end = Point::new(0, 0);

        for (y, line) in input.lines().enumerate() {
            for (x, symbol) in line.chars().enumerate() {
                let point = Point::new(x as isize, y as isize);
                match symbol {
                    '.' => {
                        nodes.insert(point);
                    }
                    'S' => {
                        start = point;
                        nodes.insert(point);
                    }
                    'E' => {
                        end = point;
                        nodes.insert(point);
                    }
                    _ => {}
                }
            }
        }

        Ok(Racetrack { nodes, start, end })
    }
}

impl Racetrack {
    fn normal_path(&self) -> Option<Vec<Point>> {
        let allowed = |point: &Point| self.neighbors(point);

        dijkstra(&self.start, &self.end, allowed)
    }

    fn neighbors(&self, point: &Point) -> Vec<Point> {
        point
            .neighbors()
            .into_iter()
            .filter(|point| self.nodes.contains(point))
            .collect()
    }
}

fn main() {
    let input = include_str!("input.data");
    let racetrack = Racetrack::from_str(input.trim()).expect("Failed to parse input");

    let path = racetrack.normal_path().expect("Failed to find path");
    let distances = path
        .iter()
        .rev()
        .enumerate()
        .map(|(distance, &point)| (point, distance))
        .collect::<Vec<_>>();

    let time_delta = 100;
    let cheat_count = distances
        .iter()
        .enumerate()
        .map(|(idx, (first, first_distance))| {
            distances
                .iter()
                .skip(idx + 1)
                .filter(|(second, second_distance)| {
                    let distance = first.distance(second);
                    distance == 2 && *second_distance - first_distance - distance >= time_delta
                })
                .count()
        })
        .sum::<usize>();

    println!(
        "There are {} cheats would save you at least {} picoseconds",
        cheat_count, time_delta
    );
}
