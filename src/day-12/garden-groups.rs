use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Display;
use std::ops::Add;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Plot {
    row: isize,
    col: isize,
}

static NEIGHBORS: [Plot; 4] = [
    Plot { row: 0, col: -1 }, // left
    Plot { row: 0, col: 1 },  // right
    Plot { row: 1, col: 0 },  // down
    Plot { row: -1, col: 0 }, // up
];

static CORNERS: [[Plot; 3]; 4] = [
    [
        Plot { col: -1, row: -1 },
        Plot { col: -1, row: 0 },
        Plot { col: 0, row: -1 },
    ],
    [
        Plot { col: 1, row: -1 },
        Plot { col: 1, row: 0 },
        Plot { col: 0, row: -1 },
    ],
    [
        Plot { col: 1, row: 1 },
        Plot { col: 1, row: 0 },
        Plot { col: 0, row: 1 },
    ],
    [
        Plot { col: -1, row: 1 },
        Plot { col: -1, row: 0 },
        Plot { col: 0, row: 1 },
    ],
];

impl Plot {
    fn new(row: usize, col: usize) -> Self {
        Plot {
            row: row as isize,
            col: col as isize,
        }
    }
}

impl Display for Plot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

impl Add for Plot {
    type Output = Plot;

    fn add(self, other: Plot) -> Plot {
        Plot {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

struct Region {
    plant: char,
    plots: HashSet<Plot>,
}

impl Region {
    fn area(&self) -> usize {
        self.plots.len()
    }
}

impl Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let plots: Vec<String> = self.plots.iter().map(|plot| format!("{plot}")).collect();
        write!(
            f,
            "Region {{ plant: '{}', plots: [{}] }}",
            self.plant,
            plots.join(", ")
        )
    }
}

#[derive(Debug)]
struct Garden {
    plots: HashMap<Plot, char>,
}

impl FromStr for Garden {
    type Err = String;
    fn from_str(garden_map: &str) -> Result<Self, Self::Err> {
        let plots = garden_map
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(col, plant)| (Plot::new(row, col), plant))
            })
            .collect();
        Ok(Garden { plots })
    }
}

impl Garden {
    fn get_regions(&self) -> Vec<Region> {
        let mut regions = Vec::new();

        let mut visited: HashSet<Plot> = HashSet::new();
        for plot in self.plots.keys() {
            if visited.contains(plot) {
                continue;
            }

            regions.push(self.get_region(plot, &mut visited));
        }

        regions
    }

    fn get_region(&self, start: &Plot, visited: &mut HashSet<Plot>) -> Region {
        let region_plant = self.plots[&start];
        let mut region_plots = HashSet::new();

        let mut stack = VecDeque::new();
        stack.push_back(*start);
        while let Some(plot) = stack.pop_front() {
            if visited.contains(&plot) || self.plots[&plot] != region_plant {
                continue;
            }
            visited.insert(plot);
            region_plots.insert(plot);

            for neighbor in self.neighbors(&plot) {
                stack.push_back(neighbor);
            }
        }

        Region {
            plant: region_plant,
            plots: region_plots,
        }
    }

    fn neighbors(&self, plot: &Plot) -> Vec<Plot> {
        NEIGHBORS
            .iter()
            .map(|dir| *plot + *dir)
            .filter(|neighbor| self.plots.contains_key(neighbor))
            .collect()
    }

    fn perimeter(&self, region: &Region) -> usize {
        region
            .plots
            .iter()
            .map(|plot| {
                4 - self
                    .neighbors(plot)
                    .iter()
                    .filter(|neighbor| self.plots[neighbor] == self.plots[plot])
                    .count()
            })
            .sum::<usize>()
    }

    fn corners(&self, region: &Region) -> usize {
        region
            .plots
            .iter()
            .map(|plot| {
                CORNERS
                    .iter()
                    .filter(|corner| {
                        let opposite = self.plots.get(&(*plot + corner[0]));
                        let first = self.plots.get(&(*plot + corner[1]));
                        let second = self.plots.get(&(*plot + corner[2]));
                        let plot_char = self.plots.get(plot);
                        (plot_char != second && plot_char != first)
                            || (plot_char == second && plot_char == first && plot_char != opposite)
                    })
                    .count()
            })
            .sum::<usize>()
    }

    fn calculate_price_by_perimeter_policy(&self) -> usize {
        self.get_regions()
            .iter()
            .map(|region| self.perimeter(region) * region.area())
            .sum()
    }

    fn calculate_price_by_sides_policy(&self) -> usize {
        self.get_regions()
            .iter()
            .map(|region| self.corners(&region) * region.area())
            .sum()
    }
}

fn main() {
    let puzzle_input = include_str!("input.data");
    let garden = Garden::from_str(puzzle_input).expect("Failed to parse garden map");

    // let regions = garden.get_regions();
    // for region in &regions {
    //     println!("{region:}");
    // }

    let timer = std::time::Instant::now();
    println!(
        "Total garden price (by perimeter) is {}",
        garden.calculate_price_by_perimeter_policy()
    );
    println!("Time elapsed: {:?}", timer.elapsed());

    let timer = std::time::Instant::now();
    println!(
        "Total garden price (by sides) is {}",
        garden.calculate_price_by_sides_policy()
    );
    println!("Time elapsed: {:?}", timer.elapsed());
}
