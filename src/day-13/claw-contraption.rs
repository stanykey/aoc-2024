use std::str::FromStr;

#[derive(Debug)]
struct ClawMachine {
    button_a: (usize, usize),
    button_b: (usize, usize),
    prize: (usize, usize),
}

impl ClawMachine {
    fn calculate_min_cost(&self, max_press: usize) -> Option<usize> {
        let (a_x, a_y) = self.button_a;
        let (b_x, b_y) = self.button_b;
        let (p_x, p_y) = self.prize;

        let mut min_cost = None;

        for k in 0..=max_press {
            for m in 0..=max_press {
                // Check if the equations are satisfied
                if k * a_x + m * b_x == p_x && k * a_y + m * b_y == p_y {
                    let cost = 3 * k + m; // Calculate the cost
                    min_cost = Some(min_cost.map_or(cost, |c: usize| c.min(cost)));
                }
            }
        }

        min_cost
    }
}

struct Arcade {
    machines: Vec<ClawMachine>,
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
                    button_a,
                    button_b,
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
    fn parse_button(record: &str) -> Result<(usize, usize), String> {
        // Example: "Button A: X+94, Y+34"
        let parts: Vec<_> = record
            .split([' ', ':', '+', ','].as_ref())
            .filter(|part| !part.is_empty())
            .collect();

        if parts.len() < 5 {
            return Err(format!("Invalid button record: {}", record));
        }

        if let (Ok(x), Ok(y)) = (parts[3].parse::<usize>(), parts[5].parse::<usize>()) {
            Ok((x, y))
        } else {
            Err(format!("Invalid button record: {}", record))
        }
    }

    fn parse_prize(record: &str) -> Result<(usize, usize), String> {
        // Example: "Prize: X=8400, Y=5400"
        let parts: Vec<_> = record
            .split([':', ' ', '=', ','].as_ref())
            .filter(|part| !part.is_empty())
            .collect();

        if parts.len() < 5 {
            return Err(format!("Invalid prize record: {}", record));
        }

        if let (Ok(x), Ok(y)) = (parts[2].parse::<usize>(), parts[4].parse::<usize>()) {
            Ok((x, y))
        } else {
            Err(format!("Invalid prize record: {}", record))
        }
    }

    fn calculate_total_cost(&self, max_press: usize) -> (usize, usize) {
        let mut total_cost = 0;
        let mut prizes_won = 0;

        for machine in &self.machines {
            if let Some(cost) = machine.calculate_min_cost(max_press) {
                total_cost += cost;
                prizes_won += 1;
            }
        }

        (prizes_won, total_cost)
    }
}

fn main() {
    let puzzle_input = include_str!("input.data");
    let arcade = Arcade::from_str(puzzle_input.trim()).expect("Failed to parse arcade machines");
    let max_press_count = 100;

    for machine in &arcade.machines {
        let (ax, ay) = machine.button_a;
        let (bx, by) = machine.button_b;
        let (px, py) = machine.prize;
        let machine_description = format!(
            "ClawMachine(A = ({}, {}), B = ({}, {}), Prize = ({}, {}))",
            ax, ay, bx, by, px, py
        );

        if let Some(cost) = machine.calculate_min_cost(max_press_count) {
            println!("{}: min cost to win is {}", machine_description, cost);
        } else {
            println!(
                "{}: no win in allowed amount of attempts",
                machine_description
            );
        };
    }

    let (prizes_won, total_cost) = arcade.calculate_total_cost(max_press_count);
    println!(
        "Prizes won: {}, Total cost: {} tokens",
        prizes_won, total_cost
    );
}
