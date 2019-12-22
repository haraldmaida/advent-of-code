//! # Day 2: 1202 Program Alarm
//!
//! On the way to your gravity assist around the Moon, your ship computer beeps
//! angrily about a "1202 program alarm". On the radio, an Elf is already
//! explaining how to handle the situation: "Don't worry, that's perfectly
//! norma--" The ship computer bursts into flames.
//!
//! You notify the Elves that the computer's magic smoke seems to have escaped.
//! "That computer ran Intcode programs like the gravity assist program it was
//! working on; surely there are enough spare parts up there to build a new
//! Intcode computer!"
//!
//! An Intcode program is a list of integers separated by commas
//! (like 1,0,0,3,99). To run one, start by looking at the first integer
//! (called position 0). Here, you will find an opcode - either 1, 2, or 99.
//! The opcode indicates what to do; for example, 99 means that the program is
//! finished and should immediately halt. Encountering an unknown opcode means
//! something went wrong.
//!
//! Opcode 1 adds together numbers read from two positions and stores the result
//! in a third position. The three integers immediately after the opcode tell
//! you these three positions - the first two indicate the positions from which
//! you should read the input values, and the third indicates the position at
//! which the output should be stored.
//!
//! For example, if your Intcode computer encounters 1,10,20,30, it should read
//! the values at positions 10 and 20, add those values, and then overwrite the
//! value at position 30 with their sum.
//!
//! Opcode 2 works exactly like opcode 1, except it multiplies the two inputs
//! instead of adding them. Again, the three integers after the opcode indicate
//! where the inputs and outputs are, not their values.
//!
//! Once you're done processing an opcode, move to the next one by stepping
//! forward 4 positions.
//!
//! For example, suppose you have the following program:
//!
//! ```text
//! 1,9,10,3,2,3,11,0,99,30,40,50
//! ```
//!
//! For the purposes of illustration, here is the same program split into
//! multiple lines:
//!
//! ```text
//! 1,9,10,3,
//! 2,3,11,0,
//! 99,
//! 30,40,50
//! ````
//!
//! The first four integers, 1,9,10,3, are at positions 0, 1, 2, and 3.
//! Together, they represent the first opcode (1, addition), the positions of
//! the two inputs (9 and 10), and the position of the output (3). To handle
//! this opcode, you first need to get the values at the input positions:
//! position 9 contains 30, and position 10 contains 40. Add these numbers
//! together to get 70. Then, store this value at the output position; here,
//! the output position (3) is at position 3, so it overwrites itself.
//! Afterward, the program looks like this:
//!
//! ```text
//! 1,9,10,70,
//! 2,3,11,0,
//! 99,
//! 30,40,50
//! ```
//!
//! Step forward 4 positions to reach the next opcode, 2. This opcode works just
//! like the previous, but it multiplies instead of adding. The inputs are at
//! positions 3 and 11; these positions contain 70 and 50 respectively.
//! Multiplying these produces 3500; this is stored at position 0:
//!
//! ```text
//! 3500,9,10,70,
//! 2,3,11,0,
//! 99,
//! 30,40,50
//! ```
//!
//! Stepping forward 4 more positions arrives at opcode 99, halting the program.
//!
//! Here are the initial and final states of a few more small programs:
//!
//! * `1,0,0,0,99` becomes `2,0,0,0,99` (1 + 1 = 2).
//! * `2,3,0,3,99` becomes `2,3,0,6,99` (3 * 2 = 6).
//! * `2,4,4,5,99,0` becomes `2,4,4,5,99,9801` (99 * 99 = 9801).
//! * `1,1,1,4,99,5,6,0,99` becomes `30,1,1,4,2,5,6,0,99`.
//!
//! Once you have a working computer, the first step is to restore the gravity
//! assist program (your puzzle input) to the "1202 program alarm" state it had
//! just before the last computer caught fire. To do this, before running the
//! program, replace position 1 with the value 12 and replace position 2 with
//! the value 2. What value is left at position 0 after the program halts?
//!
//! [Advent of Code 2019 - Day 2](https://adventofcode.com/2019/day/2)

use std::fmt::{self, Display};
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    Add,
    Mul,
    Halt,
}

impl Opcode {
    fn decode(int: i32) -> Option<Self> {
        match int {
            1 => Some(Opcode::Add),
            2 => Some(Opcode::Mul),
            99 => Some(Opcode::Halt),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Memory(Vec<i32>);

impl Memory {
    pub fn init(data: &[i32]) -> Self {
        Memory(Vec::from_iter(data.into_iter().copied()))
    }

    pub fn read_word(&self, addr: i32) -> i32 {
        self.0[addr as usize]
    }

    pub fn write_word(&mut self, addr: i32, data: i32) {
        self.0[addr as usize] = data;
    }
}

impl Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Deref for Memory {
    type Target = [i32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Memory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Next {
    Continue,
    Halt,
}

#[derive(Debug)]
pub struct IntVM {
    pc: usize,
}

impl Default for IntVM {
    fn default() -> Self {
        Self { pc: 0 }
    }
}

impl IntVM {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn reset(&mut self) {
        let default = IntVM::default();
        self.pc = default.pc;
    }

    pub fn run(&mut self, memory: &mut Memory) {
        while self.step(memory) == Next::Continue {}
    }

    pub fn step(&mut self, memory: &mut Memory) -> Next {
        let opcode_word = self.read_next_word(memory);
        if let Some(opcode) = Opcode::decode(opcode_word) {
            self.execute_instruction(opcode, memory)
        } else {
            self.panic(&format!("unknown opcode {}", opcode_word))
        }
    }

    fn halt(&mut self) -> Next {
        Next::Halt
    }

    fn panic(&mut self, message: &str) -> Next {
        error!(target: "IntVM", "{}", message);
        Next::Halt
    }

    fn read_next_word(&mut self, memory: &mut Memory) -> i32 {
        let data = memory.read_word(self.pc as i32);
        self.pc += 1;
        data
    }

    pub fn execute_instruction(&mut self, opcode: Opcode, memory: &mut Memory) -> Next {
        match opcode {
            Opcode::Add => {
                let in1_addr = self.read_next_word(memory);
                let in2_addr = self.read_next_word(memory);
                let out_addr = self.read_next_word(memory);
                let lhs = memory.read_word(in1_addr);
                let rhs = memory.read_word(in2_addr);
                let result = lhs.wrapping_add(rhs);
                memory.write_word(out_addr, result);
                Next::Continue
            }
            Opcode::Mul => {
                let in1_addr = self.read_next_word(memory);
                let in2_addr = self.read_next_word(memory);
                let out_addr = self.read_next_word(memory);
                let lhs = memory.read_word(in1_addr);
                let rhs = memory.read_word(in2_addr);
                let result = lhs.wrapping_mul(rhs);
                memory.write_word(out_addr, result);
                Next::Continue
            }
            Opcode::Halt => self.halt(),
        }
    }
}

#[aoc_generator(day2)]
pub fn parse(input: &str) -> Vec<i32> {
    input
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            line.trim().split(',').map(move |word| {
                i32::from_str(word.trim()).unwrap_or_else(|_| {
                    panic!(
                        "word at line {:03} is not a valid i32, but is {:?}",
                        idx + 1,
                        word
                    )
                })
            })
        })
        .flatten()
        .collect()
}

#[aoc(day2, part1)]
pub fn run_original_program(input: &[i32]) -> Memory {
    let mut memory = Memory::init(input);
    // restore "1202 program alarm"
    memory.write_word(1, 12);
    memory.write_word(2, 2);

    let mut int_vm = IntVM::default();
    int_vm.run(&mut memory);
    memory
}
