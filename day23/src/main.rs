use std::fs;

enum Register {
    A,
    B,
}

struct Computer {
    program: Vec<String>,
    register_a: usize,
    register_b: usize,
    pc: i32,
}

impl Computer {
    fn new(program: &str) -> Self {
        Computer {
            program: program.lines().map(String::from).collect(),
            register_a: 0,
            register_b: 0,
            pc: 0,
        }
    }

    fn run(&mut self) {
        loop {
            if self.pc >= self.program.len() as i32 {
                break;
            }

            let parts: Vec<&str> = self.program[self.pc as usize].split(' ').collect();

            match parts[0] {
                "hlf" => {
                    let register = Computer::parse_register(parts[1]);
                    self.hlf(register);
                }
                "tpl" => {
                    let register = Computer::parse_register(parts[1]);
                    self.tpl(register);
                }
                "inc" => {
                    let register = Computer::parse_register(parts[1]);
                    self.inc(register);
                }
                "jmp" => {
                    let offset = Computer::parse_offset(parts[1]);
                    self.jmp(offset);
                }
                "jie" => {
                    let register = Computer::parse_register(parts[1]);
                    let offset = Computer::parse_offset(parts[2]);
                    self.jie(register, offset);
                }
                "jio" => {
                    let register = Computer::parse_register(parts[1]);
                    let offset = Computer::parse_offset(parts[2]);
                    self.jio(register, offset);
                }
                _ => panic!("unknown instruction"),
            }
        }
    }

    fn hlf(&mut self, register: Register) {
        match register {
            Register::A => self.register_a /= 2,
            Register::B => self.register_b /= 2,
        }

        self.pc += 1;
    }

    fn tpl(&mut self, register: Register) {
        match register {
            Register::A => self.register_a *= 3,
            Register::B => self.register_b *= 3,
        }

        self.pc += 1;
    }

    fn inc(&mut self, register: Register) {
        match register {
            Register::A => self.register_a += 1,
            Register::B => self.register_b += 1,
        }

        self.pc += 1;
    }

    fn jmp(&mut self, offset: i32) {
        self.pc += offset;
    }

    fn jie(&mut self, register: Register, offset: i32) {
        match register {
            Register::A => {
                if self.register_a % 2 == 0 {
                    self.pc += offset;
                } else {
                    self.pc += 1;
                }
            }
            Register::B => {
                if self.register_b % 2 == 0 {
                    self.pc += offset;
                } else {
                    self.pc += 1;
                }
            }
        }
    }

    fn jio(&mut self, register: Register, offset: i32) {
        match register {
            Register::A => {
                if self.register_a == 1 {
                    self.pc += offset;
                } else {
                    self.pc += 1;
                }
            }
            Register::B => {
                if self.register_b == 1 {
                    self.pc += offset;
                } else {
                    self.pc += 1;
                }
            }
        }
    }

    fn parse_register(register: &str) -> Register {
        match register.trim_end_matches(',') {
            "a" => Register::A,
            "b" => Register::B,
            _ => panic!("unknown register"),
        }
    }

    fn parse_offset(offset: &str) -> i32 {
        offset.parse::<i32>().unwrap()
    }
}

fn part_1(input: &str) -> usize {
    let mut computer = Computer::new(input);
    computer.run();

    computer.register_b
}

fn part_2(input: &str) -> usize {
    let mut computer = Computer::new(input);
    computer.register_a = 1;
    computer.run();

    computer.register_b
}

fn main() {
    let input = fs::read_to_string("input").expect("file not found");
    let input = input.trim();

    assert_eq!(184, part_1(input));
    assert_eq!(231, part_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_computer() {
        let program = "inc a\n\
                       jio a, +2\n\
                       tpl a\n\
                       inc a";
        let mut computer = Computer::new(program);
        computer.run();

        assert_eq!(2, computer.register_a);
    }
}
