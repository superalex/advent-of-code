use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i32, // índice de fila
    y: i32, // índice de columna
}

fn calculate_new_points_v1(
    matrix: &Vec<Vec<char>>,
    p1: Point,
    p2: Point,
) -> Option<HashSet<Point>> {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;

    // Calculamos los nuevos puntos en la recta
    let p3 = Point {
        x: p1.x - dx,
        y: p1.y - dy,
    };

    let p4 = Point {
        x: p2.x + dx,
        y: p2.y + dy,
    };

    // println!("p1 {:?} p4 {:?} p2 {:?} p4 {:?}", p1, p2, p3, p4);

    // Verificamos si los puntos calculados están dentro de los límites de la matriz
    let rows: i32 = (matrix.len()) as i32;
    let cols: i32 = (matrix[0].len()) as i32;

    let mut valid_points = HashSet::new();

    if p3.x >= 0 && p3.x < rows && p3.y >= 0 && p3.y < cols {
        valid_points.insert(p3);
    }

    if p4.x >= 0 && p4.x < rows && p4.y >= 0 && p4.y < cols {
        valid_points.insert(p4);
    }

    // Si hay puntos válidos, los devolvemos. Si no, devolvemos None.
    if !valid_points.is_empty() {
        Some(valid_points)
    } else {
        None
    }
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut matrix = Vec::new();
    for line in input.lines() {
        let row: Vec<char> = line.chars().collect();
        matrix.push(row);
    }

    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (i, row) in matrix.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let c = matrix[i][j];
            if c != '.' && c != '\n' {
                println!("{} {} {}", c, i, j);
                antennas.entry(c).or_insert(Vec::new()).push((i, j));
            }
        }
    }
    println!("antennas {:?}", antennas);

    let mut valid_points = HashSet::new();

    for (key, coords) in &antennas {
        println!("coords {:?}", coords);
        for (i, (x1, y1)) in coords.clone().iter().enumerate() {
            for (j, (x2, y2)) in coords.clone().iter().enumerate() {
                if i <= j {
                    if x1 != x2 && y1 != y2 {
                        match calculate_new_points_v1(
                            &matrix,
                            Point {
                                x: *x1 as i32,
                                y: *y1 as i32,
                            },
                            Point {
                                x: *x2 as i32,
                                y: *y2 as i32,
                            },
                        ) {
                            Some(points) => {
                                for p in points.clone() {
                                    valid_points.insert(p);
                                }
                                println!("Valid points {key} ({x1} {y1}) ({x2} {y2}) {:?}", points);
                            }
                            None => {
                                println!("No se encontraron puntos válidos en la recta.");
                            }
                        }
                    }
                }
            }
        }
    }
    valid_points.len()
}

fn calculate_line_points(matrix: &[Vec<char>], p1: Point, p2: Point) -> HashSet<Point> {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;

    // Obtenemos los límites de la matriz
    let rows = matrix.len() as i32;
    let cols = matrix[0].len() as i32;

    // Función para verificar si un punto está dentro de los límites
    let is_within_bounds =
        |point: &Point| point.x >= 0 && point.x < rows && point.y >= 0 && point.y < cols;

    // Genera todos los puntos en una dirección dada
    let generate_points = |start: Point, dx: i32, dy: i32| {
        let mut points = HashSet::new();
        let mut current = start;
        loop {
            current = Point {
                x: current.x + dx,
                y: current.y + dy,
            };
            if !is_within_bounds(&current) {
                break;
            }
            points.insert(current);
        }
        points
    };

    // Generar puntos en ambas direcciones
    let mut points = HashSet::new();
    points.extend(generate_points(p1, -dx, -dy)); // Dirección inversa
    points.extend(generate_points(p2, dx, dy)); // Dirección directa

    points
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut matrix = Vec::new();
    for line in input.lines() {
        let row: Vec<char> = line.chars().collect();
        matrix.push(row);
    }

    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (i, row) in matrix.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let c = matrix[i][j];
            if c != '.' && c != '\n' {
                println!("{} {} {}", c, i, j);
                antennas.entry(c).or_insert(Vec::new()).push((i, j));
            }
        }
    }
    println!("antennas {:?}", antennas);

    let mut valid_points = HashSet::new();

    for (key, coords) in &antennas {
        println!("coords {:?}", coords);
        for (i, (x1, y1)) in coords.clone().iter().enumerate() {
            for (j, (x2, y2)) in coords.clone().iter().enumerate() {
                if i <= j {
                    if x1 != x2 && y1 != y2 {
                        let points = calculate_line_points(
                            &matrix,
                            Point {
                                x: *x1 as i32,
                                y: *y1 as i32,
                            },
                            Point {
                                x: *x2 as i32,
                                y: *y2 as i32,
                            },
                        );

                        valid_points.insert(Point {
                            x: *x1 as i32,
                            y: *y1 as i32,
                        });
                        valid_points.insert(Point {
                            x: *x2 as i32,
                            y: *y2 as i32,
                        });

                        for p in points.clone() {
                            valid_points.insert(p);
                        }
                        println!("Valid points {key} ({x1} {y1}) ({x2} {y2}) {:?}", points);
                    }
                }
            }
        }
    }
    valid_points.len()
}

// 1304 to low

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&TEST_INPUT), 14);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&TEST_INPUT), 34);
    }
}
