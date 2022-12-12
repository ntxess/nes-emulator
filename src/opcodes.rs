use crate::cpu::CPU;

pub struct Instruction {
    opcode:   Box<dyn Fn(&InstructionSet, &mut CPU)>,
    addrmode: Box<dyn Fn(&mut CPU)>,
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

    // Auxillary Functions
    pub fn get_opcode(&self, code: usize) -> &Box<dyn Fn(&InstructionSet, &mut CPU)> {
        return &self.matrix[code].opcode;
    }

    pub fn get_addrmode(&self, code: usize) -> &Box<dyn Fn(&mut CPU)> {
        return &self.matrix[code].addrmode;
    }

    pub fn get_cycle(&self, code: usize) -> u8{
        return self.matrix[code].cycle;
    }

    // Addressing Mode: Implicit
    fn imp(_cpu: &mut CPU) {
        //_cpu.fetched = _cpu.reg_acc;
    }

    // Addressing Mode: Immediate
    fn imm(_cpu: &mut CPU) {
        _cpu.reg_pc += 1;

        _cpu.fetched = _cpu.memory[(_cpu.reg_pc) as usize];
    }

    // Addressing Mode: Zero Page
	fn zp0(_cpu: &mut CPU) {
        _cpu.reg_pc += 1;

        let address = _cpu.memory[(_cpu.reg_pc) as usize] as u16;

        _cpu.fetched = _cpu.memory[(address) as usize];
    }		

    // Addressing Mode: Zero Page, X
    fn zpx(_cpu: &mut CPU) {
        _cpu.reg_pc += 1;

        let pos = _cpu.memory[(_cpu.reg_pc) as usize];
        let address = pos.wrapping_add(_cpu.reg_x) as u16;

        _cpu.fetched = _cpu.memory[(address) as usize];
    }

    // Addressing Mode: Zero Page, Y
	fn zpy(_cpu: &mut CPU) {
        _cpu.reg_pc += 1;

        let pos = _cpu.memory[(_cpu.reg_pc) as usize];
        let address = pos.wrapping_add(_cpu.reg_y) as u16;

        _cpu.fetched = _cpu.memory[(address) as usize];
    }

    // Addressing Mode: Relative
    fn rel(_cpu: &mut CPU) {
        // _cpu.reg_pc += 1;
    }

    // Addressing Mode: Absolute
	fn abs(_cpu: &mut CPU) {
        // _cpu.reg_pc += 1;
    }

    // Addressing Mode: Absolute, X
    fn abx(_cpu: &mut CPU) {
        _cpu.reg_pc += 1;

        let base = _cpu.mem_read_u16(_cpu.reg_pc);
        let address = base.wrapping_add(_cpu.reg_x as u16);

        _cpu.fetched = _cpu.memory[(address) as usize];

    }

    // Addressing Mode: Absolute, Y
	fn aby(_cpu: &mut CPU) {
        _cpu.reg_pc += 1;

        let base = _cpu.mem_read_u16(_cpu.reg_pc);
        let address = base.wrapping_add(_cpu.reg_y as u16);

        _cpu.fetched = _cpu.memory[(address) as usize];
    }	

    // Addressing Mode: Indirect
    fn ind(_cpu: &mut CPU) {
        // _cpu.reg_pc += 1;

    }

    // Addressing Mode: Indirect Indexed X
    fn izx(_cpu: &mut CPU) {
        _cpu.reg_pc += 1;

        let base = _cpu.mem_read(_cpu.reg_pc);
        let ptr: u8 = (base as u8).wrapping_add(_cpu.reg_x);
        let lo = _cpu.mem_read(ptr as u16);
        let hi = _cpu.mem_read(ptr.wrapping_add(1) as u16);
        let address = (hi as u16) << 8 | (lo as u16);

        _cpu.fetched = _cpu.memory[(address) as usize];
    }

    // Addressing Mode: Indirect Indexed Y
    fn izy(_cpu: &mut CPU) {
        _cpu.reg_pc += 1;

        let base = _cpu.mem_read(_cpu.reg_pc);

        let ptr: u8 = (base as u8).wrapping_add(_cpu.reg_y);
        let lo = _cpu.mem_read(ptr as u16);
        let hi = _cpu.mem_read(ptr.wrapping_add(1) as u16);
        let address = (hi as u16) << 8 | (lo as u16);

        _cpu.fetched = _cpu.memory[(address) as usize];
    }

    // Instruction: Add with Carry
    fn adc(&self, _cpu: &mut CPU) {

    }

    // Instruction: Logic AND
    fn and(&self, _cpu: &mut CPU) {
        self.get_addrmode(_cpu.memory[_cpu.reg_pc as usize] as usize)(_cpu);

        let data = _cpu.mem_read(_cpu.fetched as u16);
        _cpu.reg_acc = _cpu.reg_acc & data;

        if _cpu.reg_acc == 0 {
            _cpu.reg_status = _cpu.reg_status | 0b0000_0010;
        } else {
            _cpu.reg_status = _cpu.reg_status & 0b1111_1101;
        }

        if _cpu.reg_acc & 0b1000_0000 != 0 {
            _cpu.reg_status = _cpu.reg_status | 0b1000_0000;
        } else {
            _cpu.reg_status = _cpu.reg_status & 0b0111_1111;
        }
    }

    // Instruction: Arithmetic Shift Left
    fn asl(&self, _cpu: &mut CPU) {

    }

    // Instruction: Branch if Carry Clear
    fn bcc(&self, _cpu: &mut CPU) {

    }

    // Instruction:  Branch if Carrt Set
	fn bcs(&self, _cpu: &mut CPU) {

    }

    // Instruction: Branch if Equal
    fn beq(&self, _cpu: &mut CPU) {

    }

    // Instruction: Bit Test
    fn bit(&self, _cpu: &mut CPU) {

    }

    // Instruction: Branch if Minus
    fn bmi(&self, _cpu: &mut CPU) {

    }

    // Instruction: Branch if Not Equal
	fn bne(&self, _cpu: &mut CPU) {

    }

    // Instruction: Branch if Positive
    fn bpl(&self, _cpu: &mut CPU) {

    }

    // Instruction: Force Interrupt
    fn brk(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Branch if Overflow Clear
    fn bvc(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Branch Carry Flag
	fn bvs(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Clear Carry Flag
    fn clc(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Clear Decimal Mode
    fn cld(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Clear Interrupt Disable
    fn cli(&self, _cpu: &mut CPU) {
         
    }

    // Instruction:  Clear Overflow Flag
	fn clv(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Compare
    fn cmp(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Compare X Register
    fn cpx(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Compare Y Register
    fn cpy(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Decrement Memory
	fn dec(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Decrement X Register
    fn dex(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Decrement Y Register
    fn dey(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Exclusive OR
    fn eor(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Increment Memory
	fn inc(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Increment X Register
    fn inx(&self, _cpu: &mut CPU) {
        _cpu.reg_x = _cpu.reg_x.wrapping_add(1);

        if _cpu.reg_acc == 0 {
            _cpu.reg_status = _cpu.reg_status | 0b0000_0010;
        } else {
            _cpu.reg_status = _cpu.reg_status & 0b1111_1101;
        }

        if _cpu.reg_acc & 0b1000_0000 != 0 {
            _cpu.reg_status = _cpu.reg_status | 0b1000_0000;
        } else {
            _cpu.reg_status = _cpu.reg_status & 0b0111_1111;
        }
    }

    // Instruction: Increment Y Register
    fn iny(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Jump
    fn jmp(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Jump to Subroutine
	fn jsr(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Load Accumulator
    fn lda(&self, _cpu: &mut CPU) {
        self.get_addrmode(_cpu.memory[_cpu.reg_pc as usize] as usize)(_cpu);
        _cpu.reg_acc = _cpu.fetched;

        if _cpu.reg_acc == 0 {
            _cpu.reg_status = _cpu.reg_status | 0b0000_0010;
        } else {
            _cpu.reg_status = _cpu.reg_status & 0b1111_1101;
        }

        if _cpu.reg_acc & 0b1000_0000 != 0 {
            _cpu.reg_status = _cpu.reg_status | 0b1000_0000;
        } else {
            _cpu.reg_status = _cpu.reg_status & 0b0111_1111;
        }
    }

    // Instruction: Load X Register
    fn ldx(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Load Y Register
    fn ldy(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Logical Shift Right
	fn lsr(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: No Operation
    fn nop(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Logical Inclusive OR
    fn ora(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Push Accumulator
    fn pha(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Push Processor Status
	fn php(&self, _cpu: &mut CPU) {
         
    }
    
    // Instruction: Pull Accumulator
    fn pla(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Pull Processor Status
    fn plp(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Rotate Left
    fn rol(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Rotate Right
	fn ror(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Return from Interrupt
    fn rti(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Return from Subroutine
    fn rts(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Subtract with Carry
    fn sbc(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Set Carry Flag
	fn sec(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Set Decimal Flag
    fn sed(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Set Interrupt Disable
    fn sei(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Store Accumulator
    fn sta(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Store X Register
	fn stx(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Store Y Register
    fn sty(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Transfer Accumulator to X
    fn tax(&self, _cpu: &mut CPU) {
        _cpu.reg_x = _cpu.reg_acc;
            
        if _cpu.reg_x == 0 {
            _cpu.reg_status = _cpu.reg_status | 0b0000_0010;
        } else {
            _cpu.reg_status = _cpu.reg_status & 0b1111_1101;
        }

        if _cpu.reg_x & 0b1000_0000 != 0 {
            _cpu.reg_status = _cpu.reg_status | 0b1000_0000;
        } else {
            _cpu.reg_status = _cpu.reg_status & 0b0111_1111;
        }
    }

    // Instruction: Transfer Accumulator to Y
    fn tay(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Transfer Stack Pointer to X
	fn tsx(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Transfer X to Accumulator
    fn txa(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Transfer X to Stack Pointer
    fn txs(&self, _cpu: &mut CPU) {
         
    }

    // Instruction: Transfer Y to Accumulator
    fn tya(&self, _cpu: &mut CPU) {
         
    }

    fn xxx(&self, _cpu: &mut CPU) {
         
    }
}