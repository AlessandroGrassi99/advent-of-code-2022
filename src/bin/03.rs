use std::collections::HashSet;

struct Rucksack {
    left: String,
    right: String,
}

struct RucksackGroup {
    first: Rucksack,
    second: Rucksack,
    third: Rucksack,
}

impl Rucksack {
    pub(crate) fn new(input: &str) -> Self {
        Self {
            left: input[..input.len() / 2].to_string(),
            right: input[input.len() / 2..].to_string(),
        }
    }

    fn find_duplicates(&self) -> HashSet<char> {
        let left: HashSet<char> = self.left.clone().chars().collect();
        let right: HashSet<char> = self.right.clone().chars().collect();

        left.intersection(&right).cloned().collect()
    }

    pub(crate) fn sum_priorities_duplicates(&self) -> u32 {
        let duplicates = self.find_duplicates();
        let mut sum: u32 = 0;
        for c in duplicates {
            sum += Rucksack::get_char_priority(c)
        }

        sum
    }

    fn get_char_priority(c: char) -> u32 {
        if c.is_ascii_lowercase() {
            Into::<u32>::into(c) - 96u32
        } else {
            Into::<u32>::into(c) - 65u32 + 27u32
        }
    }

    pub(crate) fn all_unique_chars(&self) -> HashSet<char> {
        let left: HashSet<char> = self.left.clone().chars().collect();
        let right: HashSet<char> = self.right.clone().chars().collect();

        left.union(&right).cloned().collect()
    }
}

impl RucksackGroup {
    pub(crate) fn new(first: Rucksack, second: Rucksack, third: Rucksack) -> Self {
        Self {
            first,
            second,
            third,
        }
    }

    fn find_common_char(&self) -> char {
        let first = self.first.all_unique_chars();
        let second = self.second.all_unique_chars();
        let third = self.third.all_unique_chars();

        first
            .intersection(&second)
            .cloned()
            .collect::<HashSet<char>>()
            .intersection(&third)
            .cloned()
            .collect::<Vec<char>>()[0]
    }

    pub(crate) fn get_group_priority(&self) -> u32 {
        let c = self.find_common_char();

        Rucksack::get_char_priority(c)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let rucksack = Rucksack::new(line);
                rucksack.sum_priorities_duplicates()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = input.lines().collect::<Vec<_>>();

    Some(
        input
            .chunks(3)
            .map(|lines| {
                let first = Rucksack::new(lines[0]);
                let second = Rucksack::new(lines[1]);
                let third = Rucksack::new(lines[2]);

                let rucksack_group = RucksackGroup::new(first, second, third);
                rucksack_group.get_group_priority()
            })
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
