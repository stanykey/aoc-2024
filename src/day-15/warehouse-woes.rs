use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Instruction {
    fn from(instruction: char) -> Self {
        match instruction {
            '^' => Instruction::Up,
            'v' => Instruction::Down,
            '<' => Instruction::Left,
            '>' => Instruction::Right,
            _ => unreachable!(),
        }
    }
}

impl Instruction {
    fn apply(&self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Instruction::Up => (x, y - 1),
            Instruction::Down => (x, y + 1),
            Instruction::Left => (x - 1, y),
            Instruction::Right => (x + 1, y),
        }
    }
}

#[derive(Debug)]
struct Warehouse {
    map: Vec<Vec<char>>,
    robot: (usize, usize),
}

impl FromStr for Warehouse {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut map = vec![];
        let mut robot = (0, 0);
        for (y, line) in input.lines().enumerate() {
            map.push(vec![]);
            for (x, symbol) in line.chars().enumerate() {
                map[y].push(symbol);
                if symbol == '@' {
                    robot = (x, y);
                }
            }
        }
        Ok(Self { map, robot })
    }
}

impl Warehouse {
    fn apply_all(&mut self, instructions: &[Instruction]) {
        instructions.iter().for_each(|instruction| {
            self.apply_single(instruction);
        });
    }

    fn apply_single(&mut self, instruction: &Instruction) {
        let (x, y) = self.robot;
        let (new_x, new_y) = instruction.apply(x, y);

        if self.map[new_y][new_x] == '#' {
            return; // wall
        } else if self.map[new_y][new_x] == '.' {
            // empty space - go into
            self.map[y][x] = '.';
            self.map[new_y][new_x] = '@';
            self.robot = (new_x, new_y);
            return;
        }

        // try to move the box(es).
        self.try_move_box(new_x, new_y, instruction);
        if self.map[new_y][new_x] == '.' {
            // check we have free space after shift
            self.map[y][x] = '.';
            self.map[new_y][new_x] = '@';
            self.robot = (new_x, new_y);
        }
    }

    fn try_move_box(&mut self, x: usize, y: usize, instruction: &Instruction) {
        if let Some(moves) = self.can_move(x, y, instruction) {
            let moves = moves.into_iter().unique().collect::<Vec<_>>();
            for (x, y) in moves {
                let (new_x, new_y) = instruction.apply(x, y);
                (self.map[y][x], self.map[new_y][new_x]) = (self.map[new_y][new_x], self.map[y][x]);
            }
        }
    }

    fn can_move(
        &self,
        x: usize,
        y: usize,
        instruction: &Instruction,
    ) -> Option<Vec<(usize, usize)>> {
        let (new_x, new_y) = instruction.apply(x, y);

        match self.map[new_y][new_x] {
            '#' => None,               // blocked
            '.' => Some(vec![(x, y)]), // valid move, stop here
            _ => {
                // recursive case: keep moving
                let mut all_moves = self.can_move(new_x, new_y, instruction)?;
                all_moves.push((x, y));
                Some(all_moves)
            }
        }
    }

    fn gps(&self) -> Vec<(usize, usize)> {
        self.map
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter(|&(_, symbol)| *symbol == 'O')
                    .map(move |(x, _)| (x, y))
            })
            .collect()
    }
}

fn main() {
    let input = include_str!("input.data");
    let (map, instructions) = input.split_once("\r\n\r\n").unwrap();

    let mut warehouse = Warehouse::from_str(map).expect("Failed to parse map");
    let instructions: Vec<Instruction> = instructions
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(Instruction::from)
        .collect();

    warehouse.apply_all(&instructions);
    println!(
        "The value for gps cordinates: {}",
        warehouse
            .gps()
            .iter()
            .map(|(x, y)| y * 100 + x)
            .sum::<usize>()
    );
}
