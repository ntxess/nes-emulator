use crate::opcodes::InstructionSet;
use crate::bus::Bus;

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
    pub bus:           Bus,
}

pub trait Mem {
    fn mem_read(&self, addr: u16) -> u8;

    fn mem_write(&mut self, addr: u16, data: u8);

    fn mem_read_u16(&self, pos: u16) -> u16 {
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
}

impl Mem for CPU {
    fn mem_read(&self, addr: u16) -> u8 {
        self.bus.mem_read(addr)
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.bus.mem_write(addr, data)
    }

    fn mem_read_u16(&self, pos: u16) -> u16 {
        self.bus.mem_read_u16(pos)
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        self.bus.mem_write_u16(pos, data)
    }
}

impl CPU {
    pub fn new(bus: Bus) -> Self {
        CPU {
            reg_pc:        0,
            reg_acc:       0,
            reg_x:         0,
            reg_y:         0,
            reg_stack_ptr: STACK_RESET,
            reg_status:    StatusFlags::from_bits_truncate(0b100100),
            bus:           bus,
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
        for i in 0..(program.len() as u16) {
            self.mem_write(0x0600 + i, program[i as usize]);
        }
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
            callback(self);
            let code = self.mem_read(self.reg_pc);
            
            matrix.call_opcode(code)(&matrix, self);
            self.reg_pc += 1;
        }
    }
}