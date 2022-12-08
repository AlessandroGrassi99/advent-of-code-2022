use std::collections::HashSet;

fn check_window(win: &[char], check_size: usize) -> bool {
    let tmp: HashSet<&char> = HashSet::from_iter(win);
    tmp.len() == check_size
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = input.chars().collect::<Vec<char>>();
    let mut index = 4;
    while index < input.len() {
        let win = &input[index - 4..index];

        if check_window(win, 4) {
            return Some(index as u32);
        }

        index += 1;
    }

    unreachable!()
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = input.chars().collect::<Vec<char>>();
    let mut index = 14;
    while index < input.len() {
        let win = &input[index - 14..index];

        if check_window(win, 14) {
            return Some(index as u32);
        }

        index += 1;
    }

    unreachable!()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
