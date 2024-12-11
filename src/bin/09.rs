use std::iter;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut disk_code = input
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    if disk_code.len() % 2 == 1 {
        disk_code.push(0);
    }
    let mut disk: Vec<i32> = disk_code
        .chunks(2)
        .enumerate()
        .map(|(id, file)| {
            iter::repeat(id as i32)
                .take(file[0].try_into().unwrap())
                .chain(iter::repeat(-1).take(file[1].try_into().unwrap()))
        })
        .flat_map(|f| f)
        //.inspect(|f| println!("{:?}", f))
        .collect();
    let mut left = 0;
    let mut right = disk.len() - 1;
    'computer: while left <= right {
        if disk[right] != -1 {
            while disk[left] != -1 {
                left += 1;
                if left >= right {
                    break 'computer;
                }
            }
            disk[left] = disk[right];
            disk[right] = -1;
        }
        right -= 1;
    }
    //println!("{:?}", disk.clone());
    Some(
        disk.iter()
            .enumerate()
            .filter(|x| *x.1 >= 0)
            .map(|(i, file)| *file as u64 * i as u64)
            .sum::<u64>(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut disk_code = input
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    if disk_code.len() % 2 == 1 {
        disk_code.push(0);
    }
    let mut disk: Vec<(i32, u32)> = disk_code
        .chunks(2)
        .enumerate()
        .map(|(id, file)| iter::once((id as i32, file[0])).chain(iter::once((-1, file[1]))))
        .flat_map(|f| f)
        //.inspect(|f| println!("{:?}", f))
        .collect();
    'computer: for right in (0..disk.len()).rev() {
        if disk[right].0 != -1 {
            for left in 0..right{
                if disk[left].0 == -1 && disk[left].1 >= disk[right].1{
                    let remainder = disk[left].1 - disk[right].1;
                    disk[left] = disk[right];
                    disk[right] = (-1, disk[left].1);
                    disk.insert(left + 1, (-1, remainder));
                    
                    continue 'computer;
                }
            }   
        }
    }
    let flat_fuck_friday = disk
        .iter()
        .map(|(id, len)| {
            let real_id = if *id == -1 { 0 } else { *id };
            iter::repeat(real_id).take(*len as usize)
        })
        .flat_map(|f| f);
    Some(
        flat_fuck_friday
            .enumerate()
            .map(|(i, file)| file as u64 * i as u64)
            .sum::<u64>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
