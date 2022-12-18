use crate::opcodes::InstructionSet;

bitflags! {
    pub struct StatusFlags: u8 {
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

const STACK: u16 = 0x0100;
const STACK_RESET: u8 = 0xfd;

pub struct CPU {
    pub reg_pc:        u16,
    pub reg_acc:       u8,
    pub reg_x:         u8,
    pub reg_y:         u8,
    pub reg_stack_ptr: u8,
    pub reg_status:    StatusFlags,
    memory:           [u8; 0xFFFF],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            reg_pc:        0,
            reg_acc:       0,
            reg_x:         0,
            reg_y:         0,
            reg_stack_ptr: STACK_RESET,
            reg_status:    StatusFlags::from_bits_truncate(0b100100),
            memory:       [0; 0xFFFF],
        }
    }

    pub fn set_status_flags(&mut self, condition: bool, flag: StatusFlags) {
        if condition {
            self.reg_status.insert(flag);
        } else {
            self.reg_status.remove(flag);
        }
    }

    pub fn store_bitflags(&mut self, data: u8) {
        self.reg_status.bits = data;
    }

    // Auxiliary Function referenced from nes_ebook by bugzmanov
    pub fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    pub fn mem_read_u16(&self, pos: u16) -> u16 {
        let lo = self.mem_read(pos) as u16;
        let hi = self.mem_read(pos + 1) as u16;
        (hi << 8) | (lo as u16)
    }

    pub fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.mem_write(pos, lo);
        self.mem_write(pos + 1, hi);
    }

    pub fn stack_pop(&mut self) -> u8 {
        self.reg_stack_ptr = self.reg_stack_ptr.wrapping_add(1);
        self.mem_read((STACK as u16) + self.reg_stack_ptr as u16)
    }

    pub fn stack_push(&mut self, data: u8) {
        self.mem_write((STACK as u16) + self.reg_stack_ptr as u16, data);
        self.reg_stack_ptr = self.reg_stack_ptr.wrapping_sub(1);
    }

    pub fn stack_pop_u16(&mut self) -> u16 {
        let lo = self.stack_pop() as u16;
        let hi = self.stack_pop() as u16;

        hi << 8 | lo
    }

    pub fn stack_push_u16(&mut self, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.stack_push(hi);
        self.stack_push(lo);
    }

    pub fn reset(&mut self) {
        self.reg_acc = 0;
        self.reg_x = 0;
        self.reg_y = 0;
        self.reg_stack_ptr = STACK_RESET;
        self.reg_status = StatusFlags::from_bits_truncate(0b100100);

        // Reset program counter to the start of program ROM
        self.reg_pc = self.mem_read_u16(0xFFFC);
    }
    
    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn load(&mut self, program: Vec<u8>) {
        // Memory location [0x8000 .. 0xFFFF] is reserved for program ROM
        // Copy program ROM into NES memory starting from position 0x8000
        // Write reference of the start of program ROM to NES memory position 0xFFFC
        self.memory[0x0600..(0x0600 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x0600);
    }

    pub fn run(&mut self) {
        self.run_with_callback(|_| {});
    }
    
    pub fn run_with_callback<F>(&mut self, mut callback: F) 
    where 
    F: FnMut(&mut CPU),
    {
        let matrix = InstructionSet::new();
        
        while self.mem_read(self.reg_pc) != 0x00 {
            let code = self.mem_read(self.reg_pc);

            matrix.call_opcode(code)(&matrix, self);
            self.reg_pc += 1;

            callback(self);
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
        assert!(cpu.reg_status.bits() & 0b0000_0010 == 0b00);
        assert!(cpu.reg_status.bits() & 0b1000_0000 == 0);
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