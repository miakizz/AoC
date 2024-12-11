use itertools::Itertools;

advent_of_code::solution!(7);

pub trait CollectToVec: core::iter::Iterator {
    fn collect_vec(self) -> Vec<Self::Item> where Self: Sized {
        self.collect::<Vec<_>>()
    }
}

impl<I: Iterator> CollectToVec for I {}



pub fn part_one(input: &str) -> Option<u64> {
    let initial_split: Vec<_> = input
        .lines()
        .map(|x| x.split(':').collect_vec())
        .collect();
    let products = initial_split.iter().map(|x| x[0].parse::<u64>().unwrap());
    let factor_lists = initial_split.iter().map(|x| {
        x[1].split_whitespace()
            .map(|y| y.parse::<u64>().unwrap())
            .collect::<Vec<_>>()
    });
    Some(
        products
            .zip(factor_lists)
            .map(|(product, factors)| {
                let operators_list = std::iter::repeat(0..2)
                    .take(factors.len() - 1)
                    .multi_cartesian_product();

                let success = operators_list
                    .map(|operators| {
                        let mut factors_iter = factors.iter();
                        let mut accum: u64 = *factors_iter.next().unwrap();
                        operators.iter().for_each(|operator| {
                            accum = do_math(accum, *factors_iter.next().unwrap(), *operator)
                        });
                        accum
                    })
                    .any(|accum| accum == product);
                if success {
                    product
                } else {
                    0
                }
            })
            .sum(),
    )
}

fn do_math(left: u64, right: u64, operator: i32) -> u64 {
    match operator {
        0 => left + right,
        1 => left * right,
        2 => left * 10_u64.pow((right as f64).log10().floor() as u32 + 1) + right,
        _ => 0,
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    dbg!(do_math(10, 10, 2));
    let initial_split: Vec<_> = input
        .lines()
        .map(|x| x.split(':').collect::<Vec<_>>())
        .collect();
    let products = initial_split.iter().map(|x| x[0].parse::<u64>().unwrap());
    let factor_lists = initial_split.iter().map(|x| {
        x[1].split_whitespace()
            .map(|y| y.parse::<u64>().unwrap())
            .collect::<Vec<_>>()
    });
    Some(
        products
            .zip(factor_lists)
            .map(|(product, factors)| {
                let operators_list = std::iter::repeat(0..3)
                    .take(factors.len() - 1)
                    .multi_cartesian_product();

                let success = operators_list
                    .map(|operators| {
                        let mut factors_iter = factors.iter();
                        let mut accum: u64 = *factors_iter.next().unwrap();
                        operators.iter().for_each(|operator| {
                            accum = do_math(accum, *factors_iter.next().unwrap(), *operator)
                        });
                        accum
                    })
                    .any(|accum| accum == product);
                if success {
                    product
                } else {
                    0
                }
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
