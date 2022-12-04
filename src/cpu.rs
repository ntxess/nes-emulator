// Implementation of NES 6502 CPU
// https://www.nesdev.org/obelisk-6502-guide/reference.html
// http://www.6502.org/tutorials/6502opcodes.html
// https://www.nesdev.org/wiki/Nesdev_Wiki
// https://www.nesdev.org/NESDoc.pdf

// Temporary attributes
#![allow(non_snake_case)] 
#![allow(dead_code)] 

pub struct CPU {
    pub reg_pc:     u16,
    pub reg_stkptr: u8,
    pub reg_acc:    u8,
    pub reg_x:      u8,
    pub reg_y:      u8,
    pub reg_status: u8,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            reg_pc:     0x0000,
            reg_stkptr: 0x00,
            reg_acc:    0x00,
            reg_x:      0x00,
            reg_y:      0x00,
            reg_status: 0x00,
        }
    }



       
    // Addressing Modes
    
    // Addressing Mode: Implicit
    // For many 6502 instructions the source and destination of the information to be 
    // manipulated is implied directly by the function of the instruction itself and no 
    // further operand needs to be specified. Operations like 'Clear Carry Flag' (CLC) and 
    //'Return from Subroutine' (RTS) are implicit.
    fn IMP() -> u8 {
        return 0;
    }

    // Addressing Mode: Immediate
    // Immediate addressing allows the programmer to directly specify an 8 bit constant 
    // within the instruction. It is indicated by a '#' symbol followed by an numeric expression.
    fn IMM() -> u8 {
        return 0;
    }

    // Addressing Mode: Zero Page
    // An instruction using zero page addressing mode has only an 8 bit address operand. 
    // This limits it to addressing only the first 256 bytes of memory (e.g. $0000 to $00FF) 
    // where the most significant byte of the address is always zero. In zero page mode only 
    // the least significant byte of the address is held in the instruction making it shorter 
    // by one byte (important for space saving) and one less memory fetch during execution 
    // (important for speed).
	fn ZP0() -> u8 {
        return 0;
    }		

    // Addressing Mode: Zero Page, X
    // The address to be accessed by an instruction using indexed zero page addressing is 
    // calculated by taking the 8 bit zero page address from the instruction and adding the 
    // current value of the X register to it.
    fn ZPX() -> u8 {
        return 0;
    }

    // Addressing Mode: Zero Page, Y
    // The address to be accessed by an instruction using indexed zero page addressing is 
    // calculated by taking the 8 bit zero page address from the instruction and adding the 
    // current value of the Y register to it. This mode can only be used with the LDX and STX 
    // instructions.
	fn ZPY() -> u8 {
        return 0;
    }

    // Addressing Mode: Relative
    // Relative addressing mode is used by branch instructions (e.g. BEQ, BNE, etc.) which 
    // contain a signed 8 bit relative offset (e.g. -128 to +127) which is added to program 
    // counter if the condition is true. As the program counter itself is incremented during 
    // instruction execution by two the effective address range for the target instruction 
    // must be with -126 to +129 bytes of the branch.
    fn REL() -> u8 {
        return 0;
    }

    // Addressing Mode: Absolute
    // Instructions using absolute addressing contain a full 16 bit address to identify the 
    // target location. 
	fn ABS() -> u8 {
        return 0;
    }

    // Addressing Mode: Absolute, Y
    // The address to be accessed by an instruction using X register indexed absolute addressing 
    // is computed by taking the 16 bit address from the instruction and added the contents of 
    // the X register. For example if X contains $92 then an STA $2000,X instruction will store 
    // the accumulator at $2092 (e.g. $2000 + $92).
    fn ABX() -> u8 {
        return 0;
    }

    // Addressing Mode: Absolute, X
    // The Y register indexed absolute addressing mode is the same as the previous mode only with 
    // the contents of the Y register added to the 16 bit address from the instruction.
	fn ABY() -> u8 {
        return 0;
    }	

    // Addressing Mode: Indirect
    // JMP is the only 6502 instruction to support indirection. The instruction contains a 16 bit 
    // address which identifies the location of the least significant byte of another 16 bit memory 
    // address which is the real target of the instruction.
    fn IND() -> u8 {
        return 0;
    }

    // Addressing Mode: Index Indirect 
    // Indexed indirect addressing is normally used in conjunction with a table of address held on 
    // zero page. The address of the table is taken from the instruction and the X register added to 
    // it (with zero page wrap around) to give the location of the least significant byte of the 
    // target address.
    fn IZX() -> u8 {
        return 0;
    }

    // Addressing Mode: Indirect Indexed
    // Indirect indirect addressing is the most common indirection mode used on the 6502. In 
    // instruction contains the zero page location of the least significant byte of 16 bit address. 
    // The Y register is dynamically added to this value to generated the actual target address for 
    // operation.
    fn IZY() -> u8 {
        return 0;
    }

    // Instructions
    
    // Instruction: Add with Carry
    fn ADC() -> u8 {
        return 0;
    }

    // Instruction: Logic AND
    fn AND() -> u8 {
        return 0;
    }

    // Instruction: Arithmetic Shift Left
    fn ASL() -> u8 {
        return 0;
    }

    // Instruction: Branch if Carry Clear
    fn BCC() -> u8 {
        return 0;
    }

    // Instruction:  Branch if Carrt Set
	fn BCS() -> u8 {
        return 0;
    }

    // Instruction: Branch if Equal
    fn BEQ() -> u8 {
        return 0;
    }

    // Instruction: Bit Test
    fn BIT() -> u8 {
        return 0;
    }

    // Instruction: Branch if Minus
    fn BMI() -> u8 {
        return 0;
    }

    // Instruction: Branch if Not Equal
	fn BNE() -> u8 {
        return 0;
    }

    // Instruction: Branch if Positive
    fn BPL() -> u8 {
        return 0;
    }

    // Instruction: Force Interrupt
    fn BRK() -> u8 {
        return 0;
    }

    // Instruction: Branch if Overflow Clear
    fn BVC() -> u8 {
        return 0;
    }

    // Instruction: Branch Carry Flag
	fn BVS() -> u8 {
        return 0;
    }

    // Instruction: Clear Carry Flag
    fn CLC() -> u8 {
        return 0;
    }

    // Instruction: Clear Decimal Mode
    fn CLD() -> u8 {
        return 0;
    }

    // Instruction: Clear Interrupt Disable
    fn CLI() -> u8 {
        return 0;
    }

    // Instruction:  Clear Overflow Flag
	fn CLV() -> u8 {
        return 0;
    }

    // Instruction: Compare
    fn CMP() -> u8 {
        return 0;
    }

    // Instruction: Compare X Register
    fn CPX() -> u8 {
        return 0;
    }

    // Instruction: Compare Y Register
    fn CPY() -> u8 {
        return 0;
    }

    // Instruction: Decrement Memory
	fn DEC() -> u8 {
        return 0;
    }

    // Instruction: Decrement X Register
    fn DEX() -> u8 {
        return 0;
    }

    // Instruction: Decrement Y Register
    fn DEY() -> u8 {
        return 0;
    }

    // Instruction: Exclusive OR
    fn EOR() -> u8 {
        return 0;
    }

    // Instruction: Increment Memory
	fn INC() -> u8 {
        return 0;
    }

    // Instruction: Increment X Register
    fn INX() -> u8 {
        return 0;
    }

    // Instruction: Increment Y Register
    fn INY() -> u8 {
        return 0;
    }

    // Instruction: Jump
    fn JMP() -> u8 {
        return 0;
    }

    // Instruction: Jump to Subroutine
	fn JSR() -> u8 {
        return 0;
    }

    // Instruction: Load Accumulator
    fn LDA() -> u8 {
        return 0;
    }

    // Instruction: Load X Register
    fn LDX() -> u8 {
        return 0;
    }

    // Instruction: Load Y Register
    fn LDY() -> u8 {
        return 0;
    }

    // Instruction: Logical Shift Right
	fn LSR() -> u8 {
        return 0;
    }

    // Instruction: No Operation
    fn NOP() -> u8 {
        return 0;
    }

    // Instruction: Logical Inclusive OR
    fn ORA() -> u8 {
        return 0;
    }

    // Instruction: Push Accumulator
    fn PHA() -> u8 {
        return 0;
    }

    // Instruction: Push Processor Status
	fn PHP() -> u8 {
        return 0;
    }
    
    // Instruction: Pull Accumulator
    fn PLA() -> u8 {
        return 0;
    }

    // Instruction: Pull Processor Status
    fn PLP() -> u8 {
        return 0;
    }

    // Instruction: Rotate Left
    fn ROL() -> u8 {
        return 0;
    }

    // Instruction: Rotate Right
	fn ROR() -> u8 {
        return 0;
    }

    // Instruction: Return from Interrupt
    fn RTI() -> u8 {
        return 0;
    }

    // Instruction: Return from Subroutine
    fn RTS() -> u8 {
        return 0;
    }

    // Instruction: Subtract with Carry
    fn SBC() -> u8 {
        return 0;
    }

    // Instruction: Set Carry Flag
	fn SEC() -> u8 {
        return 0;
    }

    // Instruction: Set Decimal Flag
    fn SED() -> u8 {
        return 0;
    }

    // Instruction: Set Interrupt Disable
    fn SEI() -> u8 {
        return 0;
    }

    // Instruction: Store Accumulator
    fn STA() -> u8 {
        return 0;
    }

    // Instruction: Store X Register
	fn STX() -> u8 {
        return 0;
    }

    // Instruction: Store Y Register
    fn STY() -> u8 {
        return 0;
    }

    // Instruction: Transfer Accumulator to X
    fn TAX() -> u8 {
        return 0;
    }

    // Instruction: Transfer Accumulator to Y
    fn TAY() -> u8 {
        return 0;
    }

    // Instruction: Transfer Stack Pointer to X
	fn TSX() -> u8 {
        return 0;
    }

    // Instruction: Transfer X to Accumulator
    fn TXA() -> u8 {
        return 0;
    }

    // Instruction: Transfer X to Stack Pointer
    fn TXS() -> u8 {
        return 0;
    }

    // Instruction: Transfer Y to Accumulator
    fn TYA() -> u8 {
        return 0;
    }
}