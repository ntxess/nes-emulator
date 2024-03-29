// From bugzmanov nes_ebook
use crate::cpu::Mem;
use crate::cartridge::Rom;
use crate::ppu::PPU;

const RAM: u16 = 0x0000;
const RAM_MIRRORS_END: u16 = 0x1FFF;
const PPU_REGISTERS: u16 = 0x2000;
const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;

pub struct Bus {
    cpu_vram: [u8; 2048],
    prg_rom: Vec<u8>,
    ppu: PPU,
    cycles: usize,
    //gameloop_callback: Box<dyn FnMut(&NesPPU, &mut Joypad) + 'call>,
}

impl Bus {
    pub fn new(rom: Rom) -> Self {
        let ppu = PPU::new(rom.chr_rom, rom.screen_mirroring);

        Bus {
            cpu_vram: [0; 2048],
            prg_rom: rom.prg_rom,
            ppu: ppu,
            cycles: 0,
            //gameloop_callback: Box::from(gameloop_callback),
        }
    }

    fn read_prg_rom(&self, mut addr: u16) -> u8 {
        addr -= 0x8000;
        if self.prg_rom.len() == 0x4000 && addr >= 0x4000 {
            addr = addr % 0x4000;
        }
        self.prg_rom[addr as usize]
    }

    // pub fn tick(&mut self, cycles: u8) {
    //     self.cycles += cycles as usize;

    //     let nmi_before = self.ppu.nmi_interrupt.is_some();
    //     self.ppu.tick(cycles *3);
    //     let nmi_after = self.ppu.nmi_interrupt.is_some();
        
    //     if !nmi_before && nmi_after {
    //         (self.gameloop_callback)(&self.ppu, &mut self.joypad1);
    //     }
    // }
    
    pub fn poll_nmi_status(&mut self) -> Option<u8> {
        self.ppu.poll_nmi_interrupt()
    }
}

impl Mem for Bus {
    fn mem_read(&mut self, addr: u16) -> u8 {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0b00000111_11111111;
                self.cpu_vram[mirror_down_addr as usize]
            }

            0x2000 | 0x2001 | 0x2003 | 0x2005 | 0x2006 | 0x4014 => 0,
            0x2002 => self.ppu.read_status(),
            0x2004 => self.ppu.read_oam_data(),
            0x2007 => self.ppu.read_data(),
            
            0x2008..=PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_addr = addr & 0b00100000_00000111;
                self.mem_read(mirror_down_addr)
            }

            0x8000..=0xFFFF => self.read_prg_rom(addr),
            _ => 0,
        }
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0b11111111111;
                self.cpu_vram[mirror_down_addr as usize] = data;
            }

            0x2000 => {
                self.ppu.write_ctrl(data);
            }

            0x2001 => {
                self.ppu.write_mask(data);
            }

            0x2002 => panic!("attempt to write to PPU status register"),

            0x2003 => {
                self.ppu.write_oam_addr(data);
            }

            0x2004 => {
                self.ppu.write_oam_data(data);
            }

            0x2005 => {
                self.ppu.write_scroll(data);
            }

            0x2006 => {
                self.ppu.write_ppu_addr(data);
            }

            0x2007 => {
                self.ppu.write_data(data);
            }

            0x2008..=PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_addr = addr & 0b00100000_00000111;
                self.mem_write(mirror_down_addr, data);
            }

            0x8000..=0xFFFF => panic!("Attempt to write to Cartridge ROM space: {:x}", addr),

            _ => {
                println!("Ignoring mem write-access at {}", addr);
            }
        }
    }
}