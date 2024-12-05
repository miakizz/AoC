use diagonal::{diagonal_pos_neg, diagonal_pos_pos, straight_x, straight_y};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let original: Vec<Vec<char>> = input
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    let mut count = 0;
    count += find_matches(straight_x(&original));
    count += find_matches(straight_y(&original));
    count += find_matches(diagonal_pos_pos(&original));
    count += find_matches(diagonal_pos_neg(&original));
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let original: Vec<Vec<char>> = input
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    let mut count = 0;
    for y in 1..original.len() - 1 {
        for x in 1..original[0].len() - 1 {
            if original[y][x] == 'A'
                && ((original[y - 1][x - 1] == 'M' && original[y + 1][x + 1] == 'S')
                    || (original[y - 1][x - 1] == 'S' && original[y + 1][x + 1] == 'M'))
                && ((original[y - 1][x + 1] == 'M' && original[y + 1][x - 1] == 'S')
                    || (original[y - 1][x + 1] == 'S' && original[y + 1][x - 1] == 'M'))
            {
                count += 1;
            }
        }
    }
    Some(count)
}

fn find_matches(input: Vec<Vec<&char>>) -> u32 {
    let forward = vec![&'X', &'M', &'A', &'S'];
    let mut rev = forward.clone();
    rev.reverse();
    input
        .into_iter()
        .map(|x| x.windows(4).filter(|x| *x == forward || *x == rev).count())
        .sum::<usize>() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
