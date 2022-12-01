use std::cmp::max;
use std::collections::BinaryHeap;

pub fn part_one(input: &str) -> Option<u32> {
    let mut max_kcals: Option<u32> = None;
    let mut curr_kcals: u32 = 0;

    for line in input.lines() {
        if line.is_empty() {
            curr_kcals = 0;
            continue;
        }

        let kcal = line.parse::<u32>().unwrap();
        curr_kcals += kcal;
        max_kcals = Some(max(curr_kcals, max_kcals.unwrap_or(curr_kcals)));
    }

    max_kcals
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .split("\n\n")
            .map(|elf_kcals| elf_kcals.lines().flat_map(str::parse::<u32>).sum())
            .collect::<BinaryHeap<u32>>()
            .into_iter()
            .take(3)
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
