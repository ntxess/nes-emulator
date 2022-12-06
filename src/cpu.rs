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

pub struct CPU {
    pub reg_pc:        u16,
    pub reg_stack_ptr: u8,
    pub reg_acc:       u8,
    pub reg_x:         u8,
    pub reg_y:         u8,
    pub reg_status:    u8,
    pub opcode_matrix: Vec<Instruction>,
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
            opcode_matrix: vec![
                Instruction{opcode: Box::new(CPU::brk), addrmode: Box::new(CPU::imm), cycle: 7},
                Instruction{opcode: Box::new(CPU::ora), addrmode: Box::new(CPU::izx), cycle: 6},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 8},
                Instruction{opcode: Box::new(CPU::nop), addrmode: Box::new(CPU::imp), cycle: 3},
                Instruction{opcode: Box::new(CPU::ora), addrmode: Box::new(CPU::zp0), cycle: 3},
                Instruction{opcode: Box::new(CPU::asl), addrmode: Box::new(CPU::zp0), cycle: 5},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 5},
                Instruction{opcode: Box::new(CPU::php), addrmode: Box::new(CPU::imp), cycle: 3},
                Instruction{opcode: Box::new(CPU::ora), addrmode: Box::new(CPU::imm), cycle: 2},
                Instruction{opcode: Box::new(CPU::asl), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::nop), addrmode: Box::new(CPU::imp), cycle: 4},
                Instruction{opcode: Box::new(CPU::ora), addrmode: Box::new(CPU::abs), cycle: 4},
                Instruction{opcode: Box::new(CPU::asl), addrmode: Box::new(CPU::abs), cycle: 6},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 6},
                Instruction{opcode: Box::new(CPU::bpl), addrmode: Box::new(CPU::rel), cycle: 2},
                Instruction{opcode: Box::new(CPU::ora), addrmode: Box::new(CPU::izy), cycle: 5},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 8},
                Instruction{opcode: Box::new(CPU::nop), addrmode: Box::new(CPU::imp), cycle: 4},
                Instruction{opcode: Box::new(CPU::ora), addrmode: Box::new(CPU::zpx), cycle: 4},
                Instruction{opcode: Box::new(CPU::asl), addrmode: Box::new(CPU::zpx), cycle: 6},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 6},
                Instruction{opcode: Box::new(CPU::clc), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::ora), addrmode: Box::new(CPU::aby), cycle: 4},
                Instruction{opcode: Box::new(CPU::nop), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 7},
                Instruction{opcode: Box::new(CPU::nop), addrmode: Box::new(CPU::imp), cycle: 4},
                Instruction{opcode: Box::new(CPU::ora), addrmode: Box::new(CPU::abx), cycle: 4},
                Instruction{opcode: Box::new(CPU::asl), addrmode: Box::new(CPU::abx), cycle: 7},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 7},
                Instruction{opcode: Box::new(CPU::jsr), addrmode: Box::new(CPU::abs), cycle: 6},
                Instruction{opcode: Box::new(CPU::and), addrmode: Box::new(CPU::izx), cycle: 6},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 8},
                Instruction{opcode: Box::new(CPU::bit), addrmode: Box::new(CPU::zp0), cycle: 3},
                Instruction{opcode: Box::new(CPU::and), addrmode: Box::new(CPU::zp0), cycle: 3},
                Instruction{opcode: Box::new(CPU::rol), addrmode: Box::new(CPU::zp0), cycle: 5},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 5},
                Instruction{opcode: Box::new(CPU::plp), addrmode: Box::new(CPU::imp), cycle: 4},
                Instruction{opcode: Box::new(CPU::and), addrmode: Box::new(CPU::imm), cycle: 2},
                Instruction{opcode: Box::new(CPU::rol), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::bit), addrmode: Box::new(CPU::abs), cycle: 4},
                Instruction{opcode: Box::new(CPU::and), addrmode: Box::new(CPU::abs), cycle: 4},
                Instruction{opcode: Box::new(CPU::rol), addrmode: Box::new(CPU::abs), cycle: 6},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 6},
                Instruction{opcode: Box::new(CPU::bmi), addrmode: Box::new(CPU::rel), cycle: 2},
                Instruction{opcode: Box::new(CPU::and), addrmode: Box::new(CPU::izy), cycle: 5},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 8},
                Instruction{opcode: Box::new(CPU::nop), addrmode: Box::new(CPU::imp), cycle: 4},
                Instruction{opcode: Box::new(CPU::and), addrmode: Box::new(CPU::zpx), cycle: 4},
                Instruction{opcode: Box::new(CPU::rol), addrmode: Box::new(CPU::zpx), cycle: 6},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 6},
                Instruction{opcode: Box::new(CPU::sec), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::and), addrmode: Box::new(CPU::aby), cycle: 4},
                Instruction{opcode: Box::new(CPU::nop), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 7},
                Instruction{opcode: Box::new(CPU::nop), addrmode: Box::new(CPU::imp), cycle: 4},
                Instruction{opcode: Box::new(CPU::and), addrmode: Box::new(CPU::abx), cycle: 4},
                Instruction{opcode: Box::new(CPU::rol), addrmode: Box::new(CPU::abx), cycle: 7},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 7},
                Instruction{opcode: Box::new(CPU::rti), addrmode: Box::new(CPU::imp), cycle: 6},
                Instruction{opcode: Box::new(CPU::eor), addrmode: Box::new(CPU::izx), cycle: 6},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 8},
                Instruction{opcode: Box::new(CPU::nop), addrmode: Box::new(CPU::imp), cycle: 3},
                Instruction{opcode: Box::new(CPU::eor), addrmode: Box::new(CPU::zp0), cycle: 3},
                Instruction{opcode: Box::new(CPU::lsr), addrmode: Box::new(CPU::zp0), cycle: 5},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 5},
                Instruction{opcode: Box::new(CPU::pha), addrmode: Box::new(CPU::imp), cycle: 3},
                Instruction{opcode: Box::new(CPU::eor), addrmode: Box::new(CPU::imm), cycle: 2},
                Instruction{opcode: Box::new(CPU::lsr), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::jmp), addrmode: Box::new(CPU::abs), cycle: 3},
                Instruction{opcode: Box::new(CPU::eor), addrmode: Box::new(CPU::abs), cycle: 4},
                Instruction{opcode: Box::new(CPU::lsr), addrmode: Box::new(CPU::abs), cycle: 6},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 6},
                Instruction{opcode: Box::new(CPU::bvc), addrmode: Box::new(CPU::rel), cycle: 2},
                Instruction{opcode: Box::new(CPU::eor), addrmode: Box::new(CPU::izy), cycle: 5},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 8},
                Instruction{opcode: Box::new(CPU::nop), addrmode: Box::new(CPU::imp), cycle: 4},
                Instruction{opcode: Box::new(CPU::eor), addrmode: Box::new(CPU::zpx), cycle: 4},
                Instruction{opcode: Box::new(CPU::lsr), addrmode: Box::new(CPU::zpx), cycle: 6},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 6},
                Instruction{opcode: Box::new(CPU::cli), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::eor), addrmode: Box::new(CPU::aby), cycle: 4},
                Instruction{opcode: Box::new(CPU::nop), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 7},
                Instruction{opcode: Box::new(CPU::nop), addrmode: Box::new(CPU::imp), cycle: 4},
                Instruction{opcode: Box::new(CPU::eor), addrmode: Box::new(CPU::abx), cycle: 4},
                Instruction{opcode: Box::new(CPU::lsr), addrmode: Box::new(CPU::abx), cycle: 7},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 7},
                Instruction{opcode: Box::new(CPU::rts), addrmode: Box::new(CPU::imp), cycle: 6},
                Instruction{opcode: Box::new(CPU::adc), addrmode: Box::new(CPU::izx), cycle: 6},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 8},
                Instruction{opcode: Box::new(CPU::nop), addrmode: Box::new(CPU::imp), cycle: 3},
                Instruction{opcode: Box::new(CPU::adc), addrmode: Box::new(CPU::zp0), cycle: 3},
                Instruction{opcode: Box::new(CPU::ror), addrmode: Box::new(CPU::zp0), cycle: 5},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 5},
                Instruction{opcode: Box::new(CPU::pla), addrmode: Box::new(CPU::imp), cycle: 4},
                Instruction{opcode: Box::new(CPU::adc), addrmode: Box::new(CPU::imm), cycle: 2},
                Instruction{opcode: Box::new(CPU::ror), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::jmp), addrmode: Box::new(CPU::ind), cycle: 5},
                Instruction{opcode: Box::new(CPU::adc), addrmode: Box::new(CPU::abs), cycle: 4},
                Instruction{opcode: Box::new(CPU::ror), addrmode: Box::new(CPU::abs), cycle: 6},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 6},
                Instruction{opcode: Box::new(CPU::bvs), addrmode: Box::new(CPU::rel), cycle: 2},
                Instruction{opcode: Box::new(CPU::adc), addrmode: Box::new(CPU::izy), cycle: 5},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 8},
                Instruction{opcode: Box::new(CPU::nop), addrmode: Box::new(CPU::imp), cycle: 4},
                Instruction{opcode: Box::new(CPU::adc), addrmode: Box::new(CPU::zpx), cycle: 4},
                Instruction{opcode: Box::new(CPU::ror), addrmode: Box::new(CPU::zpx), cycle: 6},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 6},
                Instruction{opcode: Box::new(CPU::sei), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::adc), addrmode: Box::new(CPU::aby), cycle: 4},
                Instruction{opcode: Box::new(CPU::nop), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 7},
                Instruction{opcode: Box::new(CPU::nop), addrmode: Box::new(CPU::imp), cycle: 4},
                Instruction{opcode: Box::new(CPU::adc), addrmode: Box::new(CPU::abx), cycle: 4},
                Instruction{opcode: Box::new(CPU::ror), addrmode: Box::new(CPU::abx), cycle: 7},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 7},
                Instruction{opcode: Box::new(CPU::nop), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::sta), addrmode: Box::new(CPU::izx), cycle: 6},
                Instruction{opcode: Box::new(CPU::nop), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 6},
                Instruction{opcode: Box::new(CPU::sty), addrmode: Box::new(CPU::zp0), cycle: 3},
                Instruction{opcode: Box::new(CPU::sta), addrmode: Box::new(CPU::zp0), cycle: 3},
                Instruction{opcode: Box::new(CPU::stx), addrmode: Box::new(CPU::zp0), cycle: 3},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 3},
                Instruction{opcode: Box::new(CPU::dey), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::nop), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::txa), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::sty), addrmode: Box::new(CPU::abs), cycle: 4},
                Instruction{opcode: Box::new(CPU::sta), addrmode: Box::new(CPU::abs), cycle: 4},
                Instruction{opcode: Box::new(CPU::stx), addrmode: Box::new(CPU::abs), cycle: 4},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 4},
                Instruction{opcode: Box::new(CPU::bcc), addrmode: Box::new(CPU::rel), cycle: 2},
                Instruction{opcode: Box::new(CPU::sta), addrmode: Box::new(CPU::izy), cycle: 6},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 6},
                Instruction{opcode: Box::new(CPU::sty), addrmode: Box::new(CPU::zpx), cycle: 4},
                Instruction{opcode: Box::new(CPU::sta), addrmode: Box::new(CPU::zpx), cycle: 4},
                Instruction{opcode: Box::new(CPU::stx), addrmode: Box::new(CPU::zpy), cycle: 4},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 4},
                Instruction{opcode: Box::new(CPU::tya), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::sta), addrmode: Box::new(CPU::aby), cycle: 5},
                Instruction{opcode: Box::new(CPU::txs), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 5},
                Instruction{opcode: Box::new(CPU::nop), addrmode: Box::new(CPU::imp), cycle: 5},
                Instruction{opcode: Box::new(CPU::sta), addrmode: Box::new(CPU::abx), cycle: 5},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 5},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 5},
                Instruction{opcode: Box::new(CPU::ldy), addrmode: Box::new(CPU::imm), cycle: 2},
                Instruction{opcode: Box::new(CPU::lda), addrmode: Box::new(CPU::izx), cycle: 6},
                Instruction{opcode: Box::new(CPU::ldx), addrmode: Box::new(CPU::imm), cycle: 2},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 6},
                Instruction{opcode: Box::new(CPU::ldy), addrmode: Box::new(CPU::zp0), cycle: 3},
                Instruction{opcode: Box::new(CPU::lda), addrmode: Box::new(CPU::zp0), cycle: 3},
                Instruction{opcode: Box::new(CPU::ldx), addrmode: Box::new(CPU::zp0), cycle: 3},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 3},
                Instruction{opcode: Box::new(CPU::tay), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::lda), addrmode: Box::new(CPU::imm), cycle: 2},
                Instruction{opcode: Box::new(CPU::tax), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::ldy), addrmode: Box::new(CPU::abs), cycle: 4},
                Instruction{opcode: Box::new(CPU::lda), addrmode: Box::new(CPU::abs), cycle: 4},
                Instruction{opcode: Box::new(CPU::ldx), addrmode: Box::new(CPU::abs), cycle: 4},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 4},
                Instruction{opcode: Box::new(CPU::bcs), addrmode: Box::new(CPU::rel), cycle: 2},
                Instruction{opcode: Box::new(CPU::lda), addrmode: Box::new(CPU::izy), cycle: 5},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 5},
                Instruction{opcode: Box::new(CPU::ldy), addrmode: Box::new(CPU::zpx), cycle: 4},
                Instruction{opcode: Box::new(CPU::lda), addrmode: Box::new(CPU::zpx), cycle: 4},
                Instruction{opcode: Box::new(CPU::ldx), addrmode: Box::new(CPU::zpy), cycle: 4},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 4},
                Instruction{opcode: Box::new(CPU::clv), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::lda), addrmode: Box::new(CPU::aby), cycle: 4},
                Instruction{opcode: Box::new(CPU::tsx), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 4},
                Instruction{opcode: Box::new(CPU::ldy), addrmode: Box::new(CPU::abx), cycle: 4},
                Instruction{opcode: Box::new(CPU::lda), addrmode: Box::new(CPU::abx), cycle: 4},
                Instruction{opcode: Box::new(CPU::ldx), addrmode: Box::new(CPU::aby), cycle: 4},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 4},
                Instruction{opcode: Box::new(CPU::cpy), addrmode: Box::new(CPU::imm), cycle: 2},
                Instruction{opcode: Box::new(CPU::cmp), addrmode: Box::new(CPU::izx), cycle: 6},
                Instruction{opcode: Box::new(CPU::nop), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 8},
                Instruction{opcode: Box::new(CPU::cpy), addrmode: Box::new(CPU::zp0), cycle: 3},
                Instruction{opcode: Box::new(CPU::cmp), addrmode: Box::new(CPU::zp0), cycle: 3},
                Instruction{opcode: Box::new(CPU::dec), addrmode: Box::new(CPU::zp0), cycle: 5},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 5},
                Instruction{opcode: Box::new(CPU::iny), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::cmp), addrmode: Box::new(CPU::imm), cycle: 2},
                Instruction{opcode: Box::new(CPU::dex), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::cpy), addrmode: Box::new(CPU::abs), cycle: 4},
                Instruction{opcode: Box::new(CPU::cmp), addrmode: Box::new(CPU::abs), cycle: 4},
                Instruction{opcode: Box::new(CPU::dec), addrmode: Box::new(CPU::abs), cycle: 6},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 6},
                Instruction{opcode: Box::new(CPU::bne), addrmode: Box::new(CPU::rel), cycle: 2},
                Instruction{opcode: Box::new(CPU::cmp), addrmode: Box::new(CPU::izy), cycle: 5},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 8},
                Instruction{opcode: Box::new(CPU::nop), addrmode: Box::new(CPU::imp), cycle: 4},
                Instruction{opcode: Box::new(CPU::cmp), addrmode: Box::new(CPU::zpx), cycle: 4},
                Instruction{opcode: Box::new(CPU::dec), addrmode: Box::new(CPU::zpx), cycle: 6},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 6},
                Instruction{opcode: Box::new(CPU::cld), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::cmp), addrmode: Box::new(CPU::aby), cycle: 4},
                Instruction{opcode: Box::new(CPU::nop), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 7},
                Instruction{opcode: Box::new(CPU::nop), addrmode: Box::new(CPU::imp), cycle: 4},
                Instruction{opcode: Box::new(CPU::cmp), addrmode: Box::new(CPU::abx), cycle: 4},
                Instruction{opcode: Box::new(CPU::dec), addrmode: Box::new(CPU::abx), cycle: 7},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 7},
                Instruction{opcode: Box::new(CPU::cpx), addrmode: Box::new(CPU::imm), cycle: 2},
                Instruction{opcode: Box::new(CPU::sbc), addrmode: Box::new(CPU::izx), cycle: 6},
                Instruction{opcode: Box::new(CPU::nop), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 8},
                Instruction{opcode: Box::new(CPU::cpx), addrmode: Box::new(CPU::zp0), cycle: 3},
                Instruction{opcode: Box::new(CPU::sbc), addrmode: Box::new(CPU::zp0), cycle: 3},
                Instruction{opcode: Box::new(CPU::inc), addrmode: Box::new(CPU::zp0), cycle: 5},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 5},
                Instruction{opcode: Box::new(CPU::inx), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::sbc), addrmode: Box::new(CPU::imm), cycle: 2},
                Instruction{opcode: Box::new(CPU::nop), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::sbc), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::cpx), addrmode: Box::new(CPU::abs), cycle: 4},
                Instruction{opcode: Box::new(CPU::sbc), addrmode: Box::new(CPU::abs), cycle: 4},
                Instruction{opcode: Box::new(CPU::inc), addrmode: Box::new(CPU::abs), cycle: 6},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 6},
                Instruction{opcode: Box::new(CPU::beq), addrmode: Box::new(CPU::rel), cycle: 2},
                Instruction{opcode: Box::new(CPU::sbc), addrmode: Box::new(CPU::izy), cycle: 5},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 8},
                Instruction{opcode: Box::new(CPU::nop), addrmode: Box::new(CPU::imp), cycle: 4},
                Instruction{opcode: Box::new(CPU::sbc), addrmode: Box::new(CPU::zpx), cycle: 4},
                Instruction{opcode: Box::new(CPU::inc), addrmode: Box::new(CPU::zpx), cycle: 6},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 6},
                Instruction{opcode: Box::new(CPU::sed), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::sbc), addrmode: Box::new(CPU::aby), cycle: 4},
                Instruction{opcode: Box::new(CPU::nop), addrmode: Box::new(CPU::imp), cycle: 2},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 7},
                Instruction{opcode: Box::new(CPU::nop), addrmode: Box::new(CPU::imp), cycle: 4},
                Instruction{opcode: Box::new(CPU::sbc), addrmode: Box::new(CPU::abx), cycle: 4},
                Instruction{opcode: Box::new(CPU::inc), addrmode: Box::new(CPU::abx), cycle: 7},
                Instruction{opcode: Box::new(CPU::xxx), addrmode: Box::new(CPU::imp), cycle: 7},
                ],
        }
    }

    // Auxiliary Function

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.reg_pc = 0;

        loop {
            let opscode = program[self.reg_pc as usize];
            self.reg_pc += 1;
    
            match opscode {
                0xA9 => {
                    let param = program[self.reg_pc as usize];
                    self.reg_pc +=1;
                    self.reg_acc = param;
    
                    if self.reg_acc == 0 {
                        self.reg_status = self.reg_status | 0b0000_0010;
                    } else {
                        self.reg_status = self.reg_status & 0b1111_1101;
                    }
    
                    if self.reg_acc & 0b1000_0000 != 0 {
                        self.reg_status = self.reg_status | 0b1000_0000;
                    } else {
                        self.reg_status = self.reg_status & 0b0111_1111;
                    }
                }
                0xAA =>  {
                    self.reg_x = self.reg_acc;
                
                    if self.reg_x == 0 {
                        self.reg_status = self.reg_status | 0b0000_0010;
                    } else {
                        self.reg_status = self.reg_status & 0b1111_1101;
                    }
    
                    if self.reg_x & 0b1000_0000 != 0 {
                        self.reg_status = self.reg_status | 0b1000_0000;
                    } else {
                        self.reg_status = self.reg_status & 0b0111_1111;
                    }
                }
                0x00 => {
                    return;
                }
                _ => todo!()
            }
        }
    }

    // Addressing Modes
    
    // Addressing Mode: Implicit
    // For many 6502 instructions the source and destination of the information to be 
    // manipulated is implied directly by the function of the instruction itself and no 
    // further operand needs to be specified. Operations like 'Clear Carry Flag' (CLC) and 
    //'Return from Subroutine' (RTS) are implicit.
    fn imp(&mut self) -> u8 {
        todo!();
    }

    // Addressing Mode: Immediate
    // Immediate addressing allows the programmer to directly specify an 8 bit constant 
    // within the instruction. It is indicated by a '#' symbol followed by an numeric expression.
    fn imm(&mut self) -> u8 {
        todo!();
    }

    // Addressing Mode: Zero Page
    // An instruction using zero page addressing mode has only an 8 bit address operand. 
    // This limits it to addressing only the first 256 bytes of memory (e.g. $0000 to $00FF) 
    // where the most significant byte of the address is always zero. In zero page mode only 
    // the least significant byte of the address is held in the instruction making it shorter 
    // by one byte (important for space saving) and one less memory fetch during execution 
    // (important for speed).
	fn zp0(&mut self) -> u8 {
        todo!();
    }		

    // Addressing Mode: Zero Page, X
    // The address to be accessed by an instruction using indexed zero page addressing is 
    // calculated by taking the 8 bit zero page address from the instruction and adding the 
    // current value of the X register to it.
    fn zpx(&mut self) -> u8 {
        todo!();
    }

    // Addressing Mode: Zero Page, Y
    // The address to be accessed by an instruction using indexed zero page addressing is 
    // calculated by taking the 8 bit zero page address from the instruction and adding the 
    // current value of the Y register to it. This mode can only be used with the LDX and STX 
    // instructions.
	fn zpy(&mut self) -> u8 {
        todo!();
    }

    // Addressing Mode: Relative
    // Relative addressing mode is used by branch instructions (e.g. BEQ, BNE, etc.) which 
    // contain a signed 8 bit relative offset (e.g. -128 to +127) which is added to program 
    // counter if the condition is true. As the program counter itself is incremented during 
    // instruction execution by two the effective address range for the target instruction 
    // must be with -126 to +129 bytes of the branch.
    fn rel(&mut self) -> u8 {
        todo!();
    }

    // Addressing Mode: Absolute
    // Instructions using absolute addressing contain a full 16 bit address to identify the 
    // target location. 
	fn abs(&mut self) -> u8 {
        todo!();
    }

    // Addressing Mode: Absolute, Y
    // The address to be accessed by an instruction using X register indexed absolute addressing 
    // is computed by taking the 16 bit address from the instruction and added the contents of 
    // the X register. For example if X contains $92 then an STA $2000,X instruction will store 
    // the accumulator at $2092 (e.g. $2000 + $92).
    fn abx(&mut self) -> u8 {
        todo!();
    }

    // Addressing Mode: Absolute, X
    // The Y register indexed absolute addressing mode is the same as the previous mode only with 
    // the contents of the Y register added to the 16 bit address from the instruction.
	fn aby(&mut self) -> u8 {
        todo!();
    }	

    // Addressing Mode: Indirect
    // JMP is the only 6502 instruction to support indirection. The instruction contains a 16 bit 
    // address which identifies the location of the least significant byte of another 16 bit memory 
    // address which is the real target of the instruction.
    fn ind(&mut self) -> u8 {
        todo!();
    }

    // Addressing Mode: Index Indirect 
    // Indexed indirect addressing is normally used in conjunction with a table of address held on 
    // zero page. The address of the table is taken from the instruction and the X register added to 
    // it (with zero page wrap around) to give the location of the least significant byte of the 
    // target address.
    fn izx(&mut self) -> u8 {
        todo!();
    }

    // Addressing Mode: Indirect Indexed
    // Indirect indirect addressing is the most common indirection mode used on the 6502. In 
    // instruction contains the zero page location of the least significant byte of 16 bit address. 
    // The Y register is dynamically added to this value to generated the actual target address for 
    // operation.
    fn izy(&mut self) -> u8 {
        todo!();
    }

    // Instructions
    
    // Instruction: Add with Carry
    fn adc(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Logic AND
    fn and(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Arithmetic Shift Left
    fn asl(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Branch if Carry Clear
    fn bcc(&mut self) -> u8 {
        todo!();
    }

    // Instruction:  Branch if Carrt Set
	fn bcs(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Branch if Equal
    fn beq(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Bit Test
    fn bit(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Branch if Minus
    fn bmi(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Branch if Not Equal
	fn bne(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Branch if Positive
    fn bpl(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Force Interrupt
    fn brk(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Branch if Overflow Clear
    fn bvc(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Branch Carry Flag
	fn bvs(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Clear Carry Flag
    fn clc(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Clear Decimal Mode
    fn cld(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Clear Interrupt Disable
    fn cli(&mut self) -> u8 {
        todo!();
    }

    // Instruction:  Clear Overflow Flag
	fn clv(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Compare
    fn cmp(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Compare X Register
    fn cpx(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Compare Y Register
    fn cpy(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Decrement Memory
	fn dec(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Decrement X Register
    fn dex(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Decrement Y Register
    fn dey(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Exclusive OR
    fn eor(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Increment Memory
	fn inc(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Increment X Register
    fn inx(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Increment Y Register
    fn iny(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Jump
    fn jmp(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Jump to Subroutine
	fn jsr(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Load Accumulator
    fn lda(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Load X Register
    fn ldx(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Load Y Register
    fn ldy(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Logical Shift Right
	fn lsr(&mut self) -> u8 {
        todo!();
    }

    // Instruction: No Operation
    fn nop(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Logical Inclusive OR
    fn ora(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Push Accumulator
    fn pha(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Push Processor Status
	fn php(&mut self) -> u8 {
        todo!();
    }
    
    // Instruction: Pull Accumulator
    fn pla(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Pull Processor Status
    fn plp(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Rotate Left
    fn rol(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Rotate Right
	fn ror(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Return from Interrupt
    fn rti(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Return from Subroutine
    fn rts(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Subtract with Carry
    fn sbc(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Set Carry Flag
	fn sec(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Set Decimal Flag
    fn sed(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Set Interrupt Disable
    fn sei(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Store Accumulator
    fn sta(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Store X Register
	fn stx(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Store Y Register
    fn sty(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Transfer Accumulator to X
    fn tax(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Transfer Accumulator to Y
    fn tay(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Transfer Stack Pointer to X
	fn tsx(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Transfer X to Accumulator
    fn txa(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Transfer X to Stack Pointer
    fn txs(&mut self) -> u8 {
        todo!();
    }

    // Instruction: Transfer Y to Accumulator
    fn tya(&mut self) -> u8 {
        todo!();
    }

    fn xxx(&mut self) -> u8 {
        todo!();
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
 
    #[test]
    fn test_0xa9_lda_immidiate_load_data() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.reg_acc, 0x05);
        assert!(cpu.reg_status & 0b0000_0010 == 0b00);
        assert!(cpu.reg_status & 0b1000_0000 == 0);
    }
 
     #[test]
     fn test_0xa9_lda_zero_flag() {
         let mut cpu = CPU::new();
         cpu.interpret(vec![0xa9, 0x00, 0x00]);
         assert!(cpu.reg_status & 0b0000_0010 == 0b10);
     }
}