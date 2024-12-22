struct SecretGenerator {}

impl SecretGenerator {
    fn new() -> Self {
        Self {}
    }

    fn next(&mut self, number: u64) -> u64 {
        let mut result = number;

        // Combine mix and prune with bitwise operations
        result = (result ^ (result << 6)) & 0xFFFFFF; // << 6 is * 64, & 0xFFFFFF is prune
        result = (result ^ (result >> 5)) & 0xFFFFFF; // >> 5 is / 32, & 0xFFFFFF is prune
        result = (result ^ (result << 11)) & 0xFFFFFF; // << 11 is * 2048, & 0xFFFFFF is prune

        result
    }

    fn nth(&mut self, number: u64, nth: usize) -> u64 {
        let mut number = number;
        for _ in 0..nth {
            number = self.next(number)
        }
        number
    }
}

fn load_secret_numbers(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|line| line.parse::<u64>().expect("Invalid puzzle input"))
        .collect()
}

fn main() {
    let input = include_str!("input.data");
    let secret_numbers = load_secret_numbers(input);

    let mut secret_generator = SecretGenerator::new();

    let timer = std::time::Instant::now();
    println!(
        "The sum of the 2000th secret number generated by each buyer is {}",
        secret_numbers
            .iter()
            .map(|&number| secret_generator.nth(number, 2000))
            .sum::<u64>()
    );
    println!("The time spent is {:?}", timer.elapsed());
}
