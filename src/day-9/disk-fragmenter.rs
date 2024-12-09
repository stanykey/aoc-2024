use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Clone)]
struct Sector {
    file_id: usize,
    file_size: u64,
    free_space: u64,
}

fn load_disc_map(file_path: &Path) -> io::Result<String> {
    let result = fs::read_to_string(file_path);
    match result {
        Ok(text) => Ok(text.trim().to_string()),
        Err(e) => Err(e),
    }
}

fn parse_disk_map(disk_map: &str) -> Vec<Sector> {
    let mut sectors = Vec::new();
    let mut is_file = true;
    let mut file_id = 0;

    let mut chars = disk_map.chars();
    while let Some(c) = chars.next() {
        let size = c.to_digit(10).unwrap() as u64;

        if is_file {
            sectors.push(Sector {
                file_id,
                file_size: size,
                free_space: 0,
            });
            file_id += 1;
        } else {
            // Add free space to the previous sector
            if let Some(last_sector) = sectors.last_mut() {
                last_sector.free_space += size;
            }
        }

        is_file = !is_file;
    }

    sectors
}

fn compact_disk(sectors: &Vec<Sector>) -> Vec<Sector> {
    let mut compacted = sectors.clone();

    let mut start = 0;
    let mut end = compacted.len() - 1;
    while start < end {
        if compacted[start].free_space > 0 {
            let free_space = compacted[start].free_space;
            let move_amount = free_space.min(compacted[end].file_size);

            compacted[start].free_space = 0;
            let mut new_sector = Sector {
                file_id: compacted[end].file_id,
                file_size: move_amount,
                free_space: free_space - move_amount,
            };
            compacted[end].file_size -= move_amount;

            if compacted[end].file_size == 0 {
                let new_free_space = move_amount + compacted[end].free_space;
                if end - 1 == start {
                    new_sector.free_space += new_free_space;
                } else {
                    compacted[end - 1].free_space += new_free_space;
                }
                compacted.remove(end);
            } else {
                compacted[end].free_space += move_amount;
            }

            compacted.insert(start + 1, new_sector);
            end = compacted.len() - 1;
        }

        start += 1;
    }

    compacted
}

fn calculate_checksum(sectors: &Vec<Sector>) -> u64 {
    let mut checksum = 0;
    let mut position = 0;

    for sector in sectors {
        if sector.file_size > 0 {
            for block_offset in 0..sector.file_size {
                checksum += (position + block_offset) as u64 * sector.file_id as u64;
            }
        }

        position += sector.file_size + sector.free_space;
    }

    checksum
}

fn find_filesystem_checksum(sectors: &Vec<Sector>) -> u64 {
    let defragmented = compact_disk(sectors);
    calculate_checksum(&defragmented)
}

fn main() -> io::Result<()> {
    let file_path = Path::new("input.data");
    let disk_map = load_disc_map(file_path)?;

    let sectors = parse_disk_map(&disk_map);
    let checksum = find_filesystem_checksum(&sectors);
    println!("Filesystem checksum: {}", checksum);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{calculate_checksum, compact_disk, parse_disk_map};

    fn compact_and_calculate_checksum(disk_map: &str) -> u64 {
        let blocks = parse_disk_map(disk_map);
        let defragmented = compact_disk(&blocks);
        calculate_checksum(&defragmented)
    }

    #[test]
    fn test_example() {
        let disk_map = "2333133121414131402";
        assert_eq!(compact_and_calculate_checksum(disk_map), 1928);
    }

    #[test]
    fn test_simple_case() {
        let disk_map = "12345";
        assert_eq!(compact_and_calculate_checksum(disk_map), 60);
    }
}
