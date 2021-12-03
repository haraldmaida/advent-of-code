//! # Day 8: Handheld Halting
//!
//! Your flight to the major airline hub reaches cruising altitude without
//! incident. While you consider checking the in-flight menu for one of those
//! drinks that come with a little umbrella, you are interrupted by the kid
//! sitting next to you.
//!
//! Their handheld game console won't turn on! They ask if you can take a look.
//!
//! You narrow the problem down to a strange infinite loop in the boot code
//! (your puzzle input) of the device. You should be able to fix it, but first
//! you need to be able to run the code in isolation.
//!
//! The boot code is represented as a text file with one instruction per line of
//! text. Each instruction consists of an operation (acc, jmp, or nop) and an
//! argument (a signed number like +4 or -20).
//!
//! * acc increases or decreases a single global value called the accumulator by
//!   the value given in the argument. For example, acc +7 would increase the
//!   accumulator by 7. The accumulator starts at 0. After an acc instruction,
//!   the instruction immediately below it is executed next.
//! * jmp jumps to a new instruction relative to itself. The next instruction to
//!   execute is found using the argument as an offset from the jmp instruction;
//!   for example, jmp +2 would skip the next instruction, jmp +1 would continue
//!   to the instruction immediately below it, and jmp -20 would cause the
//!   instruction 20 lines above to be executed next.
//! * nop stands for No OPeration - it does nothing. The instruction immediately
//!   below it is executed next.
//!
//! For example, consider the following program:
//!
//! ```text
//! nop +0
//! acc +1
//! jmp +4
//! acc +3
//! jmp -3
//! acc -99
//! acc +1
//! jmp -4
//! acc +6
//! ```
//!
//! These instructions are visited in this order:
//!
//! ```text
//! nop +0  | 1
//! acc +1  | 2, 8(!)
//! jmp +4  | 3
//! acc +3  | 6
//! jmp -3  | 7
//! acc -99 |
//! acc +1  | 4
//! jmp -4  | 5
//! acc +6  |
//! ```
//!
//! First, the nop +0 does nothing. Then, the accumulator is increased from 0
//! to 1 (acc +1) and jmp +4 sets the next instruction to the other acc +1 near
//! the bottom. After it increases the accumulator from 1 to 2, jmp -4 executes,
//! setting the next instruction to the only acc +3. It sets the accumulator to
//! 5, and jmp -3 causes the program to continue back at the first acc +1.
//!
//! This is an infinite loop: with this sequence of jumps, the program will run
//! forever. The moment the program tries to run any instruction a second time,
//! you know it will never terminate.
//!
//! Immediately before the program would run an instruction a second time, the
//! value in the accumulator is 5.
//!
//! Run your copy of the boot code. Immediately before any instruction is
//! executed a second time, what value is in the accumulator?
//!
//! ## Part 2
//!
//! After some careful analysis, you believe that exactly one instruction is
//! corrupted.
//!
//! Somewhere in the program, either a jmp is supposed to be a nop, or a nop is
//! supposed to be a jmp. (No acc instructions were harmed in the corruption of
//! this boot code.)
//!
//! The program is supposed to terminate by attempting to execute an instruction
//! immediately after the last instruction in the file. By changing exactly one
//! jmp or nop, you can repair the boot code and make it terminate correctly.
//!
//! For example, consider the same program from above:
//!
//! ```text
//! nop +0
//! acc +1
//! jmp +4
//! acc +3
//! jmp -3
//! acc -99
//! acc +1
//! jmp -4
//! acc +6
//! ```
//!
//! If you change the first instruction from nop +0 to jmp +0, it would create a
//! single-instruction infinite loop, never leaving that instruction. If you
//! change almost any of the jmp instructions, the program will still eventually
//! find another jmp instruction and loop forever.
//!
//! However, if you change the second-to-last instruction (from jmp -4 to nop
//! -4), the program terminates! The instructions are visited in this order:
//!
//! ```text
//! nop +0  | 1
//! acc +1  | 2
//! jmp +4  | 3
//! acc +3  |
//! jmp -3  |
//! acc -99 |
//! acc +1  | 4
//! nop -4  | 5
//! acc +6  | 6
//! ```
//!
//! After the last instruction (acc +6), the program terminates by attempting to
//! run the instruction below the last instruction in the file. With this
//! change, after the program terminates, the accumulator contains the value 8
//! (acc +1, acc +1, acc +6).
//!
//! Fix the program so that it terminates normally by changing exactly one jmp
//! (to nop) or nop (to jmp). What is the value of the accumulator after the
//! program terminates?
//!
//! [Advent of Code 2020 - Day 8](https://adventofcode.com/2020/day/8)

use std::collections::HashSet;
use std::iter::FromIterator;
use std::ops::Index;
use std::slice::Iter;
use std::str::FromStr;

pub type Address = usize;
pub type Data = i64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program(Vec<Instruction>);

impl Program {
    pub fn new(instructions: impl IntoIterator<Item = Instruction>) -> Self {
        Program(Vec::from_iter(instructions.into_iter()))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn instructions(&self) -> Iter<'_, Instruction> {
        self.0.iter()
    }
}

impl Index<usize> for Program {
    type Output = Instruction;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instruction {
    pub operation: OperationCode,
    pub argument: Data,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationCode {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperationCodeParseError {
    InvalidOperationCode(String),
}

impl FromStr for OperationCode {
    type Err = OperationCodeParseError;

    fn from_str(opcode: &str) -> Result<Self, Self::Err> {
        match opcode {
            "acc" => Ok(OperationCode::Acc),
            "jmp" => Ok(OperationCode::Jmp),
            "nop" => Ok(OperationCode::Nop),
            _ => Err(OperationCodeParseError::InvalidOperationCode(opcode.into())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Registers {
    pub pc: Address,
    pub acc: Data,
}

impl Default for Registers {
    fn default() -> Self {
        Self { pc: 0, acc: 0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Continuation {
    Continue,
    Halt,
}

#[derive(Debug)]
pub struct VirtualMachine {
    program: Program,
}

impl VirtualMachine {
    pub fn new(program: Program) -> Self {
        Self { program }
    }

    pub fn patch(&mut self, address: Address, opcode: OperationCode) {
        self.program.0[address].operation = opcode;
    }

    pub fn step(&self, reg: &mut Registers) -> Continuation {
        if reg.pc >= self.program.len() {
            return Continuation::Halt;
        }
        let instruction = self.program[reg.pc];
        match instruction.operation {
            OperationCode::Acc => {
                reg.acc += instruction.argument;
                reg.pc += 1;
            }
            OperationCode::Jmp => {
                if instruction.argument < 0 {
                    reg.pc -= instruction.argument.abs() as usize
                } else {
                    reg.pc += instruction.argument as usize
                }
            }
            OperationCode::Nop => reg.pc += 1,
        }
        Continuation::Continue
    }
}

#[aoc_generator(day8)]
pub fn parse_program(input: &str) -> Program {
    let mut instructions = Vec::with_capacity(input.len() / 8);
    for (lno, line) in input.lines().enumerate() {
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split(char::is_whitespace);
        let opcode = parts
            .next()
            .unwrap_or_else(|| panic!("line {}: no opcode in {:?}", lno, line));
        let operation = OperationCode::from_str(opcode).unwrap_or_else(|err| {
            panic!(
                "line {}: unrecognized opcode {:?}; reason: {:?}",
                lno, opcode, err
            )
        });
        let arg = parts
            .next()
            .unwrap_or_else(|| panic!("line {}: no argument in {:?}", lno, line));
        let argument = Data::from_str(arg).unwrap_or_else(|err| {
            panic!(
                "line {}: argument is not an integer {:?}; reason: {:?}",
                lno, arg, err
            )
        });
        instructions.push(Instruction {
            operation,
            argument,
        })
    }

    Program::new(instructions)
}

#[aoc(day8, part1)]
pub fn accumulator_value_before_second_run(program: &Program) -> Data {
    let vm = VirtualMachine::new(program.clone());
    let mut registers = Registers::default();

    let mut visited = HashSet::new();
    loop {
        if !visited.insert(registers.pc) {
            break;
        }
        match vm.step(&mut registers) {
            Continuation::Continue => {}
            Continuation::Halt => break,
        }
    }
    registers.acc
}

#[aoc(day8, part2)]
pub fn accumulator_value_after_fixing_the_endless_loop(program: &Program) -> Data {
    let mut vm = VirtualMachine::new(program.clone());
    let mut patchable_instructions = program
        .instructions()
        .enumerate()
        .filter_map(|(pc, inst)| match inst.operation {
            OperationCode::Jmp => Some((pc, OperationCode::Nop)),
            _ => None,
        })
        .collect::<Vec<_>>();
    while let Some(patched_instruction) = patchable_instructions.pop() {
        vm.patch(patched_instruction.0, patched_instruction.1);
        let mut registers = Registers::default();
        let mut visited = HashSet::new();
        let continuation = loop {
            if !visited.insert(registers.pc) {
                break Continuation::Continue;
            }
            match vm.step(&mut registers) {
                Continuation::Continue => {}
                Continuation::Halt => break Continuation::Halt,
            }
        };
        match continuation {
            Continuation::Continue => {}
            Continuation::Halt => return registers.acc,
        }
        vm.patch(patched_instruction.0, OperationCode::Jmp)
    }
    panic!("that is not an answer!");
}

#[cfg(test)]
mod tests;
