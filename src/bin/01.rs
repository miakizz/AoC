use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|x| (x[0], x[1]))
        .unzip();
    left.sort();
    right.sort();
    let sum: i32 = left
        .iter()
        .zip(right.iter())
        .map(|x| i32::abs(x.0 - x.1))
        .sum();
    return Some(sum as u32);
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|x| (x[0], x[1]))
        .unzip();
    let freq = right
        .into_iter()
        .fold(HashMap::<i32, i32>::new(), |mut m, x| {
            *m.entry(x).or_default() += 1;
            m
        });
    Some(
        left.into_iter()
            .map(|x| (*freq.get(&x).unwrap_or(&0) * x))
            .sum::<i32>() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
