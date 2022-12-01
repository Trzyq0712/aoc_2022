use super::Solution;

pub struct Day1;

impl Solution for Day1 {
    fn file_name(&self) -> &'static str {
        "inputs/day1.txt"
    }

    fn solve(&self, input: &str) {
        let input: Vec<Vec<u64>> = input
            .split("\n\n")
            .map(|elf_input| {
                elf_input
                    .lines()
                    .map(|n| n.parse().expect("Was not a valid number"))
                    .collect()
            })
            .collect();
        let problem1 = Self::solve_problem1(&input);
        println!("Problem 1: {}", problem1);
        let problem2 = Self::solve_problem2(&input);
        println!("Problem 2: {}", problem2);
    }
}

impl Day1 {
    fn solve_problem1(calories: &[Vec<u64>]) -> u64 {
        calories
            .iter()
            .map(|cal_list| cal_list.iter().sum::<u64>())
            .max()
            .unwrap_or(0)
    }
    fn solve_problem2(calories: &[Vec<u64>]) -> u64 {
        let mut elves: Vec<u64> = calories
            .iter()
            .map(|cal_list| cal_list.iter().sum())
            .collect();
        elves.sort_unstable();
        elves.iter().rev().take(3).sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample_input() -> Vec<Vec<u64>> {
        vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ]
    }

    #[test]
    fn problem1_example() {
        let ans = Day1::solve_problem1(&sample_input());
        assert_eq!(ans, 24000);
    }

    #[test]
    fn problem2_example() {
        let ans = Day1::solve_problem2(&sample_input());
        assert_eq!(ans, 45000);
    }
}
