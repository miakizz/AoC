advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut split = input.split("\n\n");
    let rulesStr = split.next().unwrap();
    let pagesStr = split.next().unwrap();
    let rules: Vec<Vec<u32>> = rulesStr
        .lines()
        .map(|x| {
            x.split("|")
                .map(|y| y.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    let pages: Vec<Vec<u32>> = pagesStr
        .lines()
        .map(|x| {
            x.split(",")
                .map(|y| y.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    Some(pages
        .into_iter()
        .filter(|book| {
            rules.iter().all(|rule| {
                let mut seen_before = false;
                let mut seen_after = false;
                for page in book.into_iter() {
                    if *page == rule[0] {
                        seen_before = true;
                    } else if *page == rule[1] {
                        if seen_before {
                            return true;
                        } else {
                            seen_after = true;
                        }
                    }
                }
                return !(seen_before && seen_after)
            })
        })
        .map(|x| {
            let middle = x.len() / 2;
            x[middle]
        })
        .sum::<u32>())
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
