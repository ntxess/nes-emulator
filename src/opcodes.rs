use crate::cpu::{CPU, StatusFlags};

pub struct Instruction {
    opcode:   Box<dyn Fn(&InstructionSet, &mut CPU)>,
    addrmode: Box<dyn Fn(&mut CPU) -> u16>,
    cycle:    u8,
}

pub struct InstructionSet {
    pub matrix: Vec<Instruction>
}

impl InstructionSet {
    pub fn new() -> Self {
        InstructionSet {
            // Massive Instruction Set Matrix from OneLoneCoder's own NEW emulator repo, Thank you!
            // This matrix was modified from OneLoneCoder's C++ project to RUST.
            // Copyright 2018, 2019, 2020, 2021 OneLoneCoder.com
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

    // Auxillary Functions
    pub fn call_opcode(&self, code: u8) -> &Box<dyn Fn(&InstructionSet, &mut CPU)> {
        return &self.matrix[code as usize].opcode;
    }

    pub fn call_addrmode(&self, code: u8) -> &Box<dyn Fn(&mut CPU) -> u16> {
        return &self.matrix[code as usize].addrmode;
    }

    pub fn get_cycle(&self, code: usize) -> u8 {
        return self.matrix[code].cycle;
    }

    // TODO Addressing Mode: Implicit
    fn imp(_cpu: &mut CPU) -> u16 {
        //fetched = _cpu.reg_acc;
        0
    }

    // Addressing Mode: Immediate
    fn imm(_cpu: &mut CPU) -> u16 {
        _cpu.reg_pc += 1;

        let data = _cpu.memory[(_cpu.reg_pc) as usize] as u16;
        data
    }

    // Addressing Mode: Zero Page
	fn zp0(_cpu: &mut CPU) -> u16 {
        _cpu.reg_pc += 1;

        let address = _cpu.memory[(_cpu.reg_pc) as usize] as u16;
        let data = _cpu.memory[(address) as usize] as u16;
        data
    }		

    // Addressing Mode: Zero Page, X
    fn zpx(_cpu: &mut CPU) -> u16 {
        _cpu.reg_pc += 1;

        let pos = _cpu.memory[(_cpu.reg_pc) as usize];
        let address = pos.wrapping_add(_cpu.reg_x) as u16;
        let data = _cpu.memory[(address) as usize] as u16;
        data
    }

    // Addressing Mode: Zero Page, Y
	fn zpy(_cpu: &mut CPU) -> u16 {
        _cpu.reg_pc += 1;

        let pos = _cpu.memory[(_cpu.reg_pc) as usize];
        let address = pos.wrapping_add(_cpu.reg_y) as u16;
        let data = _cpu.memory[(address) as usize] as u16;
        data
    }

    // TODO Addressing Mode: Relative
    fn rel(_cpu: &mut CPU) -> u16 {
        // _cpu.reg_pc += 1;
        0
    }

    // TODO Addressing Mode: Absolute
	fn abs(_cpu: &mut CPU) -> u16 {
        // _cpu.reg_pc += 1;
        0
    }

    // Addressing Mode: Absolute, X
    fn abx(_cpu: &mut CPU) -> u16 {
        _cpu.reg_pc += 1;

        let base = _cpu.mem_read_u16(_cpu.reg_pc);
        let address = base.wrapping_add(_cpu.reg_x as u16);
        let data = _cpu.memory[(address) as usize] as u16;
        data
    }

    // Addressing Mode: Absolute, Y
	fn aby(_cpu: &mut CPU) -> u16 {
        _cpu.reg_pc += 1;

        let base = _cpu.mem_read_u16(_cpu.reg_pc);
        let address = base.wrapping_add(_cpu.reg_y as u16);
        let data = _cpu.memory[(address) as usize] as u16;
        data
    }	

    // TODO Addressing Mode: Indirect
    fn ind(_cpu: &mut CPU) -> u16 {
        // _cpu.reg_pc += 1;
        0
    }

    // Addressing Mode: Indirect Indexed X
    fn izx(_cpu: &mut CPU) -> u16 {
        _cpu.reg_pc += 1;

        let base = _cpu.mem_read(_cpu.reg_pc);
        let ptr: u8 = (base as u8).wrapping_add(_cpu.reg_x);
        let lo = _cpu.mem_read(ptr as u16);
        let hi = _cpu.mem_read(ptr.wrapping_add(1) as u16);
        let address = (hi as u16) << 8 | (lo as u16);
        let data = _cpu.memory[(address) as usize] as u16;
        data
    }

    // Addressing Mode: Indirect Indexed Y
    fn izy(_cpu: &mut CPU) -> u16 {
        _cpu.reg_pc += 1;

        let base = _cpu.mem_read(_cpu.reg_pc);
        let ptr: u8 = (base as u8).wrapping_add(_cpu.reg_y);
        let lo = _cpu.mem_read(ptr as u16);
        let hi = _cpu.mem_read(ptr.wrapping_add(1) as u16);
        let address = (hi as u16) << 8 | (lo as u16);
        let data = _cpu.memory[(address) as usize] as u16;
        data
    }

    // TODO Instruction: Add with Carry
    fn adc(&self, _cpu: &mut CPU) {

    }

    // Instruction: Logic AND
    fn and(&self, _cpu: &mut CPU) {
        let fetched = self.call_addrmode(_cpu.memory[_cpu.reg_pc as usize])(_cpu);
        let data = _cpu.mem_read(fetched);
        _cpu.reg_acc = _cpu.reg_acc & data;

        _cpu.set_status_flags(_cpu.reg_acc == 0, StatusFlags::ZERO);
        _cpu.set_status_flags((_cpu.reg_acc & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // TODO Instruction: Arithmetic Shift Left
    fn asl(&self, _cpu: &mut CPU) {
        let fetched = self.call_addrmode(_cpu.memory[_cpu.reg_pc as usize])(_cpu);

    }

    // Instruction: Branch if Carry Clear
    fn bcc(&self, _cpu: &mut CPU) {
        let fetched = self.call_addrmode(_cpu.memory[_cpu.reg_pc as usize])(_cpu);
        
        if !_cpu.reg_status.contains(StatusFlags::CARRY) {
            _cpu.reg_pc = _cpu.reg_pc.wrapping_add(fetched);
        }
    }

    // Instruction: Branch if Carry Set
	fn bcs(&self, _cpu: &mut CPU) {
        let fetched = self.call_addrmode(_cpu.memory[_cpu.reg_pc as usize])(_cpu);
        
        if _cpu.reg_status.contains(StatusFlags::CARRY) {
            _cpu.reg_pc = _cpu.reg_pc.wrapping_add(fetched);
        }
    }

    // Instruction: Branch if Equal
    fn beq(&self, _cpu: &mut CPU) {
        let fetched = self.call_addrmode(_cpu.memory[_cpu.reg_pc as usize])(_cpu);

        if _cpu.reg_status.contains(StatusFlags::ZERO) {
            _cpu.reg_pc = _cpu.reg_pc.wrapping_add(fetched);
        }
    }

    // Instruction: Bit Test
    fn bit(&self, _cpu: &mut CPU) {
        let fetched = self.call_addrmode(_cpu.memory[_cpu.reg_pc as usize])(_cpu);

        let data = _cpu.mem_read(fetched);

        _cpu.set_status_flags((_cpu.reg_acc & data) == 0, StatusFlags::ZERO);
        _cpu.set_status_flags((fetched & StatusFlags::OVERFLOW.bits() as u16) != 0, StatusFlags::OVERFLOW);
        _cpu.set_status_flags((fetched & StatusFlags::NEGATIVE.bits() as u16) != 0, StatusFlags::NEGATIVE);
    }

    // Instruction: Branch if Minus
    fn bmi(&self, _cpu: &mut CPU) {
        let fetched = self.call_addrmode(_cpu.memory[_cpu.reg_pc as usize])(_cpu);

        if _cpu.reg_status.contains(StatusFlags::NEGATIVE) {
            _cpu.reg_pc = _cpu.reg_pc.wrapping_add(fetched);
        }
    }

    // Instruction: Branch if Not Equal
	fn bne(&self, _cpu: &mut CPU) {
        let fetched = self.call_addrmode(_cpu.memory[_cpu.reg_pc as usize])(_cpu);

        if !_cpu.reg_status.contains(StatusFlags::ZERO) {
            _cpu.reg_pc = _cpu.reg_pc.wrapping_add(fetched);
        }
    }

    // Instruction: Branch if Positive
    fn bpl(&self, _cpu: &mut CPU) {
        let fetched = self.call_addrmode(_cpu.memory[_cpu.reg_pc as usize])(_cpu);

        if !_cpu.reg_status.contains(StatusFlags::NEGATIVE) {
            _cpu.reg_pc = _cpu.reg_pc.wrapping_add(fetched);
        }
    }

    // TODO Instruction: Force Interrupt
    fn brk(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Branch if Overflow Clear
    fn bvc(&self, _cpu: &mut CPU) {
        let fetched = self.call_addrmode(_cpu.memory[_cpu.reg_pc as usize])(_cpu);
        
        if !_cpu.reg_status.contains(StatusFlags::OVERFLOW) {
            _cpu.reg_pc = _cpu.reg_pc.wrapping_add(fetched);
        }
    }

    // Instruction: Branch Carry Flag
	fn bvs(&self, _cpu: &mut CPU) {
        let fetched = self.call_addrmode(_cpu.memory[_cpu.reg_pc as usize])(_cpu);
        
        if _cpu.reg_status.contains(StatusFlags::OVERFLOW) {
            _cpu.reg_pc = _cpu.reg_pc.wrapping_add(fetched);
        }
    }

    // Instruction: Clear Carry Flag
    fn clc(&self, _cpu: &mut CPU) {
        _cpu.reg_status.remove(StatusFlags::CARRY);
    }

    // Instruction: Clear Decimal Mode
    fn cld(&self, _cpu: &mut CPU) {
        _cpu.reg_status.remove(StatusFlags::DECIMAL);
    }

    // Instruction: Clear Interrupt Disable
    fn cli(&self, _cpu: &mut CPU) {
        _cpu.reg_status.remove(StatusFlags::INTERRUPT);
    }

    // Instruction:  Clear Overflow Flag
	fn clv(&self, _cpu: &mut CPU) {
        _cpu.reg_status.remove(StatusFlags::OVERFLOW);
    }

    // Instruction: Compare
    fn cmp(&self, _cpu: &mut CPU) {
        let fetched = self.call_addrmode(_cpu.memory[_cpu.reg_pc as usize])(_cpu);

        let data = _cpu.mem_read(fetched);
        let result = _cpu.reg_acc.wrapping_sub(data);

        _cpu.set_status_flags(_cpu.reg_acc >= data, StatusFlags::CARRY);
        _cpu.set_status_flags(_cpu.reg_acc == data, StatusFlags::ZERO);
        _cpu.set_status_flags((result & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // Instruction: Compare X Register
    fn cpx(&self, _cpu: &mut CPU) {
        let fetched = self.call_addrmode(_cpu.memory[_cpu.reg_pc as usize])(_cpu);

        let data = _cpu.mem_read(fetched);
        let result = _cpu.reg_x.wrapping_sub(data);

        _cpu.set_status_flags(_cpu.reg_x >= data, StatusFlags::CARRY);
        _cpu.set_status_flags(_cpu.reg_x == data, StatusFlags::ZERO);
        _cpu.set_status_flags((result & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // Instruction: Compare Y Register
    fn cpy(&self, _cpu: &mut CPU) {
        let fetched = self.call_addrmode(_cpu.memory[_cpu.reg_pc as usize])(_cpu);

        let data = _cpu.mem_read(fetched);
        let result = _cpu.reg_y.wrapping_sub(data);

        _cpu.set_status_flags(_cpu.reg_y >= data, StatusFlags::CARRY);
        _cpu.set_status_flags(_cpu.reg_y == data, StatusFlags::ZERO);
        _cpu.set_status_flags((result & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // Instruction: Decrement Memory
	fn dec(&self, _cpu: &mut CPU) {
        let fetched = self.call_addrmode(_cpu.memory[_cpu.reg_pc as usize])(_cpu);

        let data = _cpu.mem_read(fetched).wrapping_sub(1);
        _cpu.mem_write(fetched, data);

        _cpu.set_status_flags(data == 0, StatusFlags::ZERO);
        _cpu.set_status_flags((data & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // Instruction: Decrement X Register
    fn dex(&self, _cpu: &mut CPU) {
        _cpu.reg_x = _cpu.reg_x.wrapping_sub(1);

        _cpu.set_status_flags(_cpu.reg_x == 0, StatusFlags::ZERO);
        _cpu.set_status_flags((_cpu.reg_x & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // Instruction: Decrement Y Register
    fn dey(&self, _cpu: &mut CPU) {
        _cpu.reg_y = _cpu.reg_y.wrapping_sub(1);

        _cpu.set_status_flags(_cpu.reg_y == 0, StatusFlags::ZERO);
        _cpu.set_status_flags((_cpu.reg_y & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // Instruction: Exclusive OR
    fn eor(&self, _cpu: &mut CPU) {
        let fetched = self.call_addrmode(_cpu.memory[_cpu.reg_pc as usize])(_cpu);
        
        let data = _cpu.mem_read(fetched);
        _cpu.reg_acc = _cpu.reg_acc ^ data;

        _cpu.set_status_flags(_cpu.reg_acc == 0, StatusFlags::ZERO);
        _cpu.set_status_flags((_cpu.reg_acc & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // Instruction: Increment Memory
	fn inc(&self, _cpu: &mut CPU) {
        let fetched = self.call_addrmode(_cpu.memory[_cpu.reg_pc as usize])(_cpu);

        let data = _cpu.mem_read(fetched).wrapping_add(1);
        _cpu.mem_write(fetched, data);

        _cpu.set_status_flags(data == 0, StatusFlags::ZERO);
        _cpu.set_status_flags((data & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // Instruction: Increment X Register
    fn inx(&self, _cpu: &mut CPU) {
        _cpu.reg_x = _cpu.reg_x.wrapping_add(1);

        _cpu.set_status_flags(_cpu.reg_x == 0, StatusFlags::ZERO);
        _cpu.set_status_flags((_cpu.reg_x & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // Instruction: Increment Y Register
    fn iny(&self, _cpu: &mut CPU) {
        _cpu.reg_y = _cpu.reg_y.wrapping_add(1);

        _cpu.set_status_flags(_cpu.reg_y == 0, StatusFlags::ZERO);
        _cpu.set_status_flags((_cpu.reg_y & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // TODO Instruction: Jump
    fn jmp(&self, _cpu: &mut CPU) {
        let fetched = self.call_addrmode(_cpu.memory[_cpu.reg_pc as usize])(_cpu);
         
    }

    // TODO Instruction: Jump to Subroutine
	fn jsr(&self, _cpu: &mut CPU) {
        let fetched = self.call_addrmode(_cpu.memory[_cpu.reg_pc as usize])(_cpu);
         
    }

    // Instruction: Load Accumulator
    fn lda(&self, _cpu: &mut CPU) {
        let fetched = self.call_addrmode(_cpu.memory[_cpu.reg_pc as usize])(_cpu);

        _cpu.reg_acc = fetched as u8;

        _cpu.set_status_flags(_cpu.reg_acc == 0, StatusFlags::ZERO);
        _cpu.set_status_flags((_cpu.reg_acc & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // Instruction: Load X Register
    fn ldx(&self, _cpu: &mut CPU) {
        let fetched = self.call_addrmode(_cpu.memory[_cpu.reg_pc as usize])(_cpu);

        _cpu.reg_x = fetched as u8;

        _cpu.set_status_flags(_cpu.reg_x == 0, StatusFlags::ZERO);
        _cpu.set_status_flags((_cpu.reg_x & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // Instruction: Load Y Register
    fn ldy(&self, _cpu: &mut CPU) {
        let fetched = self.call_addrmode(_cpu.memory[_cpu.reg_pc as usize])(_cpu);
        
        _cpu.reg_y = fetched as u8;

        _cpu.set_status_flags(_cpu.reg_y == 0, StatusFlags::ZERO);
        _cpu.set_status_flags((_cpu.reg_y & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // TODO Instruction: Logical Shift Right
	fn lsr(&self, _cpu: &mut CPU) {
        let fetched = self.call_addrmode(_cpu.memory[_cpu.reg_pc as usize])(_cpu);
         
    }

    // Instruction: No Operation
    fn nop(&self, _cpu: &mut CPU) {}

    // TODO Instruction: Logical Inclusive OR
    fn ora(&self, _cpu: &mut CPU) {
        let fetched = self.call_addrmode(_cpu.memory[_cpu.reg_pc as usize])(_cpu);
         
    }

    // TODO Instruction: Push Accumulator
    fn pha(&self, _cpu: &mut CPU) {
         
    }

    // TODO Instruction: Push Processor Status
	fn php(&self, _cpu: &mut CPU) {
         
    }
    
    // TODO Instruction: Pull Accumulator
    fn pla(&self, _cpu: &mut CPU) {
         
    }

    // TODO Instruction: Pull Processor Status
    fn plp(&self, _cpu: &mut CPU) {
         
    }

    // TODO Instruction: Rotate Left
    fn rol(&self, _cpu: &mut CPU) {
        let fetched = self.call_addrmode(_cpu.memory[_cpu.reg_pc as usize])(_cpu);
         
    }

    // TODO Instruction: Rotate Right
	fn ror(&self, _cpu: &mut CPU) {
        let fetched = self.call_addrmode(_cpu.memory[_cpu.reg_pc as usize])(_cpu);
         
    }

    // TODO Instruction: Return from Interrupt
    fn rti(&self, _cpu: &mut CPU) {
         
    }

    // TODO Instruction: Return from Subroutine
    fn rts(&self, _cpu: &mut CPU) {
         
    }

    // TODO Instruction: Subtract with Carry
    fn sbc(&self, _cpu: &mut CPU) {
        let fetched = self.call_addrmode(_cpu.memory[_cpu.reg_pc as usize])(_cpu);
         
    }

    // Instruction: Set Carry Flag
	fn sec(&self, _cpu: &mut CPU) {
        _cpu.reg_status.insert(StatusFlags::CARRY);
    }

    // Instruction: Set Decimal Flag
    fn sed(&self, _cpu: &mut CPU) {
        _cpu.reg_status.insert(StatusFlags::DECIMAL);   
    }

    // Instruction: Set Interrupt Disable
    fn sei(&self, _cpu: &mut CPU) {
        _cpu.reg_status.insert(StatusFlags::INTERRUPT);   
    }

    // Instruction: Store Accumulator
    fn sta(&self, _cpu: &mut CPU) {
        let fetched = self.call_addrmode(_cpu.memory[_cpu.reg_pc as usize])(_cpu);
         
        _cpu.mem_write(fetched, _cpu.reg_acc);
    }

    // Instruction: Store X Register
	fn stx(&self, _cpu: &mut CPU) {
        let fetched = self.call_addrmode(_cpu.memory[_cpu.reg_pc as usize])(_cpu);
         
        _cpu.mem_write(fetched, _cpu.reg_x);
    }

    // Instruction: Store Y Register
    fn sty(&self, _cpu: &mut CPU) {
        let fetched = self.call_addrmode(_cpu.memory[_cpu.reg_pc as usize])(_cpu);
        
        _cpu.mem_write(fetched, _cpu.reg_y);
    }

    // Instruction: Transfer Accumulator to X
    fn tax(&self, _cpu: &mut CPU) {
        _cpu.reg_x = _cpu.reg_acc;

        _cpu.set_status_flags(_cpu.reg_x == 0, StatusFlags::ZERO);
        _cpu.set_status_flags((_cpu.reg_x & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // Instruction: Transfer Accumulator to Y
    fn tay(&self, _cpu: &mut CPU) {
        _cpu.reg_y = _cpu.reg_acc;

        _cpu.set_status_flags(_cpu.reg_y == 0, StatusFlags::ZERO);
        _cpu.set_status_flags((_cpu.reg_y & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // TODO Instruction: Transfer Stack Pointer to X
	fn tsx(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Transfer X to Accumulator
    fn txa(&self, _cpu: &mut CPU) {
        _cpu.reg_acc = _cpu.reg_x;

        _cpu.set_status_flags(_cpu.reg_acc == 0, StatusFlags::ZERO);
        _cpu.set_status_flags((_cpu.reg_acc & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // TODO Instruction: Transfer X to Stack Pointer
    fn txs(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Transfer Y to Accumulator
    fn tya(&self, _cpu: &mut CPU) {
        _cpu.reg_acc = _cpu.reg_y;

        _cpu.set_status_flags(_cpu.reg_acc == 0, StatusFlags::ZERO);
        _cpu.set_status_flags((_cpu.reg_acc & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // Added Instruction to fill the Opcode Matrix; does not exist in the real NES instruction set
    // Effectively does nothing but does nothing better than NOP
    // According to OLC not all NOPs are similar so this opcode exist as a true NOP
    fn xxx(&self, _cpu: &mut CPU) {}
}