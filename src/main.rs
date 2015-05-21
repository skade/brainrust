extern crate ncurses;

use std::env;
use std::char;
use std::io::prelude::*;
use std::fs::File;
use ncurses::*;

#[derive(Debug)]
enum Instruction {
    Forward,
    Backward,
    Increment,
    Decrement,
    Output,
    Input,
    JumpForwardIfZero,
    JumpBackwardIfNonZero
}

#[derive(Debug)]
enum MachineError {
    AddressBelowZero,
    SyntaxError
}

struct Machine {
    mem: Vec<u32>,
    program: Vec<Instruction>,
    p: usize,
    m: usize,
}

impl Machine {
    fn new() -> Machine {
        Machine {
            mem: vec![0],
            program: vec![],
            m: 0,
            p: 0,
        }
    }

    fn parse(&mut self, input:String) -> () {
        for c in input.chars() {
            match c {
                '>' => self.program.push(Instruction::Forward),
                '<' => self.program.push(Instruction::Backward),
                '+' => self.program.push(Instruction::Increment),
                '-' => self.program.push(Instruction::Decrement),
                '.' => self.program.push(Instruction::Output),
                ',' => self.program.push(Instruction::Input),
                '[' => self.program.push(Instruction::JumpForwardIfZero),
                ']' => self.program.push(Instruction::JumpBackwardIfNonZero),
                _ => ()
            }
        }
    }

    fn step(&mut self) -> Result<(), MachineError> {
        match self.program[self.p] {
            Instruction::Forward => {
                // will the new index be beyond the vector bounds?
                // can't compare with len 1 because that might be -1
                if self.m + 2 > self.mem.len() {
                    self.mem.push(0);
                }
                self.m += 1
            },
            Instruction::Backward => {
                if self.m == 0 {
                    return Err(MachineError::AddressBelowZero);
                }
                self.m -= 1
            },
            Instruction::Increment => self.mem[self.m] += 1,
            Instruction::Decrement => self.mem[self.m] -=  1,
            Instruction::Output => match char::from_u32(self.mem[self.m]) {
                Some(c) => {
                    printw(&format!("{}", c));
                    refresh();
                },
                _ => ()
            },
            //TODO: actual IO input
            Instruction::Input => self.mem[self.m] = getch() as u32,
            Instruction::JumpForwardIfZero => if self.mem[self.m] == 0 {
                let mut n = 1;
                while n > 0 {
                    //will the program pointer hit the end of the program?
                    if self.p + 2 > self.program.len() {
                        return Err(MachineError::SyntaxError);
                    }
                    self.p += 1;
                    match self.program[self.p] {
                        Instruction::JumpForwardIfZero => n += 1,
                        Instruction::JumpBackwardIfNonZero => n -= 1,
                        _ => ()
                    }
                }
            },
            Instruction::JumpBackwardIfNonZero => if self.mem[self.m] != 0 {
                let mut n = 1;
                while n > 0 {
                    //will the program pointer hit the beginning of the program?
                    if self.p == 0 {
                        return Err(MachineError::SyntaxError);
                    }
                    self.p -= 1;
                    match self.program[self.p] {
                        Instruction::JumpBackwardIfNonZero => n += 1,
                        Instruction::JumpForwardIfZero => n -= 1,
                        _ => ()
                    }
                }
            }
        }

        self.p += 1;

        Ok(())
    }

    fn run(&mut self) -> Result<(), MachineError> {
        loop {
            if self.p > self.program.len() - 1 {
                return Ok(());
            }

            try!(self.step());
        }
    }
}

fn run_code(code:String) -> () {
    initscr();
    cbreak();
    keypad(stdscr, true);
    noecho();

    let mut machine = Machine::new();
    machine.parse(code);
    match machine.run() {
        Err(e) => {
            printw(&format!("{:?}", e));
            refresh();
        },
        _ => ()
    }

    printw("\n\n---\nProgram terminated.\nPress any key to quit.");
    refresh();
    getch();
    endwin();
}

fn load_source(filename:String) -> Result<String, ()> {
    match File::open(filename) {
        Ok(mut f) => {
            let mut code = String::new();
            match f.read_to_string(&mut code) {
                Ok(_) => Ok(code),
                _ => Err(())
            }
        },
        _ => Err(())
    }
}


fn main() {
    match env::args().nth(1) {
        Some(filename) => match load_source(filename) {
            Ok(code) => run_code(code),
            _ => {println!("Can't read input file.");}
        },
        _ => println!("Source file argument missing")
    }
}
