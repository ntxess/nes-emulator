// Implementation of NES 6502 CPU
// https://www.nesdev.org/obelisk-6502-guide/reference.html
// http://www.6502.org/tutorials/6502opcodes.html
// https://www.nesdev.org/wiki/Nesdev_Wiki
// https://www.nesdev.org/NESDoc.pdf
// Following the RUST NES Guide by github:bugzmanov
// https://github.com/bugzmanov/nes_ebook

#![allow(dead_code)] 

use bitflags::bitflags;

bitflags! {
    struct StatusFlags: u8 {
        const CARRY     = 0b00000001;
        const ZERO      = 0b00000010;
        const INTERRUPT = 0b00000100;
        const DECIMAL   = 0b00001000;
        const BREAK     = 0b00010000;
        const UNUSED    = 0b00100000;
        const OVERFLOW  = 0b01000000;
        const NEGATIVE  = 0b10000000;
    }
}

pub struct Instruction {
    opcode:   Box<dyn Fn(&mut CPU) -> u8>,
    addrmode: Box<dyn Fn(&mut CPU) -> u8>,
    cycle:    u8,
}

pub struct InstructionSet {
    pub matrix: Vec<Instruction>
}

impl InstructionSet {
    pub fn new() -> Self {
        InstructionSet {
            matrix: vec![
                Instruction{opcode: Box::new(InstructionSet::brk), addrmode: Box::new(InstructionSet::imm), cycle: 7},
                Instruction{opcode: Box::new(InstructionSet::ora), addrmode: Box::new(InstructionSet::izx), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 8},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::ora), addrmode: Box::new(InstructionSet::zp0), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::asl), addrmode: Box::new(InstructionSet::zp0), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::php), addrmode: Box::new(InstructionSet::imp), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::ora), addrmode: Box::new(InstructionSet::imm), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::asl), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::ora), addrmode: Box::new(InstructionSet::abs), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::asl), addrmode: Box::new(InstructionSet::abs), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::bpl), addrmode: Box::new(InstructionSet::rel), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::ora), addrmode: Box::new(InstructionSet::izy), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 8},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::ora), addrmode: Box::new(InstructionSet::zpx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::asl), addrmode: Box::new(InstructionSet::zpx), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::clc), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::ora), addrmode: Box::new(InstructionSet::aby), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 7},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::ora), addrmode: Box::new(InstructionSet::abx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::asl), addrmode: Box::new(InstructionSet::abx), cycle: 7},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 7},
                Instruction{opcode: Box::new(InstructionSet::jsr), addrmode: Box::new(InstructionSet::abs), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::and), addrmode: Box::new(InstructionSet::izx), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 8},
                Instruction{opcode: Box::new(InstructionSet::bit), addrmode: Box::new(InstructionSet::zp0), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::and), addrmode: Box::new(InstructionSet::zp0), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::rol), addrmode: Box::new(InstructionSet::zp0), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::plp), addrmode: Box::new(InstructionSet::imp), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::and), addrmode: Box::new(InstructionSet::imm), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::rol), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::bit), addrmode: Box::new(InstructionSet::abs), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::and), addrmode: Box::new(InstructionSet::abs), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::rol), addrmode: Box::new(InstructionSet::abs), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::bmi), addrmode: Box::new(InstructionSet::rel), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::and), addrmode: Box::new(InstructionSet::izy), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 8},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::and), addrmode: Box::new(InstructionSet::zpx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::rol), addrmode: Box::new(InstructionSet::zpx), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::sec), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::and), addrmode: Box::new(InstructionSet::aby), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 7},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::and), addrmode: Box::new(InstructionSet::abx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::rol), addrmode: Box::new(InstructionSet::abx), cycle: 7},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 7},
                Instruction{opcode: Box::new(InstructionSet::rti), addrmode: Box::new(InstructionSet::imp), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::eor), addrmode: Box::new(InstructionSet::izx), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 8},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::eor), addrmode: Box::new(InstructionSet::zp0), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::lsr), addrmode: Box::new(InstructionSet::zp0), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::pha), addrmode: Box::new(InstructionSet::imp), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::eor), addrmode: Box::new(InstructionSet::imm), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::lsr), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::jmp), addrmode: Box::new(InstructionSet::abs), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::eor), addrmode: Box::new(InstructionSet::abs), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::lsr), addrmode: Box::new(InstructionSet::abs), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::bvc), addrmode: Box::new(InstructionSet::rel), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::eor), addrmode: Box::new(InstructionSet::izy), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 8},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::eor), addrmode: Box::new(InstructionSet::zpx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::lsr), addrmode: Box::new(InstructionSet::zpx), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::cli), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::eor), addrmode: Box::new(InstructionSet::aby), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 7},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::eor), addrmode: Box::new(InstructionSet::abx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::lsr), addrmode: Box::new(InstructionSet::abx), cycle: 7},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 7},
                Instruction{opcode: Box::new(InstructionSet::rts), addrmode: Box::new(InstructionSet::imp), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::adc), addrmode: Box::new(InstructionSet::izx), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 8},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::adc), addrmode: Box::new(InstructionSet::zp0), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::ror), addrmode: Box::new(InstructionSet::zp0), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::pla), addrmode: Box::new(InstructionSet::imp), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::adc), addrmode: Box::new(InstructionSet::imm), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::ror), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::jmp), addrmode: Box::new(InstructionSet::ind), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::adc), addrmode: Box::new(InstructionSet::abs), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::ror), addrmode: Box::new(InstructionSet::abs), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::bvs), addrmode: Box::new(InstructionSet::rel), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::adc), addrmode: Box::new(InstructionSet::izy), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 8},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::adc), addrmode: Box::new(InstructionSet::zpx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::ror), addrmode: Box::new(InstructionSet::zpx), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::sei), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::adc), addrmode: Box::new(InstructionSet::aby), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 7},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::adc), addrmode: Box::new(InstructionSet::abx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::ror), addrmode: Box::new(InstructionSet::abx), cycle: 7},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 7},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::sta), addrmode: Box::new(InstructionSet::izx), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::sty), addrmode: Box::new(InstructionSet::zp0), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::sta), addrmode: Box::new(InstructionSet::zp0), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::stx), addrmode: Box::new(InstructionSet::zp0), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::dey), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::txa), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::sty), addrmode: Box::new(InstructionSet::abs), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::sta), addrmode: Box::new(InstructionSet::abs), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::stx), addrmode: Box::new(InstructionSet::abs), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::bcc), addrmode: Box::new(InstructionSet::rel), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::sta), addrmode: Box::new(InstructionSet::izy), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::sty), addrmode: Box::new(InstructionSet::zpx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::sta), addrmode: Box::new(InstructionSet::zpx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::stx), addrmode: Box::new(InstructionSet::zpy), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::tya), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::sta), addrmode: Box::new(InstructionSet::aby), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::txs), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::sta), addrmode: Box::new(InstructionSet::abx), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::ldy), addrmode: Box::new(InstructionSet::imm), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::lda), addrmode: Box::new(InstructionSet::izx), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::ldx), addrmode: Box::new(InstructionSet::imm), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::ldy), addrmode: Box::new(InstructionSet::zp0), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::lda), addrmode: Box::new(InstructionSet::zp0), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::ldx), addrmode: Box::new(InstructionSet::zp0), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::tay), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::lda), addrmode: Box::new(InstructionSet::imm), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::tax), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::ldy), addrmode: Box::new(InstructionSet::abs), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::lda), addrmode: Box::new(InstructionSet::abs), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::ldx), addrmode: Box::new(InstructionSet::abs), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::bcs), addrmode: Box::new(InstructionSet::rel), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::lda), addrmode: Box::new(InstructionSet::izy), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::ldy), addrmode: Box::new(InstructionSet::zpx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::lda), addrmode: Box::new(InstructionSet::zpx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::ldx), addrmode: Box::new(InstructionSet::zpy), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::clv), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::lda), addrmode: Box::new(InstructionSet::aby), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::tsx), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::ldy), addrmode: Box::new(InstructionSet::abx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::lda), addrmode: Box::new(InstructionSet::abx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::ldx), addrmode: Box::new(InstructionSet::aby), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::cpy), addrmode: Box::new(InstructionSet::imm), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::cmp), addrmode: Box::new(InstructionSet::izx), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 8},
                Instruction{opcode: Box::new(InstructionSet::cpy), addrmode: Box::new(InstructionSet::zp0), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::cmp), addrmode: Box::new(InstructionSet::zp0), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::dec), addrmode: Box::new(InstructionSet::zp0), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::iny), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::cmp), addrmode: Box::new(InstructionSet::imm), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::dex), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::cpy), addrmode: Box::new(InstructionSet::abs), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::cmp), addrmode: Box::new(InstructionSet::abs), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::dec), addrmode: Box::new(InstructionSet::abs), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::bne), addrmode: Box::new(InstructionSet::rel), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::cmp), addrmode: Box::new(InstructionSet::izy), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 8},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::cmp), addrmode: Box::new(InstructionSet::zpx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::dec), addrmode: Box::new(InstructionSet::zpx), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::cld), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::cmp), addrmode: Box::new(InstructionSet::aby), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 7},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::cmp), addrmode: Box::new(InstructionSet::abx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::dec), addrmode: Box::new(InstructionSet::abx), cycle: 7},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 7},
                Instruction{opcode: Box::new(InstructionSet::cpx), addrmode: Box::new(InstructionSet::imm), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::sbc), addrmode: Box::new(InstructionSet::izx), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 8},
                Instruction{opcode: Box::new(InstructionSet::cpx), addrmode: Box::new(InstructionSet::zp0), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::sbc), addrmode: Box::new(InstructionSet::zp0), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::inc), addrmode: Box::new(InstructionSet::zp0), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::inx), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::sbc), addrmode: Box::new(InstructionSet::imm), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::sbc), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::cpx), addrmode: Box::new(InstructionSet::abs), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::sbc), addrmode: Box::new(InstructionSet::abs), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::inc), addrmode: Box::new(InstructionSet::abs), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::beq), addrmode: Box::new(InstructionSet::rel), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::sbc), addrmode: Box::new(InstructionSet::izy), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 8},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::sbc), addrmode: Box::new(InstructionSet::zpx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::inc), addrmode: Box::new(InstructionSet::zpx), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::sed), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::sbc), addrmode: Box::new(InstructionSet::aby), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 7},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::sbc), addrmode: Box::new(InstructionSet::abx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::inc), addrmode: Box::new(InstructionSet::abx), cycle: 7},
                Instruction{opcode: Box::new(InstructionSet::xxx), addrmode: Box::new(InstructionSet::imp), cycle: 7},
            ],
        }
    }

    pub fn get_opcode(&mut self, code: usize) -> &Box<dyn Fn(&mut CPU) -> u8> {
        return &self.matrix[code].opcode;
    }

    pub fn get_addrmode(&mut self, code: usize) -> &Box<dyn Fn(&mut CPU) -> u8> {
        return &self.matrix[code].addrmode;
    }

    pub fn get_cycle(&mut self, code: usize) -> u8 {
        return self.matrix[code].cycle;
    }

    // Addressing Modes
    
    // Addressing Mode: Implicit
    // For many 6502 instructions the source and destination of the information to be 
    // manipulated is implied directly by the function of the instruction itself and no 
    // further operand needs to be specified. Operations like 'Clear Carry Flag' (CLC) and 
    //'Return from Subroutine' (RTS) are implicit.
    fn imp(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Addressing Mode: Immediate
    // Immediate addressing allows the programmer to directly specify an 8 bit constant 
    // within the instruction. It is indicated by a '#' symbol followed by an numeric expression.
    fn imm(_cpu : &mut CPU) -> u8 {
        println!("successful call to correct address mode!");
        return 0;
        // return 0;
    }

    // Addressing Mode: Zero Page
    // An instruction using zero page addressing mode has only an 8 bit address operand. 
    // This limits it to addressing only the first 256 bytes of memory (e.g. $0000 to $00FF) 
    // where the most significant byte of the address is always zero. In zero page mode only 
    // the least significant byte of the address is held in the instruction making it shorter 
    // by one byte (important for space saving) and one less memory fetch during execution 
    // (important for speed).
	fn zp0(_cpu : &mut CPU) -> u8 {
        return 0;
    }		

    // Addressing Mode: Zero Page, X
    // The address to be accessed by an instruction using indexed zero page addressing is 
    // calculated by taking the 8 bit zero page address from the instruction and adding the 
    // current value of the X register to it.
    fn zpx(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Addressing Mode: Zero Page, Y
    // The address to be accessed by an instruction using indexed zero page addressing is 
    // calculated by taking the 8 bit zero page address from the instruction and adding the 
    // current value of the Y register to it. This mode can only be used with the LDX and STX 
    // instructions.
	fn zpy(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Addressing Mode: Relative
    // Relative addressing mode is used by branch instructions (e.g. BEQ, BNE, etc.) which 
    // contain a signed 8 bit relative offset (e.g. -128 to +127) which is added to program 
    // counter if the condition is true. As the program counter itself is incremented during 
    // instruction execution by two the effective address range for the target instruction 
    // must be with -126 to +129 bytes of the branch.
    fn rel(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Addressing Mode: Absolute
    // Instructions using absolute addressing contain a full 16 bit address to identify the 
    // target location. 
	fn abs(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Addressing Mode: Absolute, Y
    // The address to be accessed by an instruction using X register indexed absolute addressing 
    // is computed by taking the 16 bit address from the instruction and added the contents of 
    // the X register. For example if X contains $92 then an STA $2000,X instruction will store 
    // the accumulator at $2092 (e.g. $2000 + $92).
    fn abx(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Addressing Mode: Absolute, X
    // The Y register indexed absolute addressing mode is the same as the previous mode only with 
    // the contents of the Y register added to the 16 bit address from the instruction.
	fn aby(_cpu : &mut CPU) -> u8 {
        return 0;
    }	

    // Addressing Mode: Indirect
    // JMP is the only 6502 instruction to support indirection. The instruction contains a 16 bit 
    // address which identifies the location of the least significant byte of another 16 bit memory 
    // address which is the real target of the instruction.
    fn ind(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Addressing Mode: Index Indirect 
    // Indexed indirect addressing is normally used in conjunction with a table of address held on 
    // zero page. The address of the table is taken from the instruction and the X register added to 
    // it (with zero page wrap around) to give the location of the least significant byte of the 
    // target address.
    fn izx(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Addressing Mode: Indirect Indexed
    // Indirect indirect addressing is the most common indirection mode used on the 6502. In 
    // instruction contains the zero page location of the least significant byte of 16 bit address. 
    // The Y register is dynamically added to this value to generated the actual target address for 
    // operation.
    fn izy(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instructions
    
    // Instruction: Add with Carry
    fn adc(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Logic AND
    fn and(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Arithmetic Shift Left
    fn asl(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Branch if Carry Clear
    fn bcc(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction:  Branch if Carrt Set
	fn bcs(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Branch if Equal
    fn beq(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Bit Test
    fn bit(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Branch if Minus
    fn bmi(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Branch if Not Equal
	fn bne(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Branch if Positive
    fn bpl(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Force Interrupt
    fn brk(_cpu : &mut CPU) -> u8 {
        println!("reg_status changed!");
        _cpu.reg_status = 0b10;
        return 0;
        // return 0;
    }

    // Instruction: Branch if Overflow Clear
    fn bvc(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Branch Carry Flag
	fn bvs(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Clear Carry Flag
    fn clc(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Clear Decimal Mode
    fn cld(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Clear Interrupt Disable
    fn cli(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction:  Clear Overflow Flag
	fn clv(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Compare
    fn cmp(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Compare X Register
    fn cpx(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Compare Y Register
    fn cpy(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Decrement Memory
	fn dec(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Decrement X Register
    fn dex(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Decrement Y Register
    fn dey(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Exclusive OR
    fn eor(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Increment Memory
	fn inc(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Increment X Register
    fn inx(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Increment Y Register
    fn iny(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Jump
    fn jmp(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Jump to Subroutine
	fn jsr(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Load Accumulator
    fn lda(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Load X Register
    fn ldx(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Load Y Register
    fn ldy(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Logical Shift Right
	fn lsr(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: No Operation
    fn nop(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Logical Inclusive OR
    fn ora(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Push Accumulator
    fn pha(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Push Processor Status
	fn php(_cpu : &mut CPU) -> u8 {
        return 0;
    }
    
    // Instruction: Pull Accumulator
    fn pla(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Pull Processor Status
    fn plp(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Rotate Left
    fn rol(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Rotate Right
	fn ror(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Return from Interrupt
    fn rti(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Return from Subroutine
    fn rts(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Subtract with Carry
    fn sbc(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Set Carry Flag
	fn sec(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Set Decimal Flag
    fn sed(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Set Interrupt Disable
    fn sei(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Store Accumulator
    fn sta(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Store X Register
	fn stx(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Store Y Register
    fn sty(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Transfer Accumulator to X
    fn tax(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Transfer Accumulator to Y
    fn tay(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Transfer Stack Pointer to X
	fn tsx(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Transfer X to Accumulator
    fn txa(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Transfer X to Stack Pointer
    fn txs(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    // Instruction: Transfer Y to Accumulator
    fn tya(_cpu : &mut CPU) -> u8 {
        return 0;
    }

    fn xxx(_cpu : &mut CPU) -> u8 {
        return 0;
    }
}

pub struct CPU {
    pub reg_pc:        u16,
    pub reg_stack_ptr: u8,
    pub reg_acc:       u8,
    pub reg_x:         u8,
    pub reg_y:         u8,
    pub reg_status:    u8,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            reg_pc:        0x0000,
            reg_stack_ptr: 0x00,
            reg_acc:       0x00,
            reg_x:         0x00,
            reg_y:         0x00,
            reg_status:    0x00,
        }
    }

    // Auxiliary Function
    pub fn interpret(&mut self, matrix: &mut InstructionSet, program: Vec<u8>) {
        self.reg_pc = 0;

        matrix.get_opcode(program[self.reg_pc as usize] as usize)(self);
        matrix.get_addrmode(program[self.reg_pc as usize] as usize)(self);
        matrix.get_cycle(program[self.reg_pc as usize] as usize);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
 
    #[test]
    fn test_0xa9_lda_immidiate_load_data() {
        let mut cpu = CPU::new();
        let mut matrix = InstructionSet::new();

        cpu.interpret(&mut matrix, vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.reg_acc, 0x05);
        assert!(cpu.reg_status & 0b0000_0010 == 0b00);
        assert!(cpu.reg_status & 0b1000_0000 == 0);
    }
 
     #[test]
     fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        let mut matrix = InstructionSet::new();

         cpu.interpret(&mut matrix, vec![0xa9, 0x00, 0x00]);
         assert!(cpu.reg_status & 0b0000_0010 == 0b10);
     }

     #[test]
     fn test_brk() {
        let mut cpu = CPU::new();
        let mut matrix = InstructionSet::new();

        cpu.interpret(&mut matrix, vec![0x00]);
        assert!(cpu.reg_status & 0b0000_0010 == 0b10);
    }
}