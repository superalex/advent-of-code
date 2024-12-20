fn diskmap_to_blocks(d: &str) -> Vec<String> {
    let chars = d.chars().collect::<Vec<char>>();
    let mut output: Vec<String> = vec![];
    let mut id = 0;
    chars.chunks(2).for_each(|w| {
        for _ in 0..w[0].to_digit(10).unwrap() {
            output.push(id.to_string());
        }
        if w.len() > 1 {
            for _ in 0..w[1].to_digit(10).unwrap() {
                output.push(".".to_string());
            }
        }

        id += 1;
    });

    output
}

fn move_blocks(mut d: Vec<String>) -> Vec<String> {
    let mut start = 0;
    let mut end = d.len() - 1;

    while start < end {
        if d[start] == "." && d[end] != "." {
            d.swap(start, end);
        }
        if d[start] != "." {
            start += 1;
        }
        if d[end] == "." {
            end -= 1;
        }
    }
    d
}

fn move_blocks_2(mut d: Vec<String>) -> Vec<String> {
  if d.is_empty() {
      return d; // Devuelve directamente si el vector está vacío
  }

  let mut start = 0; // Apuntamos al inicio de la lista
  let mut end = d.len() - 1; // Apuntamos al final de la lista

  while end > 0 {
      start = 0;

      // Si encontramos un bloque no ".", intentamos moverlo al inicio
      if d[end] != "." {
          let mut block_end = end;

          // Encontramos el bloque completo de caracteres iguales (diferentes a ".")
          while block_end > 0 && d[block_end] == d[end] {
              block_end -= 1;
          }
          block_end += 1; // Corrige la posición al final del bloque

          let block_size = end - block_end + 1; // Tamaño del bloque

          // Buscamos el primer hueco lo suficientemente grande para el bloque
          while start < block_end {
              // Buscamos un hueco (".")
              if d[start] == "." {
                  // Verificamos si hay espacio suficiente
                  let is_space_large_enough = (start + block_size <= d.len())
                      && d[start..start + block_size].iter().all(|x| x == ".");

                  if is_space_large_enough {
                      // Mover el bloque completo al inicio
                      let block = d[block_end..=end].to_vec();
                      d.splice(start..start + block_size, block.into_iter());

                      // Eliminar el bloque original dejando huecos
                      for i in block_end..=end {
                          d[i] = ".".to_string();
                      }

                      break; // Salimos del bucle para buscar el siguiente bloque
                  }
              }
              start += 1; // Avanzamos el puntero de inicio
          }

          // Actualizamos `end` para buscar el siguiente bloque
          end = block_end - 1;
      } else {
          end -= 1; // Si no hay bloque, movemos el índice hacia la izquierda
      }
  }

  d
}

fn checksum(d: Vec<String>) -> i64 {
    let mut r: i64 = 0;
    for i in 0..d.len() {
        let s = d.get(i).unwrap();
        if s != "." {
            r += i as i64 * d.get(i).unwrap().parse::<i64>().unwrap();
        }
    }
    r
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &str) -> i64 {
    let blocks = diskmap_to_blocks(input);
    let moved_blocks = move_blocks(blocks);
    checksum(moved_blocks)
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &str) -> i64 {
    let blocks = diskmap_to_blocks(input);
    let moved_blocks = move_blocks_2(blocks);
    let moved_blocks_2 = move_blocks_2(moved_blocks);
    checksum(moved_blocks_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(
            diskmap_to_blocks(&"12345").into_iter().collect::<String>(),
            "0..111....22222"
        );
        assert_eq!(
            diskmap_to_blocks(&"2333133121414131402")
                .into_iter()
                .collect::<String>(),
            "00...111...2...333.44.5555.6666.777.888899"
        );
        assert_eq!(
            diskmap_to_blocks(&"233313312141413140202")
                .into_iter()
                .collect::<String>(),
            "00...111...2...333.44.5555.6666.777.8888991010"
        );

        assert_eq!(
            move_blocks(diskmap_to_blocks(&"12345"))
                .into_iter()
                .collect::<String>(),
            "022111222......"
        );
        assert_eq!(
            move_blocks(diskmap_to_blocks(&"2333133121414131402"))
                .into_iter()
                .collect::<String>(),
            "0099811188827773336446555566.............."
        );
        assert_eq!(
            checksum(move_blocks(diskmap_to_blocks(&"2333133121414131402"))),
            1928
        );
        assert_eq!(solve_part1(&"2333133121414131402"), 1928);
    }

    #[test]
    fn part2() {
        assert_eq!(
            move_blocks_2(diskmap_to_blocks(&"2333133121414131402"))
                .into_iter()
                .collect::<String>(),
            "00992111777.44.333....5555.6666.....8888.."
        );
    }
}



// 13098003610339 too high
    // 6376066430744 too low
// That's not the right answer; your answer is too high. If you're stuck, make sure you're using the full input data; there are also some general tips on the about page, or you can ask for hints on the subreddit. Please wait one minute before trying again. [Return to Day 9]