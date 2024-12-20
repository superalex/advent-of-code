use std::collections::VecDeque;
use std::io;

pub struct State {
    register_a: i64,
    register_b: i64,
    register_c: i64,
    output: Vec<i64>,
    instruction_pointer: i64,
    memory: Vec<i64>,
}

impl State {
    fn pp_output(&self) -> String {
        self.output
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
    fn pp_output_2(&self) -> String {
        self.output
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<_>>()
            .join("")
    }
}

pub fn literal_operand(operand: u8, state: &State) -> i64 {
    operand as i64
}

pub fn combo_operand(operand: u8, state: &State) -> i64 {
    match operand {
        0 => 0 as i64,
        1 => 1 as i64,
        2 => 2 as i64,
        3 => 3 as i64,
        4 => state.register_a,
        5 => state.register_b,
        6 => state.register_c,
        _ => 0, // ERROR
    }
}

// A number called the instruction pointer identifies the position in the program from which the next opcode will be read; it starts at 0, pointing at the first 3-bit number in the program. Except for jump instructions, the instruction pointer increases by 2 after each instruction is processed (to move past the instruction's opcode and its operand). If the computer tries to read an opcode past the end of the program, it instead halts.

pub fn run(state: &mut State) {
    while (state.instruction_pointer as usize) < state.memory.len() {
        let opcode = state
            .memory
            .get(state.instruction_pointer as usize)
            .unwrap();
        let operand = state
            .memory
            .get(state.instruction_pointer as usize + 1)
            .unwrap();
        match opcode {
            0 => {
                state.register_a = state.register_a
                    / (2_u32.pow(combo_operand((*operand) as u8, state).try_into().unwrap())
                        as i64);
                state.instruction_pointer += 2;
            }
            1 => {
                state.register_b = state.register_b ^ literal_operand((*operand) as u8, state);
                state.instruction_pointer += 2;
            }
            2 => {
                state.register_b = (combo_operand((*operand) as u8, state) % 8) & 0b111;
                state.instruction_pointer += 2;
            }
            3 => {
                if state.register_a != 0 {
                    state.instruction_pointer = literal_operand((*operand) as u8, state);
                } else {
                    state.instruction_pointer += 2;
                }
            }
            4 => {
                state.register_b = state.register_b ^ state.register_c;
                state.instruction_pointer += 2;
            }
            5 => {
                state
                    .output
                    .push((combo_operand((*operand) as u8, state) % 8) & 0b111);
                state.instruction_pointer += 2;
            }
            6 => {
                state.register_b = state.register_a
                    / (2_u32.pow(combo_operand((*operand) as u8, state).try_into().unwrap())
                        as i64);
                state.instruction_pointer += 2;
            }
            7 => {
                state.register_c = state.register_a
                    / (2_u32.pow(combo_operand((*operand) as u8, state).try_into().unwrap())
                        as i64);
                state.instruction_pointer += 2;
            }
            _ => {
                println!("ERROR");
            }
        }
    }
}

#[aoc(day17, part1)]
pub fn solve_part1(input: &str) -> String {
    let mut s = State {
        register_a: 23999685,
        register_b: 0,
        register_c: 0,
        instruction_pointer: 0,
        output: vec![],
        memory: vec![2, 4, 1, 1, 7, 5, 1, 5, 0, 3, 4, 4, 5, 5, 3, 0],
    };
    run(&mut s);
    s.pp_output()
}

#[aoc(day17, part2)]
pub fn solve_part2(input: &str) -> i64 {
    let o = vec![2, 4, 1, 1, 7, 5, 1, 5, 0, 3, 4, 4, 5, 5, 3, 0];
    let mut to_visit = VecDeque::from([(o.len(), 0)]);
    // Probamos valores de a comenzando desde 0
    let mut a = 0;
    while let Some((pos, current_value)) = to_visit.pop_front() {
      if pos == 0 {
          return current_value;
      }

      for i in 0..8 {
          let n_a = current_value * 8 + i;
          let b = ((n_a % 8) ^ 1 ^ 5 ^ (n_a / (1 << ((n_a % 8) ^ 1)) & 0b111)) & 0b111;

          if b == o[pos - 1] {
              to_visit.push_back((pos - 1, n_a));
          }
      }
  }

  0 // En caso de no encontrar la solución
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_2() {
        let mut s = State {
            register_a: 33,
            register_b: 66,
            register_c: 9,
            instruction_pointer: 0,
            output: vec![],
            memory: vec![2, 6],
        };
        run(&mut s);
        assert_eq!(s.register_b, 1);
    }
    #[test]
    fn run_5() {
        let mut s = State {
            register_a: 10,
            register_b: 0,
            register_c: 0,
            instruction_pointer: 0,
            output: vec![],
            memory: vec![5, 0, 5, 1, 5, 4],
        };
        run(&mut s);
        assert_eq!(s.pp_output(), "0,1,2");
    }

    #[test]
    fn run_0() {
        let mut s = State {
            register_a: 2024,
            register_b: 0,
            register_c: 0,
            instruction_pointer: 0,
            output: vec![],
            memory: vec![0, 1, 5, 4, 3, 0],
        };
        run(&mut s);
        assert_eq!(s.register_a, 0);
        assert_eq!(s.pp_output(), "4,2,5,6,7,7,7,7,3,1,0");
    }

    #[test]
    fn run_17() {
        let mut s = State {
            register_a: 0,
            register_b: 29,
            register_c: 0,
            instruction_pointer: 0,
            output: vec![],
            memory: vec![1, 7],
        };
        run(&mut s);
        assert_eq!(s.register_b, 26);
    }

    #[test]
    fn run_40() {
        let mut s = State {
            register_a: 0,
            register_b: 2024,
            register_c: 43690,
            instruction_pointer: 0,
            output: vec![],
            memory: vec![4, 0],
        };
        run(&mut s);
        assert_eq!(s.register_b, 44354);
    }

    #[test]
    fn run_015430() {
        let mut s = State {
            register_a: 729,
            register_b: 0,
            register_c: 0,
            instruction_pointer: 0,
            output: vec![],
            memory: vec![0, 1, 5, 4, 3, 0],
        };
        run(&mut s);
        assert_eq!(s.pp_output(), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn run_035430() {
        let mut s = State {
            register_a: 117440,
            register_b: 0,
            register_c: 0,
            instruction_pointer: 0,
            output: vec![],
            memory: vec![0, 3, 5, 4, 3, 0],
        };
        run(&mut s);
        assert_eq!(s.pp_output(), "0,3,5,4,3,0");
    }

    #[test]
    fn run_164516454365621() {
        let mut s = State {
            register_a: 164516454365621,
            register_b: 0,
            register_c: 0,
            instruction_pointer: 0,
            output: vec![],
            memory: vec![2, 4, 1, 1, 7, 5, 1, 5, 0, 3, 4, 4, 5, 5, 3, 0],
        };
        run(&mut s);
        assert_eq!(s.pp_output(), "2,4,1,1,7,5,1,5,0,3,4,4,5,5,3,0");
    }

    #[test]
    fn fast() {
        let mut a: i64 = 23999685;
        let mut b: i64 = 0;
        let mut c: i64 = 0;
        let mut o: Vec<i64> = vec![];

        while a != 0 {
            b = (a % 8) & 0b111;
            b = b ^ 1;
            c = a / ((2_u32.pow(b as u32)) as i64);
            b = b ^ 5;
            a = a / ((2_u32.pow(3 as u32)) as i64);
            b = b ^ c;
            o.push((b % 8) & 0b111);
        }
        assert_eq!(o, vec![5, 0, 3, 5, 7, 6, 1, 5, 4]);
    }

    #[test]
    fn fast_2() {
        let mut a: i64 = 23999685;
        let mut b: i64 = 0;
        let mut c: i64 = 0;
        let mut o: Vec<i64> = vec![];

        while a != 0 {
            b = (a % 8) & 0b111; // Extrae los últimos 3 bits de `a` (dígito en base 8).
            b ^= 1; // XOR con 1.
            c = a / (1 << b); // Divide `a` por 2^b.
            b ^= 5; // XOR con 5.
            a /= 8; // Divide `a` entre 8.
            b ^= c; // XOR con `c`.
            o.push(b & 0b111); // Mantén sólo los últimos 3 bits de `b`.
        }

        assert_eq!(o, vec![5, 0, 3, 5, 7, 6, 1, 5, 4]);
    }

    #[test]
    fn fast_3() {
        let mut a: i64 = 23999685;
        let mut o: Vec<i64> = vec![];

        while a != 0 {
            let b = ((a % 8) ^ 1 ^ 5 ^ (a / (1 << ((a % 8) ^ 1)) & 0b111)) & 0b111;
            a /= 8;
            o.push(b);
        }

        assert_eq!(o, vec![5, 0, 3, 5, 7, 6, 1, 5, 4]);
    }

    #[test]
    fn test_asd() {
        let mut s = State {
            register_a: 164516454365621,
            register_b: 0,
            register_c: 0,
            instruction_pointer: 0,
            output: vec![],
            memory: vec![2, 4, 1, 1, 7, 5, 1, 5, 0, 3, 4, 4, 5, 5, 3, 0],
        };
        run(&mut s);
        assert_eq!(s.pp_output(), "2,4,1,1,7,5,1,5,0,3,4,4,5,5,3,0");
    }    
}
