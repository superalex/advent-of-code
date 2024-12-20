use regex::Regex;

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut ret = 0;
    for m in re.captures_iter(input) {
        let num1 = &m[1].parse::<i32>().unwrap();
        let num2 = &m[2].parse::<i32>().unwrap();
        ret += num1 * num2;
    }
    ret
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> i32 {
    // let re = Regex::new(r"don't\(\).*?do\(\)").unwrap();
    let re = Regex::new(r"don't\(\)(?s).*?do\(\)").unwrap();
    let clean_text = re.replace_all(input, "");
    solve_part1(&clean_text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2() {
        assert_eq!(
            solve_part2(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            ),
            48
        );
    }
}
