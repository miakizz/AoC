advent_of_code::solution!(6);

use array2d::{Array2D, Error};
use cartesian::*;

pub fn part_one(input: &str) -> Option<u32> {
    let map_vec: Vec<Vec<char>> = input
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    let mut map = Array2D::from_rows(&map_vec).unwrap();
    let dim_size = map.num_rows();
    let dim: i32 = dim_size.try_into().unwrap();
    let mut pos = (0, 0);
    let mut dir: (i32, i32) = (0, 0);
    for search in cartesian!(0..dim_size, 0..dim_size) {
        let cur_point = map[search];
        dir = match cur_point {
            '^' => (-1, 0),
            'v' => (1, 0),
            '<' => (0, -1),
            '>' => (0, 1),
            _ => (0, 0),
        };
        if dir != (0, 0) {
            pos = search;
            map[search] = 'X';
            break;
        }
    }

    loop {
        let future_space_check = (pos.0 as i32 + dir.0, pos.1 as i32 + dir.1);
        if future_space_check.0 == -1
            || future_space_check.1 == -1
            || future_space_check.0 == dim
            || future_space_check.1 == dim
        {
            break;
        }
        let future_space = (future_space_check.0 as usize, future_space_check.1 as usize);
        if map[future_space] == '#' {
            dir = match dir {
                (0, 1) => (1, 0),
                (0, -1) => (-1, 0),
                (-1, 0) => (0, 1),
                (1, 0) => (0, -1),
                _ => (0, 0),
            };
        } else {
            map[pos] = 'X';
            pos = future_space;
        }
    }
    Some(map.elements_row_major_iter().filter(|x| **x == 'X').count() as u32 + 1)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map_vec: Vec<Vec<char>> = input
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    let mut map = Array2D::from_rows(&map_vec).unwrap();
    let dim_size = map.num_rows();
    let dim: i32 = dim_size.try_into().unwrap();
    let mut loops:u32 = 0;
    let mut initial_pos = (0, 0);
    let mut initial_dir: (i32, i32) = (0, 0);
    for search in cartesian!(0..dim_size, 0..dim_size) {
        let cur_point = map[search];
        initial_dir = match cur_point {
            '^' => (-1, 0),
            'v' => (1, 0),
            '<' => (0, -1),
            '>' => (0, 1),
            _ => (0, 0),
        };
        if initial_dir != (0, 0) {
            initial_pos = search;
            map[search] = 'X';
            break;
        }
    }
    for test in cartesian!(0..dim_size, 0..dim_size) {
        if map[test] == '.' || map[test] == 'X' && test != initial_pos {
            map[test] = '#';
            let mut pos = initial_pos;
            let mut dir: (i32, i32) = initial_dir;
            let mut loop_counter = 0;
            loop {
                loop_counter += 1;
                if loop_counter > 10000{loops += 1; break;}
                let future_space_check = (pos.0 as i32 + dir.0, pos.1 as i32 + dir.1);
                if future_space_check.0 == -1
                    || future_space_check.1 == -1
                    || future_space_check.0 == dim
                    || future_space_check.1 == dim
                {
                    break;
                }
                let future_space = (future_space_check.0 as usize, future_space_check.1 as usize);
                if map[future_space] == '#' {
                    dir = match dir {
                        (0, 1) => (1, 0),
                        (0, -1) => (-1, 0),
                        (-1, 0) => (0, 1),
                        (1, 0) => (0, -1),
                        _ => (0, 0),
                    };
                } else {
                    map[pos] = 'X';
                    pos = future_space;
                }
            }
            map[test] = '.';
        }
    }
    Some(loops)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
