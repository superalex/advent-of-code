#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<i32>> {
    let mut matrix = Vec::new();
    for line in input.lines() {
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        matrix.push(row);
    }
    matrix
}

fn grows(vec: &Vec<i32>) -> bool {
    vec.windows(2)
        .all(|w| w[0] <= w[1] && w[1] - w[0] >= 1 && w[1] - w[0] <= 3)
}

fn decreases(vec: &Vec<i32>) -> bool {
    vec.windows(2)
        .all(|w| w[0] >= w[1] && w[0] - w[1] >= 1 && w[0] - w[1] <= 3)
}

fn safe_part1(vec: &Vec<i32>) -> bool {
    grows(vec) || decreases(vec)
}

fn safe_part2(vec: &Vec<i32>) -> bool {
    grows(vec)
        || decreases(vec)
        || (0..vec.len())
            .map(|i| {
                let mut new_vec = vec.clone();
                new_vec.remove(i);
                new_vec
            })
            .any(|v| grows(&v) || decreases(&v))
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<Vec<i32>>) -> i32 {
    input.iter().filter(|v| safe_part1(v)).count() as i32
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Vec<Vec<i32>>) -> i32 {
    input.iter().filter(|v| safe_part2(v)).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(safe_part1(&vec![7, 6, 4, 2, 1]), true);
        assert_eq!(safe_part1(&vec![1, 2, 7, 8, 9]), false);
        assert_eq!(safe_part1(&vec![9, 7, 6, 2, 1]), false);
        assert_eq!(safe_part1(&vec![1, 3, 2, 4, 5]), false);
        assert_eq!(safe_part1(&vec![8, 6, 4, 4, 1]), false);
        assert_eq!(safe_part1(&vec![1, 3, 6, 7, 9]), true);
    }

    #[test]
    fn part2() {
        assert_eq!(safe_part2(&vec![7, 6, 4, 2, 1]), true);
        assert_eq!(safe_part2(&vec![1, 2, 7, 8, 9]), false);
        assert_eq!(safe_part2(&vec![9, 7, 6, 2, 1]), false);
        assert_eq!(safe_part2(&vec![1, 3, 2, 4, 5]), true);
        assert_eq!(safe_part2(&vec![8, 6, 4, 4, 1]), true);
        assert_eq!(safe_part2(&vec![1, 3, 6, 7, 9]), true);
    }
}
