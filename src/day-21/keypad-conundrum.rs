use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

struct PathContext {
    numeric_paths: HashMap<(char, char), Vec<String>>,
    direction_paths: HashMap<(char, char), Vec<String>>,
    memo: HashMap<(String, usize, bool), usize>,
}

impl PathContext {
    fn new() -> Self {
        let numeric_keypad: HashMap<char, Vec<(char, char)>> = [
            ('7', vec![('4', 'v'), ('8', '>')]),
            ('8', vec![('5', 'v'), ('9', '>'), ('7', '<')]),
            ('9', vec![('6', 'v'), ('8', '<')]),
            ('4', vec![('1', 'v'), ('5', '>'), ('7', '^')]),
            ('5', vec![('2', 'v'), ('6', '>'), ('4', '<'), ('8', '^')]),
            ('6', vec![('3', 'v'), ('5', '<'), ('9', '^')]),
            ('1', vec![('2', '>'), ('4', '^')]),
            ('2', vec![('3', '>'), ('5', '^'), ('1', '<'), ('0', 'v')]),
            ('0', vec![('2', '^'), ('A', '>')]),
            ('3', vec![('6', '^'), ('2', '<'), ('A', 'v')]),
            ('A', vec![('0', '<'), ('3', '^')]),
        ]
        .into_iter()
        .collect();

        let direction_keypad: HashMap<char, Vec<(char, char)>> = [
            ('^', vec![('A', '>'), ('v', 'v')]),
            ('A', vec![('^', '<'), ('>', 'v')]),
            ('>', vec![('A', '^'), ('v', '<')]),
            ('<', vec![('v', '>')]),
            ('v', vec![('<', '<'), ('^', '^'), ('>', '>')]),
        ]
        .into_iter()
        .collect();

        PathContext {
            numeric_paths: Self::find_all_paths(&numeric_keypad),
            direction_paths: Self::find_all_paths(&direction_keypad),
            memo: HashMap::new(),
        }
    }

    fn find_all_paths(
        keypad: &HashMap<char, Vec<(char, char)>>,
    ) -> HashMap<(char, char), Vec<String>> {
        let mut paths = HashMap::new();
        for &first in keypad.keys() {
            for &second in keypad.keys() {
                let path = Self::find_shortest_paths(keypad, first, second);
                paths.insert((first, second), path);
            }
        }
        paths
    }

    fn find_shortest_paths(
        neighbors: &HashMap<char, Vec<(char, char)>>,
        start: char,
        end: char,
    ) -> Vec<String> {
        let mut queue = VecDeque::new();
        queue.push_back((start, Vec::new(), HashSet::new()));

        let mut paths = Vec::new();
        let mut lowest = std::usize::MAX;
        while let Some((node, path, mut visited)) = queue.pop_front() {
            if node == end {
                if path.len() <= lowest {
                    lowest = path.len();
                    paths.push(path.iter().collect::<String>());
                }
                continue;
            }

            if !visited.contains(&node) {
                visited.insert(node);

                for (next, dir) in neighbors.get(&node).unwrap() {
                    let mut path = path.clone();
                    path.push(*dir);
                    queue.push_back((*next, path, visited.clone()));
                }
            }
        }

        paths
    }

    fn find_shortest_sequence(&mut self, sequence: String, depth: usize, numeric: bool) -> usize {
        if let Some(&cached) = self.memo.get(&(sequence.clone(), depth, numeric)) {
            return cached;
        }

        let paths = if numeric {
            self.numeric_paths.clone()
        } else {
            self.direction_paths.clone()
        };

        let result = ("A".to_string() + &sequence)
            .chars()
            .tuple_windows()
            .map(|(first, second)| {
                let shortest_paths = paths.get(&(first, second)).unwrap();
                match depth {
                    0 => shortest_paths[0].len() + 1,
                    _ => shortest_paths
                        .iter()
                        .cloned()
                        .map(|mut path| {
                            path.push('A');
                            self.find_shortest_sequence(path, depth - 1, false)
                        })
                        .min()
                        .unwrap(),
                }
            })
            .sum::<usize>();

        self.memo.insert((sequence, depth, numeric), result);
        result
    }
}

fn main() {
    let input = include_str!("input.data");
    println!("{}", input);

    let mut context = PathContext::new();

    let now = std::time::Instant::now();
    println!(
        "The sum of the complexities of the five codes on my list is {}",
        input
            .lines()
            .map(|line| {
                context.find_shortest_sequence(line.to_string(), 2, true)
                    * line.trim_end_matches('A').parse::<usize>().unwrap()
            })
            .sum::<usize>()
    );
    println!("The time spent is {:?}", now.elapsed());
}
