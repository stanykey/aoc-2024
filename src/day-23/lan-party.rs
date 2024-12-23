use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

struct Graph {
    vertexes: Vec<String>,
    edges: HashMap<String, HashSet<String>>,
}

impl FromStr for Graph {
    type Err = std::convert::Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let graph = input.lines().filter_map(|line| line.split_once('-')).fold(
            HashMap::new(),
            |mut map, (first, second)| {
                map.entry(first.to_string())
                    .or_insert_with(HashSet::new)
                    .insert(second.to_string());
                map.entry(second.to_string())
                    .or_insert_with(HashSet::new)
                    .insert(first.to_string());
                map
            },
        );

        Ok(Graph {
            vertexes: graph.keys().cloned().collect(),
            edges: graph,
        })
    }
}

impl Graph {
    fn find_triads(&self) -> HashSet<Vec<String>> {
        let mut triads: HashSet<Vec<String>> = HashSet::new();

        for (first, first_connections) in self.edges.iter() {
            for second in first_connections {
                if let Some(second_connections) = self.edges.get(second) {
                    let common_neighbors: HashSet<_> =
                        first_connections.intersection(second_connections).collect();
                    for third in common_neighbors {
                        let mut triad = Vec::from([first.clone(), second.clone(), third.clone()]);
                        triad.sort(); // sort to ensure consistency
                        triads.insert(triad);
                    }
                }
            }
        }

        triads
    }

    fn neighbors(&self, vertex: &String) -> HashSet<String> {
        self.edges.get(vertex).cloned().unwrap_or_default()
    }

    // Bron–Kerbosch algorithm to find all maximal cliques
    fn bron_kerbosch(
        &self,
        r: HashSet<String>,
        p: HashSet<String>,
        x: HashSet<String>,
        cliques: &mut Vec<HashSet<String>>,
    ) {
        if p.is_empty() && x.is_empty() {
            if r.len() > 1 {
                // exclude single-element cliques
                cliques.push(r);
            }
            return;
        }

        let mut p_clone = p.clone();
        for v in p {
            let mut r_new = r.clone();
            r_new.insert(v.clone());

            let neighbors = self.neighbors(&v);
            self.bron_kerbosch(
                r_new,
                &p_clone & &neighbors, // intersection of P and neighbors of v
                &x & &neighbors,       // intersection of X and neighbors of v
                cliques,
            );

            p_clone.remove(&v);
            let mut x_clone = x.clone();
            x_clone.insert(v);
        }
    }

    fn find_all_cliques(&self) -> Vec<HashSet<String>> {
        let mut cliques = Vec::new();
        let r = HashSet::new();
        let p: HashSet<String> = self.vertexes.iter().cloned().collect();
        let x = HashSet::new();

        self.bron_kerbosch(r, p, x, &mut cliques);
        cliques
    }
}

fn filter_triads<Filter>(graph: &Graph, filter: Filter) -> HashSet<Vec<String>>
where
    Filter: Fn(&String) -> bool,
{
    graph
        .find_triads()
        .into_iter()
        .filter(|triad| triad.iter().any(|comp| filter(comp)))
        .collect()
}

fn find_largest_clique(graph: &Graph) -> HashSet<String> {
    graph
        .find_all_cliques()
        .into_iter()
        .max_by_key(|clique| clique.len())
        .unwrap()
}

fn main() {
    let input = include_str!("input.data");

    let graph = Graph::from_str(input).expect("failed to parse data");

    let timer = std::time::Instant::now();
    let filtered_triads = filter_triads(&graph, |name: &String| name.starts_with('t'));
    println!("Number of filtered triads: {}", filtered_triads.len());
    println!("The time spent is {:?}", timer.elapsed());

    let timer = std::time::Instant::now();
    println!(
        "Password: {}",
        find_largest_clique(&graph).into_iter().sorted().join(",")
    );
    println!("The time spent is {:?}", timer.elapsed());
}
