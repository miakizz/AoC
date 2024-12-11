use array2d::Array2D;
use cartesian::*;
use std::collections::HashSet;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let side_len = (input.len() as f64).sqrt() as usize;
    let map_iter = input.chars().filter_map(|x| x.to_digit(10));
    let map = Array2D::from_iter_row_major(map_iter, side_len, side_len).unwrap();
    Some(
        cartesian!(0..side_len, 0..side_len)
            .map(|pos| {
                let mut dest: HashSet<(usize, usize)> = HashSet::new();
                visit_one(&map, &mut dest, -1, pos);
                dest.len()
            })
            .sum::<usize>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let side_len = (input.len() as f64).sqrt() as usize;
    let map_iter = input.chars().filter_map(|x| x.to_digit(10));
    let map = Array2D::from_iter_row_major(map_iter, side_len, side_len).unwrap();
    Some(
        cartesian!(0..side_len, 0..side_len)
            .map(|pos| {
                let mut dest: HashSet<Vec<(usize, usize)>> = HashSet::new();
                visit_two(&map, &mut dest, vec![], -1, pos);
                dest.len()
            })
            .sum::<usize>() as u32,
    )
}

fn visit_one(
    map: &Array2D<u32>,
    dests: &mut HashSet<(usize, usize)>,
    last_val: i32,
    to: (usize, usize),
) {
    let cur_val: i32 = map[to].try_into().unwrap();
    if last_val != cur_val - 1 {
        return;
    } else if cur_val == 9 {
        dests.insert(to);
    } else {
        if to.0 > 0 {
            visit_one(map, dests, cur_val, (to.0 - 1, to.1));
        }
        if to.0 < map.row_len() - 1 {
            visit_one(map, dests, cur_val, (to.0 + 1, to.1));
        }
        if to.1 > 0 {
            visit_one(map, dests, cur_val, (to.0, to.1 - 1));
        }
        if to.1 < map.column_len() - 1 {
            visit_one(map, dests, cur_val, (to.0, to.1 + 1));
        }
    }
}

fn visit_two(
    map: &Array2D<u32>,
    dests: &mut HashSet<Vec<(usize, usize)>>,
    mut path: Vec<(usize, usize)>,
    last_val: i32,
    to: (usize, usize),
) {
    let cur_val: i32 = map[to].try_into().unwrap();
    if last_val != cur_val - 1 {
        return;
    } else if cur_val == 9 {
        path.push(to);
        dests.insert(path);
    } else {
        path.push(to);
        if to.0 > 0 {
            visit_two(map, dests, path.clone(), cur_val, (to.0 - 1, to.1));
        }
        if to.0 < map.row_len() - 1 {
            visit_two(map, dests, path.clone(), cur_val, (to.0 + 1, to.1));
        }
        if to.1 > 0 {
            visit_two(map, dests, path.clone(), cur_val, (to.0, to.1 - 1));
        }
        if to.1 < map.column_len() - 1 {
            visit_two(map, dests, path.clone(), cur_val, (to.0, to.1 + 1));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
