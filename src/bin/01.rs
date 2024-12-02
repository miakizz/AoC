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
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
