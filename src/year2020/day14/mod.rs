//! # Day 14: Docking Data
//!
//! As your ferry approaches the sea port, the captain asks for your help again.
//! The computer system that runs this port isn't compatible with the docking
//! program on the ferry, so the docking parameters aren't being correctly
//! initialized in the docking program's memory.
//!
//! After a brief inspection, you discover that the sea port's computer system
//! uses a strange bitmask system in its initialization program. Although you
//! don't have the correct decoder chip handy, you can emulate it in software!
//!
//! The initialization program (your puzzle input) can either update the bitmask
//! or write a value to memory. Values and memory addresses are both 36-bit
//! unsigned integers. For example, ignoring bitmasks for a moment, a line like
//! `mem[8] = 11` would write the value 11 to memory address 8.
//!
//! The bitmask is always given as a string of 36 bits, written with the most
//! significant bit (representing 2^35) on the left and the least significant
//! bit (2^0, that is, the 1s bit) on the right. The current bitmask is applied
//! to values immediately before they are written to memory: a 0 or 1 overwrites
//! the corresponding bit in the value, while an X leaves the bit in the value
//! unchanged.
//!
//! For example, consider the following program:
//!
//! ```text
//! mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
//! mem[8] = 11
//! mem[7] = 101
//! mem[8] = 0
//! ```
//!
//! This program starts by specifying a bitmask (mask = ....). The mask it
//! specifies will overwrite two bits in every written value: the 2s bit is
//! overwritten with 0, and the 64s bit is overwritten with 1.
//!
//! The program then attempts to write the value 11 to memory address 8. By
//! expanding everything out to individual bits, the mask is applied as follows:
//!
//! ```text
//! value:  000000000000000000000000000000001011  (decimal 11)
//! mask:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
//! result: 000000000000000000000000000001001001  (decimal 73)
//! ```
//!
//! So, because of the mask, the value 73 is written to memory address 8
//! instead. Then, the program tries to write 101 to address 7:
//!
//! ```text
//! value:  000000000000000000000000000001100101  (decimal 101)
//! mask:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
//! result: 000000000000000000000000000001100101  (decimal 101)
//! ```
//!
//! This time, the mask has no effect, as the bits it overwrote were already the
//! values the mask tried to set. Finally, the program tries to write 0 to
//! address 8:
//!
//! ```text
//! value:  000000000000000000000000000000000000  (decimal 0)
//! mask:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
//! result: 000000000000000000000000000001000000  (decimal 64)
//! ```
//!
//! 64 is written to address 8 instead, overwriting the value that was there
//! previously.
//!
//! To initialize your ferry's docking program, you need the sum of all values
//! left in memory after the initialization program completes. (The entire
//! 36-bit address space begins initialized to the value 0 at every address.)
//! In the above example, only two values in memory are not zero - 101 (at
//! address 7) and 64 (at address 8) - producing a sum of 165.
//!
//! Execute the initialization program. What is the sum of all values left in
//! memory after it completes?
//!
//! ## Part 2
//!
//! For some reason, the sea port's computer system still can't communicate with
//! your ferry's docking program. It must be using version 2 of the decoder
//! chip!
//!
//! A version 2 decoder chip doesn't modify the values being written at all.
//! Instead, it acts as a memory address decoder. Immediately before a value is
//! written to memory, each bit in the bitmask modifies the corresponding bit of
//! the destination memory address in the following way:
//!
//! * If the bitmask bit is 0, the corresponding memory address bit is
//!   unchanged.
//! * If the bitmask bit is 1, the corresponding memory address bit is
//!   overwritten with 1.
//! * If the bitmask bit is X, the corresponding memory address bit is floating.
//!
//! A floating bit is not connected to anything and instead fluctuates
//! unpredictably. In practice, this means the floating bits will take on all
//! possible values, potentially causing many memory addresses to be written all
//! at once!
//!
//! For example, consider the following program:
//!
//! ```text
//! mask = 000000000000000000000000000000X1001X
//! mem[42] = 100
//! mask = 00000000000000000000000000000000X0XX
//! mem[26] = 1
//! ```
//!
//! When this program goes to write to memory address 42, it first applies the
//! bitmask:
//!
//! ```text
//! address: 000000000000000000000000000000101010  (decimal 42)
//! mask:    000000000000000000000000000000X1001X
//! result:  000000000000000000000000000000X1101X
//! ```
//!
//! After applying the mask, four bits are overwritten, three of which are
//! different, and two of which are floating. Floating bits take on every
//! possible combination of values; with two floating bits, four actual memory
//! addresses are written:
//!
//! ```text
//! 000000000000000000000000000000011010  (decimal 26)
//! 000000000000000000000000000000011011  (decimal 27)
//! 000000000000000000000000000000111010  (decimal 58)
//! 000000000000000000000000000000111011  (decimal 59)
//! ```
//!
//! Next, the program is about to write to memory address 26 with a different
//! bitmask:
//!
//! ```text
//! address: 000000000000000000000000000000011010  (decimal 26)
//! mask:    00000000000000000000000000000000X0XX
//! result:  00000000000000000000000000000001X0XX
//! ```
//!
//! This results in an address with three floating bits, causing writes to eight
//! memory addresses:
//!
//! ```text
//! 000000000000000000000000000000010000  (decimal 16)
//! 000000000000000000000000000000010001  (decimal 17)
//! 000000000000000000000000000000010010  (decimal 18)
//! 000000000000000000000000000000010011  (decimal 19)
//! 000000000000000000000000000000011000  (decimal 24)
//! 000000000000000000000000000000011001  (decimal 25)
//! 000000000000000000000000000000011010  (decimal 26)
//! 000000000000000000000000000000011011  (decimal 27)
//! ```
//!
//! The entire 36-bit address space still begins initialized to the value 0 at
//! every address, and you still need the sum of all values left in memory at
//! the end of the program. In this example, the sum is 208.
//!
//! Execute the initialization program using an emulator for a version 2 decoder
//! chip. What is the sum of all values left in memory after it completes?
//!
//! [Advent of Code 2020 - Day 14](https://adventofcode.com/2020/day/14)

use std::collections::HashMap;
use std::str::FromStr;

pub type Address = u64;
pub type Data = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseMaskError {
    InvalidCharacter(char),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mask {
    clr: Address,
    set: Address,
}

impl Default for Mask {
    fn default() -> Self {
        Self {
            clr: 0x_FFFF_FFFF_FFFF_FFFF,
            set: 0x_0000_0000_0000_0000,
        }
    }
}

impl FromStr for Mask {
    type Err = ParseMaskError;

    fn from_str(mask_str: &str) -> Result<Self, ParseMaskError> {
        let mut clr = 0;
        let mut set = 0;
        for c in mask_str.chars() {
            match c {
                '0' => {
                    clr <<= 1;
                    set <<= 1;
                }
                '1' => {
                    clr = (clr << 1) | 1;
                    set = (set << 1) | 1;
                }
                'X' => {
                    clr = (clr << 1) | 1;
                    set = (set << 1) | 0;
                }
                _ => return Err(ParseMaskError::InvalidCharacter(c)),
            }
        }
        Ok(Mask { clr, set })
    }
}

impl Mask {
    fn apply(&self, data: Data) -> Data {
        data & self.clr | self.set
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    /// Set the mask specified by the pair of clear mask and set mask.
    Mask(String),
    /// Set value at memory address to the given value.
    Mem(Address, Data),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub instructions: Vec<Instruction>,
}

pub trait Interpreter {
    fn execute(
        &mut self,
        instr: &Instruction,
        memory: &mut HashMap<Address, Data>,
    ) -> (Address, Data);
}

#[derive(Debug)]
pub struct Machine<V> {
    interpreter: V,
    program: Program,
    ip: usize,
    memory: HashMap<Address, Data>,
}

impl<V> Machine<V> {
    pub fn memory(&self) -> &HashMap<Address, Data> {
        &self.memory
    }
}

impl<V> Machine<V>
where
    V: Default,
{
    pub fn new(program: Program) -> Self {
        Self {
            interpreter: V::default(),
            program,
            ip: 0,
            memory: HashMap::new(),
        }
    }
}

impl<V> Machine<V>
where
    V: Interpreter,
{
    pub fn run_program(&mut self) {
        let _ = self.last();
    }
}

impl<V> Iterator for Machine<V>
where
    V: Interpreter,
{
    type Item = (usize, Address, Data);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(instr) = self.program.instructions.get(self.ip) {
            let result = self.interpreter.execute(instr, &mut self.memory);
            self.ip += 1;
            Some((self.ip, result.0, result.1))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Address,
    Value,
    Mask,
}

#[aoc_generator(day14)]
pub fn parse_init_program(input: &str) -> Program {
    let mut instructions = Vec::new();
    let mut opcode = String::new();
    let mut mask = String::new();
    let mut address = String::new();
    let mut value = String::new();
    let mut token = Token::Value;

    for c in input.chars() {
        match c {
            'a'..='z' => opcode.push(c),
            '[' => token = Token::Address,
            ']' => {}
            '=' => match token {
                Token::Address => token = Token::Value,
                _ => token = Token::Mask,
            },
            'X' | '0' if token == Token::Mask => {
                mask.push(c);
            }
            '1' if token == Token::Mask => {
                mask.push(c);
            }
            '0'..='9' => match token {
                Token::Address => address.push(c),
                Token::Value => value.push(c),
                _ => panic!("unexpected digit at this position"),
            },
            '\n' => {
                match &opcode[..] {
                    "mask" => {
                        instructions.push(Instruction::Mask(mask.clone()));
                        mask.clear();
                    }
                    "mem" => {
                        let addr = u64::from_str(&address).unwrap();
                        let val = u64::from_str(&value).unwrap();
                        instructions.push(Instruction::Mem(addr, val));
                        address.clear();
                        value.clear();
                    }
                    _ => panic!("unrecognized opcode {:?}", opcode),
                }
                opcode.clear();
            }
            _ if c.is_whitespace() => {}
            _ => panic!("unrecognized character {:?}", c),
        }
    }

    Program { instructions }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct V1 {
    mask: Mask,
}

impl Default for V1 {
    fn default() -> Self {
        Self {
            mask: Mask::default(),
        }
    }
}

impl Interpreter for V1 {
    fn execute(
        &mut self,
        instr: &Instruction,
        memory: &mut HashMap<Address, Data>,
    ) -> (Address, Data) {
        match instr {
            Instruction::Mask(new_mask) => {
                self.mask = Mask::from_str(new_mask).unwrap();
                (0, 0)
            }
            Instruction::Mem(addr, val) => {
                let data = self.mask.apply(*val);
                memory.insert(*addr, data);
                (*addr, data)
            }
        }
    }
}

#[aoc(day14, part1)]
pub fn sum_of_all_values_in_memory_v1(init_program: &Program) -> u64 {
    let mut machine = Machine::<V1>::new(init_program.clone());
    machine.run_program();
    machine.memory().values().sum()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct V2 {
    masks: Vec<Mask>,
}

impl Default for V2 {
    fn default() -> Self {
        Self { masks: Vec::new() }
    }
}

impl Interpreter for V2 {
    fn execute(
        &mut self,
        instr: &Instruction,
        memory: &mut HashMap<Address, Data>,
    ) -> (Address, Data) {
        match instr {
            Instruction::Mask(new_mask) => {
                self.masks = build_mask_permutations(new_mask);
                (0, 0)
            }
            Instruction::Mem(addr, val) => {
                self.masks.iter().for_each(|mask| {
                    memory.insert(mask.apply(*addr), *val);
                });
                (*addr, *val)
            }
        }
    }
}

fn build_mask_permutations(mask_str: &str) -> Vec<Mask> {
    let x_count = mask_str.chars().filter(|c| *c == 'X').count();
    let num_permutations = 2_u64.pow(x_count as u32);
    let clr = u64::from_str_radix(
        &mask_str
            .chars()
            .map(|c| match c {
                'X' => '0',
                '0' => '1',
                '1' => '1',
                _ => c,
            })
            .collect::<String>(),
        2,
    )
    .unwrap();
    (0..num_permutations)
        .into_iter()
        .map(|num| {
            let perm_str = format!("{:038b}", num);
            let mut perm_iter = perm_str.chars().rev();
            let set_str = mask_str
                .chars()
                .map(|c| {
                    if c == 'X' {
                        perm_iter.next().unwrap()
                    } else {
                        c
                    }
                })
                .collect::<String>();
            let set = u64::from_str_radix(&set_str, 2).unwrap();
            Mask { clr, set }
        })
        .collect()
}

#[aoc(day14, part2)]
pub fn sum_of_all_values_in_memory_v2(init_program: &Program) -> u64 {
    let mut machine = Machine::<V2>::new(init_program.clone());
    machine.run_program();
    machine.memory().values().sum()
}

#[cfg(test)]
mod tests;
