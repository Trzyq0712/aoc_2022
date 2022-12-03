use std::convert::TryFrom;
use std::str::FromStr;

use super::Solution;

pub struct Day2;

impl Solution for Day2 {
    fn file_name(&self) -> &'static str {
        "inputs/day2.txt"
    }

    fn solve(&self, input: &str) {
        let rounds: Vec<_> = input
            .lines()
            .map(|r| {
                let mut hands = r.split_ascii_whitespace();
                (hands.next().unwrap(), hands.next().unwrap())
            })
            .collect();

        let problem1 = Self::solve_problem1(&rounds);
        println!("Problem 1: {}", problem1);
        let problem2 = Self::solve_problem2(&rounds);
        println!("Problem 2: {}", problem2);
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Hand {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Hand::Rock),
            "B" => Ok(Hand::Paper),
            "C" => Ok(Hand::Scissors),
            _ => Err("Could not parse the hand"),
        }
    }
}

impl TryFrom<usize> for Hand {
    type Error = &'static str;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Hand::Rock),
            1 => Ok(Hand::Paper),
            2 => Ok(Hand::Scissors),
            _ => Err("Such variant does not exist"),
        }
    }
}

impl Hand {
    fn play_hand(self, other: Self) -> u64 {
        let dif = (self as i32 - other as i32 + 1).rem_euclid(3);
        dif as u64 * 3
    }

    fn get_value(self) -> u64 {
        self as u64 + 1
    }

    fn get_corresponding_hand(self, expected_res: &str) -> Hand {
        let change = match expected_res {
            "X" => -1,
            "Y" => 0,
            "Z" => 1,
            _ => panic!("Invalid character"),
        };
        Hand::try_from((self as i32 + change).rem_euclid(3) as usize).unwrap()
    }
}

impl Day2 {
    fn solve_problem1(rounds: &[(&str, &str)]) -> u64 {
        let mut score = 0;
        for (their, mine) in rounds {
            let their = Hand::from_str(their).unwrap();
            let mine = match *mine {
                "X" => "A",
                "Y" => "B",
                "Z" => "C",
                _ => panic!("Invalid character"),
            };
            let mine = Hand::from_str(mine).unwrap();
            score += mine.play_hand(their);
            score += mine.get_value();
        }
        score
    }
    fn solve_problem2(rounds: &[(&str, &str)]) -> u64 {
        let mut score = 0;
        for (their, mine) in rounds {
            let their = Hand::from_str(their).unwrap();
            let mine = their.get_corresponding_hand(mine);
            score += mine.play_hand(their);
            score += mine.get_value();
        }
        score
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn play_hands() {
        assert_eq!(Hand::play_hand(Hand::Paper, Hand::Rock), 6);
        assert_eq!(Hand::play_hand(Hand::Scissors, Hand::Rock), 0);
        assert_eq!(Hand::play_hand(Hand::Rock, Hand::Rock), 3);
    }

    #[test]
    fn get_corresponding_hands() {
        assert_eq!(Hand::Paper.get_corresponding_hand("Z"), Hand::Scissors);
        assert_eq!(Hand::Paper.get_corresponding_hand("X"), Hand::Rock);
        assert_eq!(Hand::Scissors.get_corresponding_hand("X"), Hand::Paper);
    }

    fn sample_input() -> &'static [(&'static str, &'static str)] {
        &[("A", "Y"), ("B", "X"), ("C", "Z")]
    }

    #[test]
    fn problem1_example() {
        let answer = Day2::solve_problem1(sample_input());
        assert_eq!(answer, 15);
    }

    #[test]
    fn problem2_example() {
        let answer = Day2::solve_problem2(sample_input());
        assert_eq!(answer, 12);
    }
}
