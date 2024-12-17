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
    fn rescale_map(&mut self) {
        let mut new_map = vec![];

        for row in &self.map {
            let mut new_row = vec![];
            for &tile in row {
                match tile {
                    '#' => new_row.extend(vec!['#', '#']),
                    'O' => new_row.extend(vec!['[', ']']),
                    '.' => new_row.extend(vec!['.', '.']),
                    '@' => new_row.extend(vec!['@', '.']),
                    _ => new_row.extend(vec![tile, tile]),
                }
            }
            new_map.push(new_row);
        }

        self.map = new_map;
        // Adjust robot's position to account for the doubled width
        self.robot = (self.robot.0 * 2, self.robot.1);
    }

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

        let box_start_pos = match (self.map[y][x], instruction) {
            ('[', Instruction::Up | Instruction::Down) => Some((x + 1, y)), // Right partner
            (']', Instruction::Up | Instruction::Down) => Some((x - 1, y)), // Left partner
            _ => None,
        };

        let box_end_pos = box_start_pos.map(|(x, y)| instruction.apply(x, y));

        match (self.map[new_y][new_x], box_start_pos, box_end_pos) {
            ('#', _, _) => return None,
            (_, _, Some((x, y))) if self.map[y][x] == '#' => return None,
            ('.', None, None) => return Some(vec![(x, y)]),
            ('.', Some((other_x, other_y)), Some((other_new_x, other_new_y)))
                if self.map[other_new_y][other_new_x] == '.' =>
            {
                return Some(vec![(x, y), (other_x, other_y)]);
            }
            _ => (),
        }

        let mut all_moves = vec![];
        if self.map[new_y][new_x] != '.' {
            all_moves.extend(self.can_move(new_x, new_y, instruction)?);
        }

        if let Some((other_new_x, other_new_y)) = box_end_pos {
            if self.map[other_new_y][other_new_x] != '.' {
                all_moves.extend(self.can_move(other_new_x, other_new_y, instruction)?);
            }
        }

        all_moves.push((x, y));
        if let Some((other_x, other_y)) = box_start_pos {
            all_moves.push((other_x, other_y));
        }

        Some(all_moves)
    }

    fn gps(&self) -> Vec<(usize, usize)> {
        self.map
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter(|&(_, symbol)| *symbol == 'O' || *symbol == '[')
                    .map(move |(x, _)| (x, y))
            })
            .collect()
    }

    fn gps_score(&self) -> usize {
        self.gps().iter().map(|(x, y)| y * 100 + x).sum()
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

    // part 1
    warehouse.apply_all(&instructions);
    println!("Part 1 - GPS sum: {}", warehouse.gps_score());

    // part 2
    let mut warehouse = Warehouse::from_str(map).expect("Failed to parse map");
    warehouse.rescale_map();
    warehouse.apply_all(&instructions);
    println!("Part 2 - GPS sum: {}", warehouse.gps_score());
}
