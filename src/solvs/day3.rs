use std::collections::HashSet;

use super::Solution;

pub struct Day3;

impl Solution for Day3 {
    fn file_name(&self) -> &'static str {
        "inputs/day3.txt"
    }

    fn solve(&self, input: &str) {
        let problem1 = Self::solve_problem1(input);
        println!("Problem 1: {}", problem1);
        let problem2 = Self::solve_problem2(input);
        println!("Problem 2: {}", problem2);
    }
}

fn get_priority(c: &char) -> u8 {
    match c {
        'a'..='z' => *c as u8 - b'a' + 1,
        'A'..='Z' => *c as u8 - b'A' + 27,
        _ => panic!("No mapping"),
    }
}

impl Day3 {
    fn solve_problem1(input: &str) -> u32 {
        let mut res = 0;
        for line in input.lines() {
            let (first, second) = line.split_at(line.len() / 2);
            let first: HashSet<_> = first.chars().collect();
            let second: HashSet<_> = second.chars().collect();
            let common = first.intersection(&second);
            common.map(get_priority).for_each(|p| res += p as u32);
        }
        res
    }

    fn solve_problem2(input: &str) -> u32 {
        let mut res = 0;
        let mut it = input.lines().peekable();
        while it.peek().is_some() {
            let (a, b, c) = (it.next().unwrap(), it.next().unwrap(), it.next().unwrap());
            let a: HashSet<_> = a.chars().collect();
            let b = b.chars().collect();
            let c = c.chars().collect();
            let common: HashSet<_> = a.intersection(&b).copied().collect();
            let common = common.intersection(&c);
            common.map(get_priority).for_each(|p| res += p as u32);
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn problem1() {
        let answer = Day3::solve_problem1(INPUT);
        assert_eq!(answer, 157);
    }

    #[test]
    fn problem2() {
        let answer = Day3::solve_problem2(INPUT);
        assert_eq!(answer, 70);
    }
}
