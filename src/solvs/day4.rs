use super::Solution;
use std::ops::{RangeBounds, RangeInclusive};

pub struct Day4;

impl Solution for Day4 {
    fn file_name(&self) -> &'static str {
        "inputs/day4.txt"
    }

    fn solve(&self, input: &str) {
        let problem1 = Self::solve_problem1(input);
        println!("Problem 1: {}", problem1);
        let problem2 = Self::solve_problem2(input);
        println!("Problem 2: {}", problem2);
    }
}

fn get_range(s: &str) -> (u32, u32) {
    let mut it = s.split('-').map(|v| v.parse::<u32>().unwrap());
    (it.next().unwrap(), it.next().unwrap())
}

impl Day4 {
    fn solve_problem1(input: &str) -> u32 {
        let mut res = 0;
        for l in input.lines() {
            let mut l = l.split(',');
            let (a, b) = get_range(l.next().unwrap());
            let (c, d) = get_range(l.next().unwrap());
            let r1 = a..=b;
            let r2 = c..=d;
            if (r1.contains(&c) && r1.contains(&d)) || (r2.contains(&a) && r2.contains(&b)) {
                res += 1;
            }
        }
        res
    }

    fn solve_problem2(input: &str) -> u32 {
        let mut res = 0;
        for l in input.lines() {
            let mut l = l.split(',');
            let (a, b) = get_range(l.next().unwrap());
            let (c, d) = get_range(l.next().unwrap());
            let r1 = a..=b;
            let r2 = c..=d;
            if r1.contains(&c) || r1.contains(&d) || r2.contains(&a) || r2.contains(&b) {
                res += 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn problem1() {
        let answer = Day4::solve_problem1(INPUT);
        assert_eq!(answer, 2);
    }

    #[test]
    fn problem1_2() {
        let answer = Day4::solve_problem1("22-65,22-66");
        assert_eq!(answer, 1);
    }

    #[test]
    fn problem2() {
        let answer = Day4::solve_problem2(INPUT);
        assert_eq!(answer, 4);
    }
}
