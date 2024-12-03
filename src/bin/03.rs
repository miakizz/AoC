advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let inst_re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let num_re = Regex::new(r"\d+").unwrap();
    Some(
        inst_re
            .find_iter(input)
            .map(|inst| {
                num_re
                    .find_iter(inst.as_str())
                    .map(|factor| factor.as_str().parse::<u32>().unwrap())
                    .product::<u32>()
            })
            .sum::<u32>()
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let inst_re = Regex::new(r"(mul\(\d+,\d+\)|do\(\)|don't\(\))").unwrap();
    let num_re = Regex::new(r"\d+").unwrap();
    let instructions = inst_re.find_iter(input).map(|x| x.as_str());
    let mut state = true;
    let mut sum = 0;
    for inst in instructions {
        if inst == "don't()" {
            state = false;
        } else if inst == "do()" {
            state = true;
        } else {
            if state {
                sum += num_re
                    .find_iter(inst)
                    .map(|y| y.as_str().parse::<u32>().unwrap())
                    .product::<u32>();
            }
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
