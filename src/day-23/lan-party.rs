use std::collections::HashMap;
use std::collections::HashSet;

fn load_network_map(input: &str) -> HashMap<String, HashSet<String>> {
    let mut map = HashMap::new();
    for line in input.lines() {
        let (first, second) = line.split_once('-').expect("invalid input");
        map.entry(first.to_string())
            .or_insert_with(HashSet::new)
            .insert(second.to_string());
        map.entry(second.to_string())
            .or_insert_with(HashSet::new)
            .insert(first.to_string());
    }
    map
}

fn find_triads(map: &HashMap<String, HashSet<String>>) -> HashSet<Vec<String>> {
    let mut triads: HashSet<Vec<String>> = HashSet::new();

    for (first, first_connections) in map {
        for (second, second_connections) in map {
            if first != second && first_connections.contains(second) {
                let common_neighbors: HashSet<_> = first_connections
                    .intersection(&second_connections)
                    .collect();
                for third in common_neighbors {
                    let mut triad: Vec<String> = vec![first.clone(), second.clone(), third.clone()];
                    triad.sort(); // sort to ensure consistency in the order
                    triads.insert(triad);
                }
            }
        }
    }

    triads
}

fn filter_triads_with_t(triads: HashSet<Vec<String>>) -> HashSet<Vec<String>> {
    triads
        .into_iter()
        .filter(|triad| triad.iter().any(|comp| comp.starts_with('t')))
        .collect()
}

fn main() {
    let input = include_str!("input.data");

    let map = load_network_map(input);

    let timer = std::time::Instant::now();
    let triads = find_triads(&map);
    let filtered_triads = filter_triads_with_t(triads);
    println!("Number of filtered triads: {}", filtered_triads.len());
    println!("The time spent is {:?}", timer.elapsed());
}
