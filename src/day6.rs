use std::collections::HashSet;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Vec<HashSet<char>>> {
    let mut matrix = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars().collect::<Vec<_>>() {
            let mut r: HashSet<char> = HashSet::new();
            r.insert(c);
            row.push(r);
        }
        matrix.push(row);
    }
    matrix
}

pub fn find_position(matrix: &Vec<Vec<HashSet<char>>>, search: char) -> (usize, usize) {
    for (i, row) in matrix.iter().enumerate() {
        if let Some(j) = row.iter().position(|c| c.contains(&search)) {
            return (i, j);
        }
    }
    (0, 0)
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn next(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn to_char(&self) -> char {
        match self {
            Direction::Up => 'U',
            Direction::Right => 'R',
            Direction::Down => 'D',
            Direction::Left => 'L',
        }
    }
}

pub fn move_it_1(
    grid: &mut Vec<Vec<HashSet<char>>>,
    direction: Direction,
    position: (usize, usize),
) -> i32 {
    println!("{:?} {:?}", position, direction);

    let next_position: (i32, i32) = match direction {
        Direction::Up => (position.0 as i32 - 1, position.1 as i32),
        Direction::Right => (position.0 as i32, position.1 as i32 + 1),
        Direction::Down => (position.0 as i32 + 1, position.1 as i32),
        Direction::Left => (position.0 as i32, position.1 as i32 - 1),
    };
    if next_position.0 >= grid.len() as i32
        || next_position.1 >= grid[0].len() as i32
        || next_position.0 < 0
        || next_position.1 < 0
    {
        return grid.iter().flatten().filter(|&c| c.contains(&'X')).count() as i32;
    }

    let safe_next_position = (next_position.0 as usize, next_position.1 as usize);

    if grid[safe_next_position.0][safe_next_position.1].contains(&'#') {
        // grid[position.0][position.1].insert('X');
        return move_it_1(grid, direction.next(), position);
    }
    grid[safe_next_position.0][safe_next_position.1].insert('X');
    move_it_1(grid, direction, safe_next_position)
}

pub fn move_it_2(
    grid: &mut Vec<Vec<HashSet<char>>>,
    direction: Direction,
    position: (usize, usize),
) -> i32 {
    if grid[position.0][position.1].contains(&direction.to_char()) {
        return -1;
    }
    grid[position.0][position.1].insert(direction.to_char());
    grid[position.0][position.1].insert('X');

    // println!("{} {} {}", position.0, position.1, direction.to_char());

    let next_position: (i32, i32) = match direction {
        Direction::Up => (position.0 as i32 - 1, position.1 as i32),
        Direction::Right => (position.0 as i32, position.1 as i32 + 1),
        Direction::Down => (position.0 as i32 + 1, position.1 as i32),
        Direction::Left => (position.0 as i32, position.1 as i32 - 1),
    };
    if next_position.0 >= grid.len() as i32
        || next_position.1 >= grid[0].len() as i32
        || next_position.0 < 0
        || next_position.1 < 0
    {
        return 0;
    }

    let safe_next_position = (next_position.0 as usize, next_position.1 as usize);

    if grid[safe_next_position.0][safe_next_position.1].contains(&'#') {
        return move_it_2(grid, direction.next(), position);
    }
    grid[safe_next_position.0][safe_next_position.1].insert('X');
    move_it_2(grid, direction, safe_next_position)
}

#[aoc(day6, part1)]
pub fn solve_part1(grid: &Vec<Vec<HashSet<char>>>) -> i32 {
    let initial_position = find_position(grid, '^');
    let direction: Direction = Direction::Up;
    move_it_1(&mut grid.clone(), direction, initial_position)
}

// 1587 too high
#[aoc(day6, part2)]
pub fn solve_part2(grid: &Vec<Vec<HashSet<char>>>) -> i32 {
    let initial_position = find_position(grid, '^');
    let direction: Direction = Direction::Up;
    let mut count = 0;
    (1..grid.len())
        .flat_map(|row| (0..grid[0].len()).map(move |col| (row, col)))
        .for_each(|(row, col)| {
            if !grid[row][col].contains(&'#') {
                let mut grid_with_obstacle = grid.clone();
                grid_with_obstacle[row][col].insert('#');
                let res = move_it_2(
                    &mut grid_with_obstacle,
                    direction,
                    (initial_position.0, initial_position.1),
                );
                if res == -1 {
                    count += 1;
                }
            }
        });
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&input_generator(&TEST_INPUT)), 41);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&input_generator(&TEST_INPUT)), 6);
    }
}
