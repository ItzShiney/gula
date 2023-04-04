mod instruction;

use crate::serde::*;
use crate::vm::VM;
pub use instruction::*;
use std::fmt;
use std::fmt::Debug;
use std::mem::size_of;

pub struct Instructions {
    bytes: Vec<u8>,
    pos: usize,
}

impl Debug for Instructions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "instructions:")?;

        let mut pos = self.pos;
        while pos < self.bytes.len() {
            let instruction: Instruction = self.bytes[pos..].deserialize();
            pos += instruction.serialized_len();
            writeln!(f, "    {:?}", instruction)?;
        }

        Ok(())
    }
}

impl From<Vec<Instruction>> for Instructions {
    fn from(instructions: Vec<Instruction>) -> Self {
        let mut bytes = Vec::default();

        for instruction_i in 0..instructions.len() {
            let mut instruction = instructions[instruction_i];

            use Instruction::*;

            match &mut instruction {
                Jump(instructions_offset)
                | JumpIf(instructions_offset)
                | JumpUnless(instructions_offset) => {
                    let skipped_instructions_i_range = match instructions_offset {
                        ..=0 => instruction_i - (-*instructions_offset as usize)..=instruction_i,

                        1.. => instruction_i + 1..=instruction_i + (*instructions_offset as usize),

                        _ => unreachable!(),
                    };

                    let mut offset = 0_isize;
                    for offseted_instruction_i in skipped_instructions_i_range {
                        let offseted_instruction = &instructions[offseted_instruction_i];
                        offset += offseted_instruction.serialized_len() as isize;
                    }

                    if *instructions_offset <= 0 {
                        offset = -offset;
                    }

                    *instructions_offset = offset;
                }

                _ => {}
            }

            let instruction_bytes = instruction.serialize();
            bytes.extend(instruction_bytes);
        }

        Self { bytes, pos: 0 }
    }
}

impl Instructions {
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        unsafe { &self.bytes.get_unchecked(self.pos..) }
    }

    #[inline]
    pub fn read<T: Serialize + Deserialize>(&mut self) -> T {
        let res: T = self.as_slice().deserialize();
        self.pos += res.serialized_len();
        res
    }

    #[inline]
    pub fn jump(&mut self, bytes_offset: isize) {
        self.pos = self.pos.overflowing_add_signed(bytes_offset).0;
    }

    #[inline]
    pub fn end(&mut self) {
        self.bytes.clear();
        self.pos = 0;
    }

    #[inline]
    pub fn eval(&mut self, vm: &mut VM) {
        while !self.as_slice().is_empty() {
            self.eval_next(vm);
        }
    }

    #[inline]
    pub fn eval_next(&mut self, vm: &mut VM) {
        let instruction_id: InstructionID = self.as_slice().deserialize();
        self.pos += size_of::<InstructionID>();

        Instruction::eval(instruction_id, self, vm);
    }
}
