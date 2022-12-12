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
    pub memory:       [u8; 0xFFFF],
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
            memory:       [0; 0xFFFF],
        }
    }

    // Auxiliary Function
    pub fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    pub fn mem_read_u16(&self, pos: u16) -> u16 {
        let lo = self.mem_read(pos) as u16;
        let hi = self.mem_read(pos + 1) as u16;
        (hi << 8) | (lo as u16)
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.mem_write(pos, lo);
        self.mem_write(pos + 1, hi);
    }

    pub fn reset(&mut self) {
        self.reg_acc = 0;
        self.reg_x = 0;
        self.reg_status = 0;

        // Reset program counter to the start of program ROM
        self.reg_pc = self.mem_read_u16(0xFFFC);
    }
    
    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.run();
    }

    pub fn load(&mut self, program: Vec<u8>) {
        // Memory location [0x8000 .. 0xFFFF] is reserved for program ROM
        // Copy program ROM into NES memory starting from position 0x8000
        // Write reference of the start of program ROM to NES memory position 0xFFFC
        self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    pub fn run(&mut self) {
        let matrix = InstructionSet::new();

        while (self.reg_pc as usize) < self.memory.len() {
            matrix.get_opcode(self.memory[self.reg_pc as usize] as usize)(&matrix, self);
            //matrix.get_cycle(self.memory[self.reg_pc as usize] as usize);
            self.reg_pc += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
 
    #[test]
    fn test_0xa9_lda_immidiate_load_data() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.reg_acc, 5);
        assert!(cpu.reg_status & 0b0000_0010 == 0);
        assert!(cpu.reg_status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.reg_status & 0b0000_0010 == 0b10);
    }
    
    #[test]
    fn test_0xa9_lda_negative_flag() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xa9, 0xff, 0x00]);
        assert!(cpu.reg_status & 0b1000_0000 == 0b1000_0000);

    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();

        cpu.reg_acc = 10;
        cpu.load_and_run(vec![0xaa, 0x00]);
        assert_eq!(cpu.reg_x, 10)
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
        assert_eq!(cpu.reg_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();

        cpu.reg_x = 0xff;
        cpu.load_and_run(vec![0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.reg_x, 1)
    }

    #[test]
    fn test_lda_from_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);

        cpu.load_and_run(vec![0xa5, 0x10, 0x00]);
        assert_eq!(cpu.reg_acc, 0x55);
    }
}