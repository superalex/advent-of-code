#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    let mut matrix = Vec::new();
    for line in input.lines() {
        let row: Vec<char> = line.chars().collect();
        matrix.push(row);
    }
    matrix
}

pub fn xmas(word: String) -> bool {
    word == "XMAS" || word.chars().rev().collect::<String>() == "XMAS"
}

pub fn mas(word: String) -> bool {
    word == "MAS" || word.chars().rev().collect::<String>() == "MAS"
}

#[aoc(day4, part1)]
pub fn solve_part1(grid: &Vec<Vec<char>>) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut counter = 0;
    // Horizontal
    for i in 0..rows {
        for j in 0..=cols - 4 {
            let mut word = String::new();
            for k in 0..4 {
                word.push(grid[i][j + k]);
            }
            if xmas(word) {
                counter += 1;
            }
        }
    }

    // Vertical
    for j in 0..cols {
        for i in 0..=rows - 4 {
            let mut word = String::new();
            for k in 0..4 {
                word.push(grid[i + k][j]);
            }
            if xmas(word) {
                counter += 1;
            }
        }
    }

    // Diagonal
    for i in 0..=rows - 4 {
        for j in 0..=cols - 4 {
            let mut word = String::new();
            for k in 0..4 {
                word.push(grid[i + k][j + k]);
            }
            if xmas(word) {
                counter += 1;
            }
        }
    }

    // Diagonal
    for i in 0..=rows - 4 {
        for j in 3..cols {
            let mut word = String::new();
            for k in 0..4 {
                word.push(grid[i + k][j - k]);
            }
            if xmas(word) {
                counter += 1;
            }
        }
    }
    counter
}

#[aoc(day4, part2)]
pub fn solve_part2(grid: &Vec<Vec<char>>) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut counter = 0;

    for i in 1..=rows - 2 {
        for j in 1..=cols - 2 {
            let mut word1 = String::new();
            word1.push(grid[i - 1][j - 1]);
            word1.push(grid[i][j]);
            word1.push(grid[i + 1][j + 1]);
            let mut word2 = String::new();
            word2.push(grid[i - 1][j + 1]);
            word2.push(grid[i][j]);
            word2.push(grid[i + 1][j - 1]);
            if mas(word1) && mas(word2) {
                counter += 1;
            }
        }
    }
    counter
}
