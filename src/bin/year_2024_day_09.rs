fn main() {
    use std::io::Read;

    let mut input_content = std::fs::File::open("input.txt").expect("No input.txt file");
    let mut buffer = String::new();

    input_content.read_to_string(&mut buffer).unwrap();
    let input: Input = buffer.as_str().parse().unwrap();

    let ans1 = part_one(&input);
    let ans2 = part_two(&input);

    if let Ok(ans1) = ans1 {
        println!("Result of part 1: {ans1}")
    } else {
        println!("Part 1 fails.")
    }

    if let Ok(ans2) = ans2 {
        println!("Result of part 2: {ans2}")
    } else {
        println!("Part 2 fails.")
    }
}

struct Input {
    disk_map: Vec<u8>,
}

impl std::str::FromStr for Input {
    type Err = String;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let content = content.trim();
        if !content.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err("disk map contains a non-digit character".to_string());
        }

        Ok(Self {
            disk_map: content.bytes().map(|byte| byte - b'0').collect(),
        })
    }
}

fn expand(disk_map: &[u8]) -> Vec<Option<u64>> {
    disk_map
        .iter()
        .copied()
        .enumerate()
        .flat_map(|(index, length)| {
            let entry = (index % 2 == 0).then_some((index / 2) as u64);
            std::iter::repeat_n(entry, usize::from(length))
        })
        .collect()
}

fn checksum(disk: &[Option<u64>]) -> u128 {
    disk.iter()
        .enumerate()
        .filter_map(|(index, file_id)| file_id.map(|file_id| index as u128 * u128::from(file_id)))
        .sum()
}

#[forbid(unsafe_code)]
fn part_one(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    let mut disk = expand(&input.disk_map);

    if !disk.is_empty() {
        let mut left = 0;
        let mut right = disk.len() - 1;

        while left < right {
            while left < disk.len() && disk[left].is_some() {
                left += 1;
            }
            while right > 0 && disk[right].is_none() {
                right -= 1;
            }
            if left >= right {
                break;
            }

            disk[left] = disk[right].take();
        }
    }

    Ok(checksum(&disk))
}

#[forbid(unsafe_code)]
fn part_two(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    let mut disk = expand(&input.disk_map);
    let file_count = input.disk_map.len().div_ceil(2);

    for file_id in (0..file_count).rev().map(|file_id| file_id as u64) {
        let Some(start) = disk.iter().position(|&entry| entry == Some(file_id)) else {
            continue;
        };
        let length = disk[start..].iter().take_while(|&&entry| entry == Some(file_id)).count();
        let Some(destination) =
            disk[..start].windows(length).position(|window| window.iter().all(Option::is_none))
        else {
            continue;
        };

        for offset in 0..length {
            disk[destination + offset] = Some(file_id);
            disk[start + offset] = None;
        }
    }

    Ok(checksum(&disk))
}

#[cfg(test)]
mod test {
    use parameterized::parameterized;

    use super::*;

    #[parameterized(input = { "2333133121414131402" }, expected = { "1928" })]
    fn test_part_1(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_one(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }

    #[parameterized(input = { "2333133121414131402" }, expected = { "2858" })]
    fn test_part_2(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_two(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }
}
