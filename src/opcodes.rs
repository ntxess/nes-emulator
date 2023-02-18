use crate::cpu::{CPU, StatusFlags, Mem};

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
        println!("New Set");
        InstructionSet {
            // Massive Instruction Set Matrix from OneLoneCoder's own emulator repo, Thank you!
            // Copyright 2018, 2019, 2020, 2021 OneLoneCoder.com
            // OLC's Original matrix does not support the unofficial opcodes 
            // This matrix was modified with additions of the unofficial opcodes + address fixes
            // Additional fix from original: Brk opcode changed from imm to imp per the NES engineering guide.
            matrix: vec![
                Instruction{opcode: Box::new(InstructionSet::brk), addrmode: Box::new(InstructionSet::imp), cycle: 7},
                Instruction{opcode: Box::new(InstructionSet::ora), addrmode: Box::new(InstructionSet::izx), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::slo), addrmode: Box::new(InstructionSet::izx), cycle: 8},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::zp0), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::ora), addrmode: Box::new(InstructionSet::zp0), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::asl), addrmode: Box::new(InstructionSet::zp0), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::slo), addrmode: Box::new(InstructionSet::zp0), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::php), addrmode: Box::new(InstructionSet::imp), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::ora), addrmode: Box::new(InstructionSet::imm), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::asl), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::anc), addrmode: Box::new(InstructionSet::imm), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::abs), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::ora), addrmode: Box::new(InstructionSet::abs), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::asl), addrmode: Box::new(InstructionSet::abs), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::slo), addrmode: Box::new(InstructionSet::abs), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::bpl), addrmode: Box::new(InstructionSet::rel), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::ora), addrmode: Box::new(InstructionSet::izy), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::slo), addrmode: Box::new(InstructionSet::izy), cycle: 8},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::zpx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::ora), addrmode: Box::new(InstructionSet::zpx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::asl), addrmode: Box::new(InstructionSet::zpx), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::slo), addrmode: Box::new(InstructionSet::zpx), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::clc), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::ora), addrmode: Box::new(InstructionSet::aby), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::slo), addrmode: Box::new(InstructionSet::aby), cycle: 7},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::abx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::ora), addrmode: Box::new(InstructionSet::abx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::asl), addrmode: Box::new(InstructionSet::abx), cycle: 7},
                Instruction{opcode: Box::new(InstructionSet::slo), addrmode: Box::new(InstructionSet::abx), cycle: 7},
                Instruction{opcode: Box::new(InstructionSet::jsr), addrmode: Box::new(InstructionSet::abs), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::and), addrmode: Box::new(InstructionSet::izx), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::rla), addrmode: Box::new(InstructionSet::izx), cycle: 8},
                Instruction{opcode: Box::new(InstructionSet::bit), addrmode: Box::new(InstructionSet::zp0), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::and), addrmode: Box::new(InstructionSet::zp0), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::rol), addrmode: Box::new(InstructionSet::zp0), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::rla), addrmode: Box::new(InstructionSet::zp0), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::plp), addrmode: Box::new(InstructionSet::imp), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::and), addrmode: Box::new(InstructionSet::imm), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::rol), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::anc), addrmode: Box::new(InstructionSet::imm), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::bit), addrmode: Box::new(InstructionSet::abs), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::and), addrmode: Box::new(InstructionSet::abs), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::rol), addrmode: Box::new(InstructionSet::abs), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::rla), addrmode: Box::new(InstructionSet::abs), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::bmi), addrmode: Box::new(InstructionSet::rel), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::and), addrmode: Box::new(InstructionSet::izy), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::rla), addrmode: Box::new(InstructionSet::izy), cycle: 8},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::zpx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::and), addrmode: Box::new(InstructionSet::zpx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::rol), addrmode: Box::new(InstructionSet::zpx), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::rla), addrmode: Box::new(InstructionSet::zpx), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::sec), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::and), addrmode: Box::new(InstructionSet::aby), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::rla), addrmode: Box::new(InstructionSet::aby), cycle: 7},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::abx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::and), addrmode: Box::new(InstructionSet::abx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::rol), addrmode: Box::new(InstructionSet::abx), cycle: 7},
                Instruction{opcode: Box::new(InstructionSet::rla), addrmode: Box::new(InstructionSet::abx), cycle: 7},
                Instruction{opcode: Box::new(InstructionSet::rti), addrmode: Box::new(InstructionSet::imp), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::eor), addrmode: Box::new(InstructionSet::izx), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::sre), addrmode: Box::new(InstructionSet::izx), cycle: 8},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::zp0), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::eor), addrmode: Box::new(InstructionSet::zp0), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::lsr), addrmode: Box::new(InstructionSet::zp0), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::sre), addrmode: Box::new(InstructionSet::zp0), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::pha), addrmode: Box::new(InstructionSet::imp), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::eor), addrmode: Box::new(InstructionSet::imm), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::lsr), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::alr), addrmode: Box::new(InstructionSet::imm), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::jmp), addrmode: Box::new(InstructionSet::abs), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::eor), addrmode: Box::new(InstructionSet::abs), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::lsr), addrmode: Box::new(InstructionSet::abs), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::sre), addrmode: Box::new(InstructionSet::abs), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::bvc), addrmode: Box::new(InstructionSet::rel), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::eor), addrmode: Box::new(InstructionSet::izy), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::sre), addrmode: Box::new(InstructionSet::izy), cycle: 8},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::zpx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::eor), addrmode: Box::new(InstructionSet::zpx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::lsr), addrmode: Box::new(InstructionSet::zpx), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::sre), addrmode: Box::new(InstructionSet::zpx), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::cli), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::eor), addrmode: Box::new(InstructionSet::aby), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::sre), addrmode: Box::new(InstructionSet::aby), cycle: 7},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::abx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::eor), addrmode: Box::new(InstructionSet::abx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::lsr), addrmode: Box::new(InstructionSet::abx), cycle: 7},
                Instruction{opcode: Box::new(InstructionSet::sre), addrmode: Box::new(InstructionSet::abx), cycle: 7},
                Instruction{opcode: Box::new(InstructionSet::rts), addrmode: Box::new(InstructionSet::imp), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::adc), addrmode: Box::new(InstructionSet::izx), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::rra), addrmode: Box::new(InstructionSet::izx), cycle: 8},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::zp0), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::adc), addrmode: Box::new(InstructionSet::zp0), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::ror), addrmode: Box::new(InstructionSet::zp0), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::rra), addrmode: Box::new(InstructionSet::zp0), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::pla), addrmode: Box::new(InstructionSet::imp), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::adc), addrmode: Box::new(InstructionSet::imm), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::ror), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::arr), addrmode: Box::new(InstructionSet::imm), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::jmp), addrmode: Box::new(InstructionSet::ind), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::adc), addrmode: Box::new(InstructionSet::abs), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::ror), addrmode: Box::new(InstructionSet::abs), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::rra), addrmode: Box::new(InstructionSet::abs), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::bvs), addrmode: Box::new(InstructionSet::rel), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::adc), addrmode: Box::new(InstructionSet::izy), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::rra), addrmode: Box::new(InstructionSet::izy), cycle: 8},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::zpx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::adc), addrmode: Box::new(InstructionSet::zpx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::ror), addrmode: Box::new(InstructionSet::zpx), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::rra), addrmode: Box::new(InstructionSet::zpx), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::sei), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::adc), addrmode: Box::new(InstructionSet::aby), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::rra), addrmode: Box::new(InstructionSet::aby), cycle: 7},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::abx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::adc), addrmode: Box::new(InstructionSet::abx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::ror), addrmode: Box::new(InstructionSet::abx), cycle: 7},
                Instruction{opcode: Box::new(InstructionSet::rra), addrmode: Box::new(InstructionSet::abx), cycle: 7},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::sta), addrmode: Box::new(InstructionSet::izx), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::sax), addrmode: Box::new(InstructionSet::izx), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::sty), addrmode: Box::new(InstructionSet::zp0), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::sta), addrmode: Box::new(InstructionSet::zp0), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::stx), addrmode: Box::new(InstructionSet::zp0), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::sax), addrmode: Box::new(InstructionSet::zp0), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::dey), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::txa), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::xaa), addrmode: Box::new(InstructionSet::imm), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::sty), addrmode: Box::new(InstructionSet::abs), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::sta), addrmode: Box::new(InstructionSet::abs), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::stx), addrmode: Box::new(InstructionSet::abs), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::sax), addrmode: Box::new(InstructionSet::abs), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::bcc), addrmode: Box::new(InstructionSet::rel), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::sta), addrmode: Box::new(InstructionSet::izy), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::ahx), addrmode: Box::new(InstructionSet::izy), cycle: 8},
                Instruction{opcode: Box::new(InstructionSet::sty), addrmode: Box::new(InstructionSet::zpx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::sta), addrmode: Box::new(InstructionSet::zpx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::stx), addrmode: Box::new(InstructionSet::zpy), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::sax), addrmode: Box::new(InstructionSet::zpy), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::tya), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::sta), addrmode: Box::new(InstructionSet::aby), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::txs), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::tas), addrmode: Box::new(InstructionSet::aby), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::shy), addrmode: Box::new(InstructionSet::abx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::sta), addrmode: Box::new(InstructionSet::abx), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::shx), addrmode: Box::new(InstructionSet::aby), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::ahx), addrmode: Box::new(InstructionSet::aby), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::ldy), addrmode: Box::new(InstructionSet::imm), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::lda), addrmode: Box::new(InstructionSet::izx), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::ldx), addrmode: Box::new(InstructionSet::imm), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::lax), addrmode: Box::new(InstructionSet::izx), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::ldy), addrmode: Box::new(InstructionSet::zp0), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::lda), addrmode: Box::new(InstructionSet::zp0), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::ldx), addrmode: Box::new(InstructionSet::zp0), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::lax), addrmode: Box::new(InstructionSet::zp0), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::tay), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::lda), addrmode: Box::new(InstructionSet::imm), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::tax), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::lxa), addrmode: Box::new(InstructionSet::imm), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::ldy), addrmode: Box::new(InstructionSet::abs), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::lda), addrmode: Box::new(InstructionSet::abs), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::ldx), addrmode: Box::new(InstructionSet::abs), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::lax), addrmode: Box::new(InstructionSet::abs), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::bcs), addrmode: Box::new(InstructionSet::rel), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::lda), addrmode: Box::new(InstructionSet::izy), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::lax), addrmode: Box::new(InstructionSet::izy), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::ldy), addrmode: Box::new(InstructionSet::zpx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::lda), addrmode: Box::new(InstructionSet::zpx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::ldx), addrmode: Box::new(InstructionSet::zpy), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::lax), addrmode: Box::new(InstructionSet::zpy), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::clv), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::lda), addrmode: Box::new(InstructionSet::aby), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::tsx), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::las), addrmode: Box::new(InstructionSet::aby), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::ldy), addrmode: Box::new(InstructionSet::abx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::lda), addrmode: Box::new(InstructionSet::abx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::ldx), addrmode: Box::new(InstructionSet::aby), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::lax), addrmode: Box::new(InstructionSet::aby), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::cpy), addrmode: Box::new(InstructionSet::imm), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::cmp), addrmode: Box::new(InstructionSet::izx), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::dcp), addrmode: Box::new(InstructionSet::izx), cycle: 8},
                Instruction{opcode: Box::new(InstructionSet::cpy), addrmode: Box::new(InstructionSet::zp0), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::cmp), addrmode: Box::new(InstructionSet::zp0), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::dec), addrmode: Box::new(InstructionSet::zp0), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::dcp), addrmode: Box::new(InstructionSet::zp0), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::iny), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::cmp), addrmode: Box::new(InstructionSet::imm), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::dex), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::axs), addrmode: Box::new(InstructionSet::imm), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::cpy), addrmode: Box::new(InstructionSet::abs), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::cmp), addrmode: Box::new(InstructionSet::abs), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::dec), addrmode: Box::new(InstructionSet::abs), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::dcp), addrmode: Box::new(InstructionSet::abs), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::bne), addrmode: Box::new(InstructionSet::rel), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::cmp), addrmode: Box::new(InstructionSet::izy), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::dcp), addrmode: Box::new(InstructionSet::izy), cycle: 8},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::zpx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::cmp), addrmode: Box::new(InstructionSet::zpx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::dec), addrmode: Box::new(InstructionSet::zpx), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::dcp), addrmode: Box::new(InstructionSet::zpx), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::cld), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::cmp), addrmode: Box::new(InstructionSet::aby), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::dcp), addrmode: Box::new(InstructionSet::aby), cycle: 7},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::abx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::cmp), addrmode: Box::new(InstructionSet::abx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::dec), addrmode: Box::new(InstructionSet::abx), cycle: 7},
                Instruction{opcode: Box::new(InstructionSet::dcp), addrmode: Box::new(InstructionSet::abx), cycle: 7},
                Instruction{opcode: Box::new(InstructionSet::cpx), addrmode: Box::new(InstructionSet::imm), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::sbc), addrmode: Box::new(InstructionSet::izx), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::isb), addrmode: Box::new(InstructionSet::izx), cycle: 8},
                Instruction{opcode: Box::new(InstructionSet::cpx), addrmode: Box::new(InstructionSet::zp0), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::sbc), addrmode: Box::new(InstructionSet::zp0), cycle: 3},
                Instruction{opcode: Box::new(InstructionSet::inc), addrmode: Box::new(InstructionSet::zp0), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::isb), addrmode: Box::new(InstructionSet::zp0), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::inx), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::sbc), addrmode: Box::new(InstructionSet::imm), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::sbc), addrmode: Box::new(InstructionSet::imm), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::cpx), addrmode: Box::new(InstructionSet::abs), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::sbc), addrmode: Box::new(InstructionSet::abs), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::inc), addrmode: Box::new(InstructionSet::abs), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::isb), addrmode: Box::new(InstructionSet::abs), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::beq), addrmode: Box::new(InstructionSet::rel), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::sbc), addrmode: Box::new(InstructionSet::izy), cycle: 5},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::isb), addrmode: Box::new(InstructionSet::izy), cycle: 8},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::zpx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::sbc), addrmode: Box::new(InstructionSet::zpx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::inc), addrmode: Box::new(InstructionSet::zpx), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::isb), addrmode: Box::new(InstructionSet::zpx), cycle: 6},
                Instruction{opcode: Box::new(InstructionSet::sed), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::sbc), addrmode: Box::new(InstructionSet::aby), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::imp), cycle: 2},
                Instruction{opcode: Box::new(InstructionSet::isb), addrmode: Box::new(InstructionSet::aby), cycle: 7},
                Instruction{opcode: Box::new(InstructionSet::nop), addrmode: Box::new(InstructionSet::abx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::sbc), addrmode: Box::new(InstructionSet::abx), cycle: 4},
                Instruction{opcode: Box::new(InstructionSet::inc), addrmode: Box::new(InstructionSet::abx), cycle: 7},
                Instruction{opcode: Box::new(InstructionSet::isb), addrmode: Box::new(InstructionSet::abx), cycle: 7},
            ]
        }
    }

    // Auxillary Functions
    pub fn call_opcode(&self, code: u8) -> &Box<dyn Fn(&InstructionSet, &mut CPU)> {
        return &self.matrix[code as usize].opcode;
    }

    pub fn get_address(&self, code: u8) -> &Box<dyn Fn(&mut CPU) -> u16> {
        return &self.matrix[code as usize].addrmode;
    }

    pub fn get_cycle(&self, code: u8) -> u8 {
        return self.matrix[code as usize].cycle;
    }

    // Addressing Mode: Implicit / Accumulator
    // Implicit addressing mode requires no additional logic to obtain address
    // Thus, we can use it for opcodes that require the Accumulator addressing mode as well
    // This code call is mostly redundant but added for the sake of implementation per the NES engineering guide
    fn imp(cpu: &mut CPU) -> u16 { 
        cpu.reg_acc as u16
    }

    // Addressing Mode: Immediate
    fn imm(cpu: &mut CPU) -> u16 {
        cpu.reg_pc += 1;
        cpu.reg_pc
    }

    // Addressing Mode: Zero Page
    fn zp0(cpu: &mut CPU) -> u16 {
        cpu.reg_pc += 1;

        let address = cpu.mem_read(cpu.reg_pc) as u16;
        address
    }		

    // Addressing Mode: Zero Page, X
    fn zpx(cpu: &mut CPU) -> u16 {
        cpu.reg_pc += 1;

        let pos = cpu.mem_read(cpu.reg_pc);
        let address = pos.wrapping_add(cpu.reg_x) as u16;
        address
    }

    // Addressing Mode: Zero Page, Y
    fn zpy(cpu: &mut CPU) -> u16 {
        cpu.reg_pc += 1;

        let pos = cpu.mem_read(cpu.reg_pc);
        let address = pos.wrapping_add(cpu.reg_y) as u16;
        address
    }

    // Addressing Mode: Relative
    fn rel(cpu: &mut CPU) -> u16 { 
        cpu.reg_pc += 1;
        cpu.reg_pc
    }

    // Addressing Mode: Absolute
    // Absolute addressing mode and any other addressing mode that require a memory read of u16 requires an extra
    // Program counter increment, skipping over the extra byte needed for the opcode instruction
    fn abs(cpu: &mut CPU) -> u16 {
        cpu.reg_pc += 1;

        let address = cpu.mem_read_u16(cpu.reg_pc);
        cpu.reg_pc += 1;
        address
    }

    // Addressing Mode: Absolute, X
    fn abx(cpu: &mut CPU) -> u16 {
        cpu.reg_pc += 1;

        let base = cpu.mem_read_u16(cpu.reg_pc);
        let address = base.wrapping_add(cpu.reg_x as u16);
        cpu.reg_pc += 1;
        address
    }

    // Addressing Mode: Absolute, Y
    fn aby(cpu: &mut CPU) -> u16 {
        cpu.reg_pc += 1;

        let base = cpu.mem_read_u16(cpu.reg_pc);
        let address = base.wrapping_add(cpu.reg_y as u16);
        cpu.reg_pc += 1;
        address
    }	

    // Addressing Mode: Indirect
    fn ind(cpu: &mut CPU) -> u16 {
        cpu.reg_pc += 1;

        let address = cpu.mem_read_u16(cpu.reg_pc);
        cpu.reg_pc += 1;
        address
    }

    // Addressing Mode: Indirect Indexed X
    fn izx(cpu: &mut CPU) -> u16 {
        cpu.reg_pc += 1;

        let base = cpu.mem_read(cpu.reg_pc);
        let ptr = base.wrapping_add(cpu.reg_x);
        let lo = cpu.mem_read(ptr as u16);
        let hi = cpu.mem_read(ptr.wrapping_add(1) as u16);
        let address = (hi as u16) << 8 | (lo as u16);
        address
    }

    // Addressing Mode: Indirect Indexed Y
    fn izy(cpu: &mut CPU) -> u16 {
        cpu.reg_pc += 1;

        let base = cpu.mem_read(cpu.reg_pc);
        let lo = cpu.mem_read(base as u16);
        let hi = cpu.mem_read((base as u8).wrapping_add(1) as u16);
        let deref_base = (hi as u16) << 8 | (lo as u16);
        let deref = deref_base.wrapping_add(cpu.reg_y as u16);
        deref
    }

    // Instruction: Add with Carry
    fn adc(&self, cpu: &mut CPU) {
        let address = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu);
        let data = cpu.mem_read(address);

        let sum = cpu.reg_acc as u16
            + data as u16
            + (if cpu.reg_status.contains(StatusFlags::CARRY) {
                1
            } else {
                0
            }) as u16;

        let carry = sum > 0xff;

        if carry {
            cpu.reg_status.insert(StatusFlags::CARRY);
        } else {
            cpu.reg_status.remove(StatusFlags::CARRY);
        }

        let result = sum as u8;

        if (data ^ result) & (result ^ cpu.reg_acc) & 0x80 != 0 {
            cpu.reg_status.insert(StatusFlags::OVERFLOW);
        } else {
            cpu.reg_status.remove(StatusFlags::OVERFLOW)
        }
        cpu.reg_acc = result;

        cpu.set_status_flags(cpu.reg_acc == 0, StatusFlags::ZERO);
        cpu.set_status_flags((cpu.reg_acc & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // TODO Unofficial opcode
    fn ahx(&self, cpu: &mut CPU) {
        
    }

    // TODO Unofficial opcode
    fn alr(&self, cpu: &mut CPU) {
    
    }

    // TODO Unofficial opcode
    fn anc(&self, cpu: &mut CPU) {
    
    }

    // Instruction: Logic AND
    fn and(&self, cpu: &mut CPU) {
        let address = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu);
        let data = cpu.mem_read(address);
        cpu.reg_acc = cpu.reg_acc & data;

        cpu.set_status_flags(cpu.reg_acc == 0, StatusFlags::ZERO);
        cpu.set_status_flags((cpu.reg_acc & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // TODO Unofficial opcode
    fn arr(&self, cpu: &mut CPU) {
    
    }

    // Instruction: Arithmetic Shift Left
    fn asl(&self, cpu: &mut CPU) {
        // Logic only for Accumulator addressing mode
        if 0x0a == cpu.mem_read(cpu.reg_pc) {
            let data = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu) as u8;
            cpu.set_status_flags(data >> 7 == 1, StatusFlags::CARRY);
            cpu.reg_acc = data << 1;

            cpu.set_status_flags(cpu.reg_acc == 0, StatusFlags::ZERO);
            cpu.set_status_flags((cpu.reg_acc & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
        } else {
            let address = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu);
            let mut data = cpu.mem_read(address);
            cpu.set_status_flags(data >> 7 == 1, StatusFlags::CARRY);
            data = data << 1;
            cpu.mem_write(address, data);

            cpu.set_status_flags(data == 0, StatusFlags::ZERO);
            cpu.set_status_flags((data & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
        }
    }

    // TODO Unofficial opcode
    fn axs(&self, cpu: &mut CPU) {
    
    }

    // Instruction: Branch if Carry Clear
    fn bcc(&self, cpu: &mut CPU) {
        let address = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu);

        // Every branching instruction requires an offset of -1 byte to the program counter in order to not skip the 
        // next instruction
        if !cpu.reg_status.contains(StatusFlags::CARRY) {
            let jump_addr = cpu.mem_read(address) as i8;
            cpu.reg_pc = cpu.reg_pc.wrapping_add(1).wrapping_add(jump_addr as u16) - 1;
        }
    }

    // Instruction: Branch if Carry Set
    fn bcs(&self, cpu: &mut CPU) {
        let address = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu);

        if cpu.reg_status.contains(StatusFlags::CARRY) {
            let jump_addr = cpu.mem_read(address) as i8;
            cpu.reg_pc = cpu.reg_pc.wrapping_add(1).wrapping_add(jump_addr as u16) - 1;
        }
    }

    // Instruction: Branch if Equal
    fn beq(&self, cpu: &mut CPU) {
        let address = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu);

        if cpu.reg_status.contains(StatusFlags::ZERO) {
            let jump_addr = cpu.mem_read(address) as i8;
            cpu.reg_pc = cpu.reg_pc.wrapping_add(1).wrapping_add(jump_addr as u16) - 1;
        }
    }

    // Instruction: Bit Test
    fn bit(&self, cpu: &mut CPU) {
        let address = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu);
        let data = cpu.mem_read(address);

        cpu.set_status_flags((cpu.reg_acc & data) == 0, StatusFlags::ZERO);
        cpu.reg_status.set(StatusFlags::NEGATIVE, data & 0b10000000 > 0);
        cpu.reg_status.set(StatusFlags::OVERFLOW, data & 0b01000000 > 0);
    }

    // Instruction: Branch if Minus
    fn bmi(&self, cpu: &mut CPU) {
        let address = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu);

        if cpu.reg_status.contains(StatusFlags::NEGATIVE) {
            let jump_addr = cpu.mem_read(address) as i8;
            cpu.reg_pc = cpu.reg_pc.wrapping_add(1).wrapping_add(jump_addr as u16) - 1;
        }
    }

    // Instruction: Branch if Not Equal
    fn bne(&self, cpu: &mut CPU) {
        let address = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu);

        if !cpu.reg_status.contains(StatusFlags::ZERO) {
            let jump_addr = cpu.mem_read(address) as i8;
            cpu.reg_pc = cpu.reg_pc.wrapping_add(1).wrapping_add(jump_addr as u16) - 1;
        }
    }

    // Instruction: Branch if Positive
    fn bpl(&self, cpu: &mut CPU) {
        let address = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu);

        if !cpu.reg_status.contains(StatusFlags::NEGATIVE) {
            let jump_addr = cpu.mem_read(address) as i8;
            cpu.reg_pc = cpu.reg_pc.wrapping_add(1).wrapping_add(jump_addr as u16) - 1;
        }
    }

    // Instruction: Force Interrupt
    fn brk(&self, _cpu: &mut CPU) {
        return;
    }

    // Instruction: Branch if Overflow Clear
    fn bvc(&self, cpu: &mut CPU) {
        let address = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu);

        if !cpu.reg_status.contains(StatusFlags::OVERFLOW) {
            let jump_addr = cpu.mem_read(address) as i8;
            cpu.reg_pc = cpu.reg_pc.wrapping_add(1).wrapping_add(jump_addr as u16) - 1;
        }
    }

    // Instruction: Branch Carry Flag
    fn bvs(&self, cpu: &mut CPU) {
        let address = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu);

        if cpu.reg_status.contains(StatusFlags::OVERFLOW) {
            let jump_addr = cpu.mem_read(address) as i8;
            cpu.reg_pc = cpu.reg_pc.wrapping_add(1).wrapping_add(jump_addr as u16) - 1;     
        }
    }

    // Instruction: Clear Carry Flag
    fn clc(&self, cpu: &mut CPU) {
        cpu.reg_status.remove(StatusFlags::CARRY);
    }

    // Instruction: Clear Decimal Mode
    fn cld(&self, cpu: &mut CPU) {
        cpu.reg_status.remove(StatusFlags::DECIMAL);
    }

    // Instruction: Clear Interrupt Disable
    fn cli(&self, cpu: &mut CPU) {
        cpu.reg_status.remove(StatusFlags::INTERRUPT);
    }

    // Instruction:  Clear Overflow Flag
    fn clv(&self, cpu: &mut CPU) {
        cpu.reg_status.remove(StatusFlags::OVERFLOW);
    }

    // Instruction: Compare
    fn cmp(&self, cpu: &mut CPU) {
        let address = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu);
        let data = cpu.mem_read(address);
        let result = cpu.reg_acc.wrapping_sub(data);

        cpu.set_status_flags(cpu.reg_acc >= data, StatusFlags::CARRY);
        cpu.set_status_flags(result == 0, StatusFlags::ZERO);
        cpu.set_status_flags((result & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // Instruction: Compare X Register
    fn cpx(&self, cpu: &mut CPU) {
        let address = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu);
        let data = cpu.mem_read(address);
        let result = cpu.reg_x.wrapping_sub(data);

        cpu.set_status_flags(cpu.reg_x >= data, StatusFlags::CARRY);
        cpu.set_status_flags(result == 0, StatusFlags::ZERO);
        cpu.set_status_flags((result & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // Instruction: Compare Y Register
    fn cpy(&self, cpu: &mut CPU) {
        let address = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu);
        let data = cpu.mem_read(address);
        let result = cpu.reg_y.wrapping_sub(data);

        cpu.set_status_flags(cpu.reg_y >= data, StatusFlags::CARRY);
        cpu.set_status_flags(result == 0, StatusFlags::ZERO);
        cpu.set_status_flags((result & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // Unofficial opcode
    fn dcp(&self, cpu: &mut CPU) {
        let address = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu);
        let mut data = cpu.mem_read(address);
        data = data.wrapping_sub(1);
        cpu.mem_write(address, data);

        if data <= cpu.reg_acc {
            cpu.reg_status.insert(StatusFlags::CARRY);
        }

        cpu.set_status_flags(cpu.reg_acc.wrapping_sub(data) == 0, StatusFlags::ZERO);
        cpu.set_status_flags((cpu.reg_acc.wrapping_sub(data) & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // Instruction: Decrement Memory
    fn dec(&self, cpu: &mut CPU) {
        let address = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu);
        let data = cpu.mem_read(address).wrapping_sub(1);
        cpu.mem_write(address, data);

        cpu.set_status_flags(data == 0, StatusFlags::ZERO);
        cpu.set_status_flags((data & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // Instruction: Decrement X Register
    fn dex(&self, cpu: &mut CPU) {
        cpu.reg_x = cpu.reg_x.wrapping_sub(1);

        cpu.set_status_flags(cpu.reg_x == 0, StatusFlags::ZERO);
        cpu.set_status_flags((cpu.reg_x & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // Instruction: Decrement Y Register
    fn dey(&self, cpu: &mut CPU) {
        cpu.reg_y = cpu.reg_y.wrapping_sub(1);

        cpu.set_status_flags(cpu.reg_y == 0, StatusFlags::ZERO);
        cpu.set_status_flags((cpu.reg_y & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // Instruction: Exclusive OR
    fn eor(&self, cpu: &mut CPU) {
        let address = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu);
        let data = cpu.mem_read(address);
        cpu.reg_acc = cpu.reg_acc ^ data;

        cpu.set_status_flags(cpu.reg_acc == 0, StatusFlags::ZERO);
        cpu.set_status_flags((cpu.reg_acc & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // Instruction: Increment Memory
    fn inc(&self, cpu: &mut CPU) {
        let address = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu);
        let data = cpu.mem_read(address).wrapping_add(1);
        cpu.mem_write(address, data);

        cpu.set_status_flags(data == 0, StatusFlags::ZERO);
        cpu.set_status_flags((data & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // Instruction: Increment X Register
    fn inx(&self, cpu: &mut CPU) {
        cpu.reg_x = cpu.reg_x.wrapping_add(1);

        cpu.set_status_flags(cpu.reg_x == 0, StatusFlags::ZERO);
        cpu.set_status_flags((cpu.reg_x & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // Instruction: Increment Y Register
    fn iny(&self, cpu: &mut CPU) {
        cpu.reg_y = cpu.reg_y.wrapping_add(1);

        cpu.set_status_flags(cpu.reg_y == 0, StatusFlags::ZERO);
        cpu.set_status_flags((cpu.reg_y & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // TODO Unofficial opcode
    fn isb(&self, cpu: &mut CPU) {
    
    }

    // Instruction: Jump
    fn jmp(&self, cpu: &mut CPU) {
        if 0x4c == cpu.mem_read(cpu.reg_pc) {
            let address = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu);
            cpu.reg_pc = address;
        } else {
            let address = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu);
            let indirect_ref = if address & 0x00FF == 0x00FF {
                let lo = cpu.mem_read(address);
                let hi = cpu.mem_read(address & 0xFF00);
                (hi as u16) << 8 | (lo as u16)
            } else {
                cpu.mem_read_u16(address)
            };
            cpu.reg_pc = indirect_ref;
        }
        // After the jump, we need to subtract by 1 byte since right after this instruction call,
        // the program counter increments. In order to run and not skip next intruction we have to do this:
        cpu.reg_pc -= 1;
    }

    // Instruction: Jump to Subroutine
    // For some reason even though JSR's addressing mode is Absolute we do not grab the address through the Aboslute addressing mode
    // Instead we do this:
    fn jsr(&self, cpu: &mut CPU) {     
        cpu.reg_pc += 1;

        cpu.stack_push_u16(cpu.reg_pc + 1);
        let target_address = cpu.mem_read_u16(cpu.reg_pc);

        // Subtract by 1 byte back because once we leave this opcode the next instruction will be incremented over
        // Therefore in order to call the next instruction after the jump, we have to subtract by 1 byte back
        cpu.reg_pc = target_address - 1;
    }

    // TODO Unofficial opcode
    fn las(&self, cpu: &mut CPU) {
    
    }

    // TODO Unofficial opcode
    fn lax(&self, cpu: &mut CPU) {
    
    }

    // Instruction: Load Accumulator
    fn lda(&self, cpu: &mut CPU) {
        let address = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu);
        let data = cpu.mem_read(address);
        cpu.reg_acc = data;

        cpu.set_status_flags(cpu.reg_acc == 0, StatusFlags::ZERO);
        cpu.set_status_flags((cpu.reg_acc & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // Instruction: Load X Register
    fn ldx(&self, cpu: &mut CPU) {
        let address = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu);
        let data = cpu.mem_read(address);
        cpu.reg_x = data;

        cpu.set_status_flags(cpu.reg_x == 0, StatusFlags::ZERO);
        cpu.set_status_flags((cpu.reg_x & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // Instruction: Load Y Register
    fn ldy(&self, cpu: &mut CPU) {
        let address = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu);
        let data = cpu.mem_read(address);
        cpu.reg_y = data;

        cpu.set_status_flags(cpu.reg_y == 0, StatusFlags::ZERO);
        cpu.set_status_flags((cpu.reg_y & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // Instruction: Logical Shift Right
    fn lsr(&self, cpu: &mut CPU) {
        // Logic only for Accumulator addressing mode
        if 0x4a == cpu.mem_read(cpu.reg_pc) {
            let data = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu) as u8;
            cpu.set_status_flags(data & 1 == 1, StatusFlags::CARRY);
            cpu.reg_acc = data >> 1;

            cpu.set_status_flags(cpu.reg_acc == 0, StatusFlags::ZERO);
            cpu.set_status_flags((cpu.reg_acc & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
        } else {
            let address = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu);
            let mut data = cpu.mem_read(address);
            cpu.set_status_flags(data & 1 == 1, StatusFlags::CARRY);
            data = data >> 1;
            cpu.mem_write(address, data);

            cpu.set_status_flags(data == 0, StatusFlags::ZERO);
            cpu.set_status_flags((data & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
        }         
    }

    // TODO Unofficial opcode
    fn lxa(&self, cpu: &mut CPU) {
    
    }

    // Instruction: No Operation
    // This includes the unofficial Double NOP: DOP
    // It grabs the address just to move forward the reg_pc but does not actually do anything
    fn nop(&self, cpu: &mut CPU) {
        let code = cpu.mem_read(cpu.reg_pc);

        match code {
            0x80 | 0x82 | 0x89 | 0xc2 | 0xe2 | 0x04 | 0x44 | 
            0x64 | 0x14 | 0x34 | 0x54 | 0x74 | 0xd4 | 0xf4 | 
            0x0c | 0x1c | 0x3c | 0x5c | 0x7c | 0xdc | 0xfc => {
                let address = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu);
                let _data = cpu.mem_read(address);
            }
            _ => {}
        }
    }

    // Instruction: Logical Inclusive OR
    fn ora(&self, cpu: &mut CPU) {
        let address = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu);
        let data = cpu.mem_read(address);
        cpu.reg_acc = cpu.reg_acc | data;
        
        cpu.set_status_flags(cpu.reg_acc == 0, StatusFlags::ZERO);
        cpu.set_status_flags((cpu.reg_acc & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // Instruction: Push Accumulator
    fn pha(&self, cpu: &mut CPU) {
        cpu.stack_push(cpu.reg_acc);
    }

    // Instruction: Push Processor Status
    fn php(&self, cpu: &mut CPU) {
        let mut flags = cpu.reg_status.clone();
        flags.insert(StatusFlags::BREAK);
        flags.insert(StatusFlags::UNUSED);
        cpu.stack_push(flags.bits());
    }
    
    // Instruction: Pull Accumulator
    fn pla(&self, cpu: &mut CPU) {
        cpu.reg_acc = cpu.stack_pop();

        cpu.set_status_flags(cpu.reg_acc == 0, StatusFlags::ZERO);
        cpu.set_status_flags((cpu.reg_acc & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // Instruction: Pull Processor Status
    fn plp(&self, cpu: &mut CPU) {
        let data = cpu.stack_pop();
        cpu.store_bitflags(data);
        cpu.reg_status.remove(StatusFlags::BREAK);
        cpu.reg_status.insert(StatusFlags::UNUSED);
    }

    // Unofficial opcode
    fn rla(&self, cpu: &mut CPU) {
        let address = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu);
        let mut data = cpu.mem_read(address);
        let carry = cpu.reg_status.contains(StatusFlags::CARRY);
        cpu.set_status_flags(data >> 7 == 1, StatusFlags::CARRY);
        data = data << 1;
        
        if carry {
            data = data | 1;
        }
        cpu.mem_write(address, data);

        cpu.set_status_flags((data & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);

        cpu.reg_acc &= data;

        cpu.set_status_flags(cpu.reg_acc == 0, StatusFlags::ZERO);
        cpu.set_status_flags((cpu.reg_acc & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // Instruction: Rotate Left
    fn rol(&self, cpu: &mut CPU) {
        if 0x2a == cpu.mem_read(cpu.reg_pc) {
            let mut data = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu) as u8;
            let carry = cpu.reg_status.contains(StatusFlags::CARRY);
            cpu.set_status_flags(data >> 7 == 1, StatusFlags::CARRY);
            data = data << 1;
            
            if carry {
                data = data | 1;
            }
            cpu.reg_acc = data;

            cpu.set_status_flags(cpu.reg_acc == 0, StatusFlags::ZERO);
            cpu.set_status_flags((cpu.reg_acc & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
        } else {
            let address = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu);
            let mut data = cpu.mem_read(address);
            let carry = cpu.reg_status.contains(StatusFlags::CARRY);
            cpu.set_status_flags(data >> 7 == 1, StatusFlags::CARRY);
            data = data << 1;
            
            if carry {
                data = data | 1;
            }
            cpu.mem_write(address, data);

            // Note: zero flag is only set for the accumulator addressing mode
            cpu.set_status_flags((data & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
        }
    }

    // Instruction: Rotate Right
    fn ror(&self, cpu: &mut CPU) {
        if 0x6a == cpu.mem_read(cpu.reg_pc) {
            let mut data = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu) as u8;
            let carry = cpu.reg_status.contains(StatusFlags::CARRY);
            cpu.set_status_flags(data & 1 == 1, StatusFlags::CARRY);
            data = data >> 1;
            
            if carry {
                data = data | 0b10000000;
            }
            cpu.reg_acc = data;

            cpu.set_status_flags(cpu.reg_acc == 0, StatusFlags::ZERO);
            cpu.set_status_flags((cpu.reg_acc & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
        } else {
            let address = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu);
            let mut data = cpu.mem_read(address);
            let carry = cpu.reg_status.contains(StatusFlags::CARRY);
            cpu.set_status_flags(data & 1 == 1, StatusFlags::CARRY);
            data = data >> 1;
            
            if carry {
                data = data | 0b10000000;
            }
            cpu.mem_write(address, data);

            // Note: zero flag is only set for the accumulator addressing mode
            cpu.set_status_flags((data & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
        }
    }

    // TODO Unofficial opcode
    fn rra(&self, cpu: &mut CPU) {
    
    }

    // Instruction: Return from Interrupt
    fn rti(&self, cpu: &mut CPU) {
        let data = cpu.stack_pop();
        cpu.store_bitflags(data);
        cpu.reg_status.remove(StatusFlags::BREAK);
        cpu.reg_status.insert(StatusFlags::UNUSED);

        cpu.reg_pc = cpu.stack_pop_u16();
    }

    // Instruction: Return from Subroutine
    fn rts(&self, cpu: &mut CPU) {
        // Considering that we pushed the Return address 1 byte during the subroutine call
        // we wouldnt need to offset return address by 1
        cpu.reg_pc = cpu.stack_pop_u16();
    }

    // TODO Unofficial opcode
    fn sax(&self, cpu: &mut CPU) {
    
    }

    // Instruction: Subtract with Carry
    fn sbc(&self, cpu: &mut CPU) {
        let address = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu);
        let mut data = cpu.mem_read(address);
        data = (data as i8).wrapping_neg().wrapping_sub(1) as u8;

        let sum = cpu.reg_acc as u16
            + data as u16
            + (if cpu.reg_status.contains(StatusFlags::CARRY) {
                1
            } else {
                0
            }) as u16;

        let carry = sum > 0xff;

        if carry {
            cpu.reg_status.insert(StatusFlags::CARRY);
        } else {
            cpu.reg_status.remove(StatusFlags::CARRY);
        }

        let result = sum as u8;

        if (data ^ result) & (result ^ cpu.reg_acc) & 0x80 != 0 {
            cpu.reg_status.insert(StatusFlags::OVERFLOW);
        } else {
            cpu.reg_status.remove(StatusFlags::OVERFLOW)
        }
        cpu.reg_acc = result;
        
        cpu.set_status_flags(cpu.reg_acc == 0, StatusFlags::ZERO);
        cpu.set_status_flags((cpu.reg_acc & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // Instruction: Set Carry Flag
    fn sec(&self, cpu: &mut CPU) {
        cpu.reg_status.insert(StatusFlags::CARRY);
    }

    // Instruction: Set Decimal Flag
    fn sed(&self, cpu: &mut CPU) {
        cpu.reg_status.insert(StatusFlags::DECIMAL);   
    }

    // Instruction: Set Interrupt Disable
    fn sei(&self, cpu: &mut CPU) {
        cpu.reg_status.insert(StatusFlags::INTERRUPT);   
    }

    // TODO Unofficial opcode
    fn shx(&self, cpu: &mut CPU) {
        // let address = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu);
        // let data = cpu.mem_read(address);
        // cpu.reg_x = data;

        // cpu.set_status_flags(cpu.reg_x == 0, StatusFlags::ZERO);
        // cpu.set_status_flags((cpu.reg_x & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // TODO Unofficial opcode
    fn shy(&self, cpu: &mut CPU) {
    
    }

    // Unofficial opcode
    fn slo(&self, cpu: &mut CPU) {
        let address = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu);
        let mut data = cpu.mem_read(address);
        cpu.set_status_flags(data >> 7 == 1, StatusFlags::CARRY);
        data = data << 1;
        cpu.mem_write(address, data);

        cpu.set_status_flags(data == 0, StatusFlags::ZERO);
        cpu.set_status_flags((data & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);

        cpu.reg_acc |= data;

        cpu.set_status_flags(cpu.reg_acc == 0, StatusFlags::ZERO);
        cpu.set_status_flags((cpu.reg_acc & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // Unofficial opcode
    fn sre(&self, cpu: &mut CPU) {
        let address = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu);
        let mut data = cpu.mem_read(address);
        cpu.set_status_flags(data & 1 == 1, StatusFlags::CARRY);
        data = data >> 1;
        cpu.mem_write(address, data);

        cpu.set_status_flags(data == 0, StatusFlags::ZERO);
        cpu.set_status_flags((data & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);

        cpu.reg_acc ^= data;

        cpu.set_status_flags(cpu.reg_acc == 0, StatusFlags::ZERO);
        cpu.set_status_flags((cpu.reg_acc & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // Instruction: Store Accumulator
    fn sta(&self, cpu: &mut CPU) {
        let address = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu);
        cpu.mem_write(address, cpu.reg_acc);
    }

    // Instruction: Store X Register
    fn stx(&self, cpu: &mut CPU) {
        let address = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu);
        cpu.mem_write(address, cpu.reg_x);
    }

    // Instruction: Store Y Register
    fn sty(&self, cpu: &mut CPU) {
        let address = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu);
        cpu.mem_write(address, cpu.reg_y);
    }

    // Unofficial opcode
    fn tas(&self, cpu: &mut CPU) {
        let address = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu) + (cpu.reg_y as u16);
        cpu.reg_stack_ptr = cpu.reg_acc & cpu.reg_x;
        let data = ((address >> 8) as u8 + 1) & cpu.reg_stack_ptr;
        cpu.mem_write(address, data);
    }

    // Instruction: Transfer Accumulator to X
    fn tax(&self, cpu: &mut CPU) {
        cpu.reg_x = cpu.reg_acc;

        cpu.set_status_flags(cpu.reg_x == 0, StatusFlags::ZERO);
        cpu.set_status_flags((cpu.reg_x & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // Instruction: Transfer Accumulator to Y
    fn tay(&self, cpu: &mut CPU) {
        cpu.reg_y = cpu.reg_acc;

        cpu.set_status_flags(cpu.reg_y == 0, StatusFlags::ZERO);
        cpu.set_status_flags((cpu.reg_y & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // Instruction: Transfer Stack Pointer to X
    fn tsx(&self, cpu: &mut CPU) {
        cpu.reg_x = cpu.reg_stack_ptr;

        cpu.set_status_flags(cpu.reg_x == 0, StatusFlags::ZERO);
        cpu.set_status_flags((cpu.reg_x & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // Instruction: Transfer X to Accumulator
    fn txa(&self, cpu: &mut CPU) {
        cpu.reg_acc = cpu.reg_x;

        cpu.set_status_flags(cpu.reg_acc == 0, StatusFlags::ZERO);
        cpu.set_status_flags((cpu.reg_acc & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // Instruction: Transfer X to Stack Pointer
    fn txs(&self, cpu: &mut CPU) {
        cpu.reg_stack_ptr = cpu.reg_x;
    }

    // Instruction: Transfer Y to Accumulator
    fn tya(&self, cpu: &mut CPU) {
        cpu.reg_acc = cpu.reg_y;

        cpu.set_status_flags(cpu.reg_acc == 0, StatusFlags::ZERO);
        cpu.set_status_flags((cpu.reg_acc & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }

    // Unofficial opcode
    fn xaa(&self, cpu: &mut CPU) {
        cpu.reg_acc = cpu.reg_x;

        cpu.set_status_flags(cpu.reg_acc == 0, StatusFlags::ZERO);
        cpu.set_status_flags((cpu.reg_acc & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);

        let address = self.get_address(cpu.mem_read(cpu.reg_pc))(cpu);
        let data = cpu.mem_read(address);
        cpu.reg_acc &= data;

        cpu.set_status_flags(cpu.reg_acc == 0, StatusFlags::ZERO);
        cpu.set_status_flags((cpu.reg_acc & StatusFlags::NEGATIVE.bits()) != 0, StatusFlags::NEGATIVE);
    }
}


