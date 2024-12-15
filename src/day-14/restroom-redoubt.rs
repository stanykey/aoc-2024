use std::fmt::Display;
use std::ops::Add;
use std::str::FromStr;

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Display for Point {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "({}, {})", self.x, self.y)
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(record: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = record.split(['=', ',']).collect();
        if parts.len() != 3 {
            return Err(format!("Invalid point format: {}", record));
        }

        if let (Ok(x), Ok(y)) = (parts[1].parse(), parts[2].parse()) {
            return Ok(Point::new(x, y));
        }

        Err(format!("Invalid values: {}", record))
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

#[derive(Clone)]
struct Robot {
    position: Point,
    velocity: Point,
}

impl Display for Robot {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "(position = {}, velocity = {})",
            self.position, self.velocity
        )
    }
}

impl FromStr for Robot {
    type Err = String;

    fn from_str(record: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = record.split_whitespace().collect();
        if parts.len() != 2 {
            return Err(format!("Invalid robot record format: {}", record));
        }

        let points = (Point::from_str(parts[0]), Point::from_str(parts[1]));
        if let (Ok(position), Ok(velocity)) = (points.0, points.1) {
            return Ok(Robot { position, velocity });
        }

        Err(format!("Invalid robot record format: {}", record))
    }
}

impl Robot {
    fn step(&mut self, count: usize, room_width: usize, room_height: usize) {
        let new_x = self.position.x + self.velocity.x * count as i32;
        let new_y = self.position.y + self.velocity.y * count as i32;
        self.position.x = new_x.rem_euclid(room_width as i32);
        self.position.y = new_y.rem_euclid(room_height as i32);
    }

    fn quadrant(&self, room_width: usize, room_height: usize) -> Option<usize> {
        let x = self.position.x;
        let y = self.position.y;
        let x_bound = room_width as i32;
        let y_bound = room_height as i32;
        if x < x_bound / 2 && y < y_bound / 2 {
            Some(0)
        } else if x > x_bound / 2 && y < y_bound / 2 {
            Some(1)
        } else if x < x_bound / 2 && y > y_bound / 2 {
            Some(2)
        } else if x > x_bound / 2 && y > y_bound / 2 {
            Some(3)
        } else {
            None
        }
    }
}

#[derive(Clone)]
struct Room {
    width: usize,
    height: usize,
    robots: Vec<Robot>,
}

impl Room {
    fn parse(width: usize, height: usize, records: &str) -> Result<Room, String> {
        let robots = records
            .lines()
            .map(|line| {
                Robot::from_str(line).map_err(|error| format!("Error parsing robot: {}", error))
            })
            .collect::<Result<Vec<Robot>, String>>()?;

        Ok(Self {
            width,
            height,
            robots,
        })
    }

    fn simulate(&self, steps: usize) -> Room {
        let mut room = self.clone();
        room.robots
            .iter_mut()
            .for_each(|robot| robot.step(steps, room.width, room.height));
        room
    }

    fn get_safety_factor(&self) -> usize {
        self.robots
            .iter()
            .filter_map(|robot| robot.quadrant(self.width, self.height))
            .fold([0; 4], |mut acc, quadrant_index| {
                acc[quadrant_index] += 1;
                acc
            })
            .iter()
            .product::<usize>()
    }

    #[allow(dead_code)]
    fn print_state(&self) {
        // create a 2D vector to represent the grid
        let mut grid = vec![vec![0; self.width]; self.height];

        // populate the grid with the number of robots at each position
        for robot in &self.robots {
            let x = robot.position.x.rem_euclid(self.width as i32) as usize;
            let y = robot.position.y.rem_euclid(self.height as i32) as usize;
            grid[y][x] += 1;
        }

        // print the grid
        for row in grid.iter() {
            for &cell in row {
                if cell == 0 {
                    print!("[.]");
                } else {
                    print!("[{}]", cell);
                }
            }
            println!(); // newline after each row
        }
    }
}

fn main() {
    let puzzle_input = include_str!("input.data");

    // let width = 11;
    // let height = 7;
    // let room = Room::parse(width, height, puzzle_input.trim()).expect("Failed to parse room data");

    let width = 101;
    let height = 103;
    let room = Room::parse(width, height, puzzle_input.trim()).expect("Failed to parse room data");

    let timer = std::time::Instant::now();
    let new_room = room.simulate(100);
    let safety_factor = new_room.get_safety_factor();
    // new_room.print_state();
    println!("The safety factor after 100 seconds is {safety_factor}",);
    println!("Time elapsed: {:?}", timer.elapsed());
}
