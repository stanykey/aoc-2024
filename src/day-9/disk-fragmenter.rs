use std::time::Instant;
use std::{io, num::ParseIntError, str::FromStr};

#[derive(Debug, Clone, Copy)]
struct Block {
    id: u64,
    used: u64,
    free: u64,
}

#[derive(Debug)]
struct Disk {
    blocks: Vec<Block>,
}

impl FromStr for Disk {
    type Err = ParseIntError;

    fn from_str(puzzle: &str) -> Result<Self, Self::Err> {
        let blocks = puzzle
            .trim()
            .chars()
            .collect::<Vec<char>>()
            .chunks(2)
            .enumerate()
            .map(|(id, chunk)| {
                let used = chunk[0].to_string().parse()?;
                let free = chunk.get(1).unwrap_or(&'0').to_string().parse()?;
                Ok(Block {
                    id: id as u64,
                    used,
                    free,
                })
            })
            .collect::<Result<Vec<_>, ParseIntError>>()?;

        Ok(Disk { blocks })
    }
}

impl Disk {
    fn compact_by_block_policy(&self) -> Self {
        let mut compacted = self.blocks.clone();

        let mut start = 0;
        let mut end = compacted.len() - 1;
        while start < end {
            if compacted[start].free > 0 {
                let free_space = compacted[start].free;
                let move_amount = free_space.min(compacted[end].used);

                compacted[start].free = 0;
                let mut new_sector = Block {
                    id: compacted[end].id,
                    used: move_amount,
                    free: free_space - move_amount,
                };
                compacted[end].used -= move_amount;

                if compacted[end].used == 0 {
                    let new_free_space = move_amount + compacted[end].free;
                    if end - 1 == start {
                        new_sector.free += new_free_space;
                    } else {
                        compacted[end - 1].free += new_free_space;
                    }
                    compacted.remove(end);
                } else {
                    compacted[end].free += move_amount;
                }

                compacted.insert(start + 1, new_sector);
                end = compacted.len() - 1;
            }

            start += 1;
        }

        Self { blocks: compacted }
    }

    fn compact_by_file_policy(&self) -> Self {
        let mut optimized = self.blocks.clone();

        let file_ids: Vec<u64> = optimized
            .iter()
            .skip(1) // we never move first file
            .map(|sector| sector.id)
            .rev()
            .collect();

        for file_id in file_ids {
            let file_index = optimized
                .iter()
                .position(|sector| sector.id == file_id)
                .expect("file not found");

            let file_size = optimized[file_index].used;

            let fit_index = optimized
                .iter()
                .enumerate()
                .find(|(index, sector)| index < &file_index && sector.free >= file_size)
                .map(|(index, _)| index);

            if let Some(target_index) = fit_index {
                let free_space = optimized[target_index].free - file_size;
                optimized[target_index].free = 0;

                let mut new_block = Block {
                    id: file_id,
                    used: file_size,
                    free: free_space,
                };

                let sector_free_space = optimized[file_index].free;
                if file_index - 1 == target_index {
                    new_block.free += file_size + sector_free_space
                } else {
                    optimized[file_index - 1].free += file_size + sector_free_space;
                }

                optimized.remove(file_index);
                optimized.insert(target_index + 1, new_block);
            }
        }

        Self { blocks: optimized }
    }

    fn calculate_checksum(&self) -> u64 {
        let mut checksum = 0;
        let mut position = 0;
        for block in &self.blocks {
            checksum += (position..position + block.used)
                .map(|pos| pos * block.id)
                .sum::<u64>();
            position += block.used + block.free;
        }
        checksum
    }
}

fn main() -> io::Result<()> {
    let puzzle_input = include_str!("input.data");
    let disk = Disk::from_str(puzzle_input).expect("could not load disk map");

    let timer = Instant::now();
    let checksum = disk.compact_by_block_policy().calculate_checksum();
    println!(
        "Filesystem checksum after defragmentation by block policy is {}",
        checksum
    );
    println!("Time elapsed: {:?}", timer.elapsed());

    let timer = Instant::now();
    let checksum = disk.compact_by_file_policy().calculate_checksum();
    println!(
        "Filesystem checksum after defragmentation by file policy is {}",
        checksum
    );
    println!("Time elapsed: {:?}", timer.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::Disk;
    use std::str::FromStr;

    #[test]
    fn test_example_defragmentation_by_blocks() {
        let disk = Disk::from_str("2333133121414131402").expect("could not load disk map");
        assert_eq!(disk.compact_by_block_policy().calculate_checksum(), 1928);
    }

    #[test]
    fn test_simple_case_defragmentation_by_blocks() {
        let disk = Disk::from_str("12345").expect("could not load disk map");
        assert_eq!(disk.compact_by_block_policy().calculate_checksum(), 60);
    }

    #[test]
    fn test_example_defragmentation_by_files() {
        let disk = Disk::from_str("2333133121414131402").expect("could not load disk map");
        assert_eq!(disk.compact_by_file_policy().calculate_checksum(), 2858);
    }
}
