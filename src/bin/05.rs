use std::collections::VecDeque;

pub fn part_one(input: &str) -> Option<String> {
    Some({
        let (crates, ops) = input.split_once("\n\n").unwrap();
        let mut stacks = vec![VecDeque::new(); 9]; //= [VecDeque::new(); 9];

        let mut stacks_lines = crates.lines().peekable();
        while let Some(line) = stacks_lines.next() {
            if stacks_lines.peek().is_none() {
                break;
            }

            line.chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .filter(|(_, c)| !c.is_whitespace())
                .for_each(|(i, c)| {
                    stacks[i].push_front(c);
                });
        }

        ops.lines().for_each(|line| {
            let mut line_component = line.split(' ');
            let quantity = line_component.nth(1).unwrap().parse::<u32>().unwrap();
            let from = line_component.nth(1).unwrap().parse::<usize>().unwrap();
            let to = line_component.nth(1).unwrap().parse::<usize>().unwrap();

            for _ in 0..quantity {
                let tmp = stacks[from - 1].pop_back().unwrap();
                stacks[to - 1].push_back(tmp);
            }
        });

        let mut res = String::default();
        for stack in stacks {
            match stack.back() {
                None => (),
                Some(c) => res.push(*c),
            }
        }

        res
    })
}

pub fn part_two(input: &str) -> Option<String> {
    Some({
        let (crates, ops) = input.split_once("\n\n").unwrap();
        let mut stacks = vec![VecDeque::new(); 9]; //= [VecDeque::new(); 9];

        let mut stacks_lines = crates.lines().peekable();
        while let Some(line) = stacks_lines.next() {
            if stacks_lines.peek().is_none() {
                break;
            }

            line.chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .filter(|(_, c)| !c.is_whitespace())
                .for_each(|(i, c)| {
                    stacks[i].push_front(c);
                });
        }

        ops.lines().for_each(|line| {
            let mut line_component = line.split(' ');
            let quantity = line_component.nth(1).unwrap().parse::<u32>().unwrap();
            let from = line_component.nth(1).unwrap().parse::<usize>().unwrap();
            let to = line_component.nth(1).unwrap().parse::<usize>().unwrap();

            let mut crates_to_move = VecDeque::new();
            for _ in 0..quantity {
                crates_to_move.push_front(stacks[from - 1].pop_back().unwrap());
            }

            for x in crates_to_move {
                stacks[to - 1].push_back(x);
            }
        });

        let mut res = String::default();
        for stack in stacks {
            match stack.back() {
                None => (),
                Some(c) => res.push(*c),
            }
        }

        res
    })
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
