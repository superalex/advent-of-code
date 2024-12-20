#[derive(Clone, Debug)]
pub struct Equation {
    result: i64,
    numbers: Vec<i64>,
}

impl Equation {
    fn valid_part1(&self) -> bool {
        if self.numbers.len() >= 2 {
            let mut numbers = self.numbers.clone();
            let first_two: Vec<_> = numbers.drain(0..2).collect();
            // println!("Primeros elementos movidos: {} y {}", first_two[0], first_two[1]);
            let sum = first_two[0] + first_two[1];
            numbers.insert(0, sum);
            let eq_sum = Equation {
                result: self.result,
                numbers: numbers.clone(),
            };

            let mul = first_two[0] * first_two[1];
            numbers[0] = mul;
            let eq_mul = Equation {
                result: self.result,
                numbers,
            };
            eq_sum.valid_part1() || eq_mul.valid_part1()
        } else {
            self.result == self.numbers[0]
        }
    }

    fn valid_part2(&self) -> bool {
        if self.numbers.len() >= 2 {
            let mut numbers = self.numbers.clone();
            let first_two: Vec<_> = numbers.drain(0..2).collect();
            // println!("Primeros elementos movidos: {} y {}", first_two[0], first_two[1]);
            let sum = first_two[0] + first_two[1];
            numbers.insert(0, sum);
            let eq_sum = Equation {
                result: self.result,
                numbers: numbers.clone(),
            };

            let mul = first_two[0] * first_two[1];
            let mut numbers_mul = numbers.clone();
            numbers_mul[0] = mul;
            let eq_mul = Equation {
                result: self.result,
                numbers: numbers_mul,
            };

            let concat = (first_two[0].to_string() + &first_two[1].to_string())
                .parse::<i64>()
                .unwrap();
            let mut numbers_concat = numbers.clone();
            numbers_concat[0] = concat;
            let eq_concat = Equation {
                result: self.result,
                numbers: numbers_concat,
            };

            eq_sum.valid_part2() || eq_mul.valid_part2() || eq_concat.valid_part2()
        } else {
            self.result == self.numbers[0]
        }
    }
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<Equation> {
    let mut ret = Vec::new();
    for line in input.lines() {
        let parts = line.split(": ").collect::<Vec<&str>>();
        let result = parts[0].parse::<i64>().unwrap();
        let numbers: Vec<i64> = parts[1]
            .split(" ")
            .map(|num| num.parse::<i64>().unwrap())
            .collect();
        ret.push(Equation { result, numbers });
    }
    ret
}

#[aoc(day7, part1)]
pub fn solve_part1(equations: &Vec<Equation>) -> i64 {
    equations
        .iter()
        .map(|e| if e.valid_part1() { e.result } else { 0 })
        .sum()
}

#[aoc(day7, part2)]
pub fn solve_part2(equations: &Vec<Equation>) -> i64 {
    equations
        .iter()
        .map(|e| if e.valid_part2() { e.result } else { 0 })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&input_generator(&TEST_INPUT)), 3749);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&input_generator(&TEST_INPUT)), 11387);
    }
}
