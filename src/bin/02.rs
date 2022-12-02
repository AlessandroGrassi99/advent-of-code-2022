#[derive(Copy, Clone)]
pub enum GameOutcome {
    Lost,
    Draw,
    Win,
}

impl From<char> for GameOutcome {
    fn from(value: char) -> Self {
        match value {
            'X' => Self::Lost,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Shape {
    Rock,
    Paper,
    Scissor,
}

impl From<char> for Shape {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissor,
            'X' => Self::Rock,
            'Y' => Self::Paper,
            'Z' => Self::Scissor,
            _ => unreachable!(),
        }
    }
}

impl GameOutcome {
    pub fn process_game1(a: char, b: char) -> u32 {
        let a = Shape::from(a);
        let b = Shape::from(b);
        let outcome = Self::game_logic(a, b);

        Self::outcome_points(outcome) + Self::shape_points(b)
    }

    pub fn process_game2(a: char, b: char) -> u32 {
        let a = Shape::from(a);
        let b = GameOutcome::from(b);
        let shape_to_choose = Self::game_logic_prediction(a, b);

        Self::outcome_points(b) + Self::shape_points(shape_to_choose)
    }

    fn game_logic(a: Shape, b: Shape) -> Self {
        use Shape::*;
        if a == b {
            Self::Draw
        } else if (b == Rock && a == Scissor)
            || (b == Paper && a == Rock)
            || (b == Scissor && a == Paper)
        {
            Self::Win
        } else {
            Self::Lost
        }
    }

    fn game_logic_prediction(a: Shape, b: GameOutcome) -> Shape {
        use GameOutcome::*;
        use Shape::*;

        match (b, a) {
            (Lost, Rock) => Scissor,
            (Lost, Paper) => Rock,
            (Lost, Scissor) => Paper,
            (Draw, _) => a,
            (Win, Rock) => Paper,
            (Win, Paper) => Scissor,
            (Win, Scissor) => Rock,
        }
    }

    fn outcome_points(outcome: Self) -> u32 {
        match outcome {
            GameOutcome::Lost => 0,
            GameOutcome::Draw => 3,
            GameOutcome::Win => 6,
        }
    }

    fn shape_points(shape: Shape) -> u32 {
        match shape {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissor => 3,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut line = line.chars();
                GameOutcome::process_game1(line.next().unwrap(), line.nth(1).unwrap())
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut line = line.chars();
                GameOutcome::process_game2(line.next().unwrap(), line.nth(1).unwrap())
            })
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
