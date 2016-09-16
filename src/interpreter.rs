use std::io;
use std::io::{Read, Write};

const TAPE_SIZE: usize = 30000;

pub struct Machine {
    data_ptr: usize,
    tape: [u8; TAPE_SIZE],

    instr_ptr: usize,
    instr_tape: Vec<char>,
}

impl Machine {
    pub fn new(instr_tape: Vec<char>) -> Machine {
        Machine { 
            data_ptr: 0, 
            tape: [0; TAPE_SIZE],
            instr_ptr: 0,
            instr_tape: instr_tape,
        }
    }

    pub fn execute(&mut self, debug: bool) {
        while self.instr_ptr < self.instr_tape.len() {
            if debug { 
                println!("Data Ptr: {}\nData Value: {}\nInstruction Pointer: {}\nCurrent Instruction: {}\n", 
                         self.data_ptr,
                         self.tape[self.data_ptr],
                         self.instr_ptr,
                         self.instr_tape[self.instr_ptr]); 
            }
            self.execute_step();
            if self.data_ptr > TAPE_SIZE {
                panic!("Data pointer traveled beyond the bounds of the tape!");
            }
        }
    }

    fn execute_step(&mut self) {
        // fetch
        let instruction = self.instr_tape[self.instr_ptr];

        // decode/execute
        match instruction {
            '>' => self.data_ptr += 1,
            '<' => self.data_ptr -= 1,
            '+' => self.tape[self.data_ptr] += 1,
            '-' => self.tape[self.data_ptr] -= 1,
            ',' => {
                // Get only the first byte of input.
                let input = io::stdin()
                    .bytes()
                    .next()
                    .and_then(|result| result.ok())
                    .unwrap();
                self.tape[self.data_ptr] = input;
            },
            '.' => {
                let mut stdout = io::stdout();
                let output = self.tape[self.data_ptr];
                stdout.write(&[output]).ok();
            },
            '[' => {
                if self.tape[self.data_ptr] == 0 {
                    let mut counter = 1;
                    while counter > 0 {
                        self.instr_ptr += 1;
                        let current_instr = self.instr_tape[self.instr_ptr];
                        if current_instr == '[' { counter += 1; } 
                        else if current_instr == ']' { counter -= 1; }
                    }
                }
            },
            ']' => {
                if self.tape[self.data_ptr] != 0 {
                    let mut counter = 1;
                    while counter > 0 {
                        self.instr_ptr -= 1;
                        let current_instr = self.instr_tape[self.instr_ptr];
                        if current_instr == '[' { counter -= 1; } 
                        else if current_instr == ']' { counter += 1; }
                    }
                }
            }
            _ => {}, // do nothing
        }

        // increment
        self.instr_ptr += 1;
    }
}
