use std::collections::HashMap;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .filter_map(|l| {
            let mut val = l.trim().split_whitespace().map(|d| d.parse::<i32>());
            match (val.next(), val.next()) {
                (Some(Ok(a)), Some(Ok(b))) => Some((a, b)),
                _ => None,
            }
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[(i32, i32)]) -> i32 {
    let mut left_list: Vec<i32> = vec![];
    let mut right_list: Vec<i32> = vec![];

    input.iter().for_each(|&(l, r)| {
        left_list.push(l);
        right_list.push(r);
    });

    let mut left_sorted = left_list.clone();
    let mut right_sorted = right_list.clone();
    left_sorted.sort();
    right_sorted.sort();

    left_sorted
        .iter()
        .zip(right_sorted.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[(i32, i32)]) -> i32 {
    let mut left_list: Vec<i32> = vec![];
    let mut right_list: Vec<i32> = vec![];

    input.iter().for_each(|&(l, r)| {
        left_list.push(l);
        right_list.push(r);
    });

    // Hashmap to count numbers in right list
    let mut counts = HashMap::new();
    for &num in &right_list {
        *counts.entry(num).or_insert(0) += 1;
    }

    left_list
        .iter()
        .map(|&num| num * counts.get(&num).unwrap_or(&0))
        .sum()
}
