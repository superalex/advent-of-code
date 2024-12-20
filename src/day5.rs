use std::collections::HashSet;

#[aoc(day5, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let sections = input.split("\n\n").collect::<Vec<&str>>();
    let mut set: HashSet<(i32, i32)> = HashSet::new();

    for line in sections[0].lines() {
        let pages: Vec<i32> = line
            .split("|")
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        set.insert((pages[0], pages[1]));
    }

    let mut acum = 0;
    for update in sections[1].lines() {
        let pages: Vec<i32> = update
            .split(",")
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        let valid = pages.windows(2).all(|p| set.contains(&(p[0], p[1])));
        if valid {
            acum += pages[pages.len() / 2];
        }
    }

    acum
}

pub fn fix_pages(set: &HashSet<(i32, i32)>, pages: &Vec<i32>) -> i32 {
    let valid = pages.windows(2).all(|p| set.contains(&(p[0], p[1])));
    if valid {
        pages[pages.len() / 2]
    } else {
        let mut fixed = pages.clone();
        for (i, _n) in fixed.clone().iter().enumerate() {
            if i < fixed.len() - 1 && !set.contains(&(fixed[i], fixed[i + 1])) {
                fixed.swap(i, i + 1);
            }
        }
        fix_pages(set, &fixed)
    }
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let sections = input.split("\n\n").collect::<Vec<&str>>();
    let mut set: HashSet<(i32, i32)> = HashSet::new();

    for line in sections[0].lines() {
        let pages: Vec<i32> = line
            .split("|")
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        set.insert((pages[0], pages[1]));
    }

    let mut acum = 0;
    for update in sections[1].lines() {
        let pages: Vec<i32> = update
            .split(",")
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        let valid = pages.windows(2).all(|p| set.contains(&(p[0], p[1])));
        if !valid {
            let fixed = pages.clone();
            acum += fix_pages(&set, &fixed);
        }
    }

    acum
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&TEST_INPUT), 143);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&TEST_INPUT), 123);
    }
}
