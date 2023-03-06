use crate::cpu::Mem;
use crate::cpu::CPU;
use crate::debugcodes;
use crate::debugcodes::AddressingMode;

pub fn trace(cpu: &mut CPU) -> String {
    let ref opscodes: Vec<debugcodes::OpCodes> = *debugcodes::DEBUG_OPCODES;

    let code = cpu.mem_read(cpu.reg_pc);
    let ops = &opscodes[code as usize];

    let begin = cpu.reg_pc;
    let mut hex_dump = vec![];
    hex_dump.push(code);

    let tmp = match ops.len {
        1 => match code {
            0x0a | 0x4a | 0x2a | 0x6a => format!("A "),
            _ => String::from(""),
        },
        2 => {
            let address = cpu.mem_read(begin + 1);
            match ops.mode {
                AddressingMode::Immediate => {
                    hex_dump.push(address);
                    format!("#${:02x}", address)
                }

                AddressingMode::ZeroPage => {
                    let stored_value = cpu.mem_read(address as u16);

                    hex_dump.push(address);
                    format!("${:02x} = {:02x}", address, stored_value)
                }
                
                AddressingMode::ZeroPage_X => {
                    let mem_addr = address.wrapping_add(cpu.reg_x) as u16;
                    let stored_value = cpu.mem_read(mem_addr as u16);

                    hex_dump.push(address);
                    format!("${:02x},X @ {:02x} = {:02x}", address, mem_addr, stored_value)
                }
                
                AddressingMode::ZeroPage_Y => {
                    let mem_addr = address.wrapping_add(cpu.reg_y) as u16;
                    let stored_value = cpu.mem_read(mem_addr as u16);

                    hex_dump.push(address);
                    format!("${:02x},Y @ {:02x} = {:02x}", address, mem_addr, stored_value)
                }
                
                AddressingMode::Indirect_X => {
                    let addr_offset = address.wrapping_add(cpu.reg_x);
                    let lo = cpu.mem_read(addr_offset as u16);
                    let hi = cpu.mem_read(addr_offset.wrapping_add(1) as u16);
                    let mem_addr = (hi as u16) << 8 | (lo as u16);
                    let stored_value = cpu.mem_read(mem_addr);

                    hex_dump.push(address);
                    format!("(${:02x},X) @ {:02x} = {:04x} = {:02x}", 
                    address, addr_offset, mem_addr, stored_value) 
                }

                AddressingMode::Indirect_Y => {
                    let lo = cpu.mem_read(address as u16);
                    let hi = cpu.mem_read((address as u8).wrapping_add(1) as u16);
                    let addr_offset = (hi as u16) << 8 | (lo as u16);
                    let mem_addr = addr_offset.wrapping_add(cpu.reg_y as u16);
                    let stored_value = cpu.mem_read(mem_addr);

                    hex_dump.push(address);
                    format!(
                        "(${:02x}),Y = {:04x} @ {:04x} = {:02x}",
                        address, addr_offset, mem_addr, stored_value) 
                }

                AddressingMode::NoneAddressing => {
                    hex_dump.push(address);

                    let jmp_address: usize =
                        (begin as usize + 2).wrapping_add((address as i8) as usize);
                    format!("${:04x}", jmp_address)
                }

                _ => panic!(
                    "unexpected addressing mode {:?} has ops-len 2. code {:02x}",
                    ops.mode, code
                ),
            }
        }
        3 => {
            let address_lo = cpu.mem_read(begin + 1);
            let address_hi = cpu.mem_read(begin + 2);
            hex_dump.push(address_lo);
            hex_dump.push(address_hi);
            let address = cpu.mem_read_u16(begin + 1);

            match ops.mode {
                AddressingMode::NoneAddressing => {
                    if code == 0x6c {
                        //jmp indirect
                        let jmp_addr = if address & 0x00FF == 0x00FF {
                            let lo = cpu.mem_read(address);
                            let hi = cpu.mem_read(address & 0xFF00);
                            (hi as u16) << 8 | (lo as u16)
                        } else {
                            cpu.mem_read_u16(address)
                        };

                        format!("(${:04x}) = {:04x}", address, jmp_addr)
                    } else {
                        format!("${:04x}", address)
                    }
                }

                AddressingMode::Absolute => {
                    let stored_value: u8 = cpu.mem_read(address);

                    format!("${:04x} = {:02x}", address, stored_value)
                }

                AddressingMode::Absolute_X => {
                    let mem_addr = address.wrapping_add(cpu.reg_x as u16);
                    let stored_value = cpu.mem_read(mem_addr);
                    
                    format!("${:04x},X @ {:04x} = {:02x}", address, mem_addr, stored_value)
                }

                AddressingMode::Absolute_Y => {
                    let mem_addr = address.wrapping_add(cpu.reg_y as u16);
                    let stored_value = cpu.mem_read(mem_addr);

                    format!("${:04x},Y @ {:04x} = {:02x}", address, mem_addr, stored_value)
                }

                _ => panic!(
                    "unexpected addressing mode {:?} has ops-len 3. code {:02x}",
                    ops.mode, code
                ),
            }
        }
        _ => String::from(""),
    };

    let hex_str = hex_dump
        .iter()
        .map(|z| format!("{:02x}", z))
        .collect::<Vec<String>>()
        .join(" ");
    
    let asm_str = format!("{:04x}  {:8} {: >4} {}", begin, hex_str, ops.mnemonic, tmp)
        .trim()
        .to_string();

    format!(
        "{:47} A:{:02x} X:{:02x} Y:{:02x} P:{:02x} SP:{:02x}",
        asm_str, cpu.reg_acc, cpu.reg_x, cpu.reg_y, cpu.reg_status, cpu.reg_stack_ptr,
    )
    .to_ascii_uppercase()
}