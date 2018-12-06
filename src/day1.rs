use std::collections::HashSet;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i64 {
    input
        .split_whitespace()
        .into_iter()
        .map(|c| str::parse::<i64>(c).unwrap())
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i64 {
    let mut answer: Option<i64> = None;
    let mut set = HashSet::<i64>::new();

    let mut current_frequency = 0;
    for current_input in input
        .split_whitespace()
        .into_iter()
        .map(str::parse::<i64>)
        .map(Result::unwrap)
        .cycle()
    {
        current_frequency += current_input;

        if set.contains(&current_frequency) {
            answer = Some(current_frequency);
            break;
        } else {
            set.insert(current_frequency);
        }
    }

    answer.unwrap()
}

#[cfg(test)]
mod part_one {
    use super::*;

    #[test]
    fn scenario_1() {
        let input = "+1
        +1
        +1";

        let expected = 3;

        assert_eq!(part1(input), expected);
    }
}

#[cfg(test)]
mod part_two {
    // Don't need no tests
}
