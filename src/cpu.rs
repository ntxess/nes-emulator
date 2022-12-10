// Implementation of NES 6502 CPU
// https://www.nesdev.org/obelisk-6502-guide/reference.html
// http://www.6502.org/tutorials/6502opcodes.html
// https://www.nesdev.org/wiki/Nesdev_Wiki
// https://www.nesdev.org/NESDoc.pdf
// Following the RUST NES Guide by github:bugzmanov
// https://github.com/bugzmanov/nes_ebook

use crate::opcodes::InstructionSet;
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

pub struct CPU {
    pub reg_pc:        u16,
    pub reg_stack_ptr: u8,
    pub reg_acc:       u8,
    pub reg_x:         u8,
    pub reg_y:         u8,
    pub reg_status:    u8,
    pub fetched:       u8,
    memory:            [u8; 0xFFFF],
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
            fetched:       0x00,
            memory:        [0; 0xFFFF],
        }
    }

    // Auxiliary Function
    pub fn interpret(&mut self, program: &[u8]) {
        let mut matrix = InstructionSet::new();
        self.reg_pc = 0;

        while (self.reg_pc as usize) < program.len() {
            matrix.get_addrmode(program[self.reg_pc as usize] as usize)(self, program);
            matrix.get_opcode(program[self.reg_pc as usize] as usize)(self);
            matrix.get_cycle(program[self.reg_pc as usize] as usize);
            self.reg_pc += 1;

        }
    }

    pub fn reset(&mut self) {
        self.reg_acc = 0;
        self.reg_x = 0;
        self.reg_status = 0;

        self.memory[0] = 0; 

        // self.reg_pc = self.mem_read_u16(0xFFFC);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
 
    #[test]
    fn test_0xa9_lda_immidiate_load_data() {
        let mut cpu = CPU::new();

        cpu.interpret(&[0xa9, 0x05, 0x00]);
        assert_eq!(cpu.reg_acc, 5);
        assert!(cpu.reg_status & 0b0000_0010 == 0);
        assert!(cpu.reg_status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();

        cpu.interpret(&[0xa9, 0x00, 0x00]);
        assert!(cpu.reg_status & 0b0000_0010 == 0b10);
    }
    
    #[test]
    fn test_0xa9_lda_negative_flag() {
        let mut cpu = CPU::new();

        cpu.interpret(&[0xa9, 0xff, 0x00]);
        assert!(cpu.reg_status & 0b1000_0000 == 0b1000_0000);

    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();

        cpu.reg_acc = 10;
        cpu.interpret(&[0xaa, 0x00]);

        assert_eq!(cpu.reg_x, 10)
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();

        cpu.interpret(&[0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.reg_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();

        cpu.reg_x = 0xff;
        cpu.interpret(&[0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.reg_x, 1)
    }

    #[test]
    fn test_individual() {
        let mut cpu = CPU::new();

        cpu.interpret(&[0xa9, 0xc0, 0x00]);
        assert_eq!(cpu.reg_acc, 0xc1)
    }
}