use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy)]
struct ClawMachine {
    a: Point,
    b: Point,
    prize: Point,
}

impl Display for ClawMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "a: {}, b: {}, prize: {}", self.a, self.b, self.prize)
    }
}

impl ClawMachine {
    fn count_cost_of_win(&self) -> Option<isize> {
        // we need to solce a system of linear Diophantine equations
        // some details at https://en.wikipedia.org/wiki/Diophantine_equation

        // solve for b.
        let num = self.a.y * self.prize.x - self.a.x * self.prize.y;
        let denom = self.a.y * self.b.x - self.a.x * self.b.y;

        // we can't divide by zero
        // the division must be exact because you either press the button
        if denom == 0 {
            return None;
        } else if num % denom != 0 {
            return None;
        }
        let b = num / denom;

        // solve for a.
        let num = self.prize.x - b * self.b.x;
        let denom = self.a.x;
        if denom == 0 {
            return None;
        } else if num % denom != 0 {
            return None;
        }
        let a = num / denom;

        Some(a * 3 + b)
    }
}

#[derive(Debug)]
struct Arcade {
    machines: Vec<ClawMachine>,
}

impl Display for Arcade {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for machine in &self.machines {
            writeln!(f, "{}", machine).expect("Failed to pring machine");
        }
        Ok(())
    }
}

impl FromStr for Arcade {
    type Err = String;

    fn from_str(record: &str) -> Result<Self, Self::Err> {
        let mut machines = Vec::new();
        let mut lines = record.lines().filter(|line| !line.is_empty());

        while let Some(button_a_line) = lines.next() {
            let button_b_line = lines.next().ok_or("Missing Button B line")?;
            let prize_line = lines.next().ok_or("Missing Prize line")?;

            let button_a = Self::parse_button(button_a_line);
            let button_b = Self::parse_button(button_b_line);
            let prize = Self::parse_prize(prize_line);

            if let (Ok(button_a), Ok(button_b), Ok(prize)) = (button_a, button_b, prize) {
                machines.push(ClawMachine {
                    a: button_a,
                    b: button_b,
                    prize,
                });
            } else {
                return Err("Invalid Arcade input".to_string());
            }
        }

        Ok(Arcade { machines })
    }
}

impl Arcade {
    fn parse_button(record: &str) -> Result<Point, String> {
        // Example: "Button A: X+94, Y+34"
        let parts: Vec<_> = record
            .split([' ', ':', '+', ','].as_ref())
            .filter(|part| !part.is_empty())
            .collect();

        if parts.len() < 5 {
            return Err(format!("Invalid button record: {}", record));
        }

        if let (Ok(x), Ok(y)) = (parts[3].parse::<isize>(), parts[5].parse::<isize>()) {
            Ok(Point { x, y })
        } else {
            Err(format!("Invalid button record: {}", record))
        }
    }

    fn parse_prize(record: &str) -> Result<Point, String> {
        // Example: "Prize: X=8400, Y=5400"
        let parts: Vec<_> = record
            .split([':', ' ', '=', ','].as_ref())
            .filter(|part| !part.is_empty())
            .collect();

        if parts.len() < 5 {
            return Err(format!("Invalid prize record: {}", record));
        }

        if let (Ok(x), Ok(y)) = (parts[2].parse::<isize>(), parts[4].parse::<isize>()) {
            Ok(Point { x, y })
        } else {
            Err(format!("Invalid prize record: {}", record))
        }
    }

    fn calculate_cost(&self) -> (usize, usize) {
        let mut total_cost = 0;
        let mut prizes_won = 0;

        for machine in &self.machines {
            if let Some(cost) = machine.count_cost_of_win() {
                total_cost += cost;
                prizes_won += 1;
            }
        }

        (prizes_won, total_cost as usize)
    }
}

fn main() {
    let puzzle_input = include_str!("input.data");
    let arcade = Arcade::from_str(puzzle_input.trim()).expect("Failed to parse arcade machines");
    println!("{}", arcade);

    let (prizes_won, total_cost) = arcade.calculate_cost();
    println!("Prizes won: {prizes_won}, Total cost: {total_cost} tokens");

    let arcade = Arcade {
        machines: arcade
            .machines
            .iter()
            .map(|machine| ClawMachine {
                a: machine.a,
                b: machine.b,
                prize: Point {
                    x: machine.prize.x + 10_000_000_000_000,
                    y: machine.prize.y + 10_000_000_000_000,
                },
            })
            .collect(),
    };

    let (prizes_won, total_cost) = arcade.calculate_cost();
    println!(
        "Prizes won: {}, Total cost: {} tokens",
        prizes_won, total_cost
    );
}
