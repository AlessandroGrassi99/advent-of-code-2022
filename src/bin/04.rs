use std::ops::RangeInclusive;

fn range_from_str(value: &str) -> RangeInclusive<u32> {
    let (sx, dx) = value.split_once('-').unwrap();
    sx.parse::<u32>().unwrap()..=dx.parse::<u32>().unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let (sx, dx) = line.split_once(',').unwrap();
                (range_from_str(sx), range_from_str(dx))
            })
            .filter(|(a, b)| {
                (a.contains(b.start()) && a.contains(b.end()))
                    || (b.contains(a.start()) && b.contains(a.end()))
            })
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let (sx, dx) = line.split_once(',').unwrap();
                (range_from_str(sx), range_from_str(dx))
            })
            .filter(|(a, b)| {
                (a.contains(b.start()) || a.contains(b.end()))
                    || (b.contains(a.start()) || b.contains(a.end()))
            })
            .count() as u32,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
