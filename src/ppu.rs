use crate::cartridge::Mirroring;

pub struct PPU {
    //pub ctrl: ControlRegister,
    //pub mask: MaskRegister,
    //pub status: StatusRegister,
    pub oam_addr: u8,
    pub oam_data: [u8; 256],
    //pub scroll: ScrollRegister,
    pub addr: AddrRegister,
    pub palette_table: [u8; 32],
    pub mirroring: Mirroring,
    pub vram: [u8; 2048],
    pub chr_rom: Vec<u8>,
    internal_data_buf: u8,
}

impl PPU {
    pub fn new_empty_rom()  -> Self {
        PPU::new(vec![0; 2048], Mirroring::HORIZONTAL)
    }

    pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
        PPU {
            //ctrl: ControlRegister::new(),
            //mask: MaskRegister::new(),
            //status: StatusRegister::new(),
            oam_addr: 0,
            oam_data: [0; 256],
            //scroll: ScrollRegister::new(),
            addr: AddrRegister::new(),
            palette_table: [0; 32],
            mirroring: mirroring,
            vram: [0; 2048],
            chr_rom: chr_rom,
            internal_data_buf: 0,
        }
    }

    pub fn write_ctrl(&mut self, value: u8) {

    }

    pub fn write_mask(&mut self, value: u8) {

    }

    pub fn read_status(&mut self) -> u8 {
        0
    }

    pub fn write_oam_addr(&mut self, value: u8) {

    }

    pub fn write_oam_data(&mut self, value: u8) {

    }

    pub fn read_oam_data(&self) -> u8 {
        0
    }

    pub fn write_scroll(&mut self, value: u8) {

    }

    pub fn write_ppu_addr(&mut self, value: u8) {
        self.addr.update(value);
    } 

    pub fn write_data(&mut self, value: u8) {
        let addr = self.addr.get();
        match addr {
            0..=0x1fff => println!("attempt to write to chr rom space {}", addr), 
            0x2000..=0x2fff => {
                self.vram[self.mirror_vram_addr(addr) as usize] = value;
            }
            0x3000..=0x3eff => unimplemented!("addr {} shouldn't be used in reality", addr),

            //Addresses $3F10/$3F14/$3F18/$3F1C are mirrors of $3F00/$3F04/$3F08/$3F0C
            0x3f10 | 0x3f14 | 0x3f18 | 0x3f1c => {
                let add_mirror = addr - 0x10;
                self.palette_table[(add_mirror - 0x3f00) as usize] = value;
            }
            0x3f00..=0x3fff =>
            {
                self.palette_table[(addr - 0x3f00) as usize] = value;
            }
            _ => panic!("unexpected access to mirrored space {}", addr),
        }
        self.increment_vram_addr();
    }

    pub fn read_data(&mut self) -> u8 {
        0
    }

    pub fn write_oam_dma(&mut self, value: &[u8; 256]) {

    }

    pub fn mirror_vram_addr(&self, addr: u16) -> u16 {
        let mirrored_vram = addr & 0b10111111111111; // mirror down 0x3000-0x3eff to 0x2000 - 0x2eff
        let vram_index = mirrored_vram - 0x2000; // to vram vector
        let name_table = vram_index / 0x400;
        match (&self.mirroring, name_table) {
            (Mirroring::VERTICAL, 2) | (Mirroring::VERTICAL, 3) => vram_index - 0x800,
            (Mirroring::HORIZONTAL, 2) => vram_index - 0x400,
            (Mirroring::HORIZONTAL, 1) => vram_index - 0x400,
            (Mirroring::HORIZONTAL, 3) => vram_index - 0x800,
            _ => vram_index,
        }
    }

    pub fn increment_vram_addr(&mut self) {

    }
}

pub struct AddrRegister {
    value: u16,
    hi_ptr: bool,
}

impl AddrRegister {
    pub fn new() -> Self {
        AddrRegister {
            value: 0,
            hi_ptr: true,
        }
    }

    fn set(&mut self, data: u16) {
        self.value = data;
    }

    pub fn get(&self) -> u16 {
        self.value
    }

    pub fn update(&mut self, data: u8) {
        if self.hi_ptr {
            let hi = (data as u16) << 8;
            let lo = self.value & 0xff;
            self.value = hi | lo;
        } else {
            let hi = self.value & 0xff00;
            let lo = data as u16;
            self.value = hi | lo;
        }

        self.set(self.get() & 0b11111111111111);
        self.hi_ptr = !self.hi_ptr;
    }

    pub fn increment(&mut self, inc: u8) {
        self.value = self.value.wrapping_add(inc as u16);
        self.set(self.get() & 0b11111111111111);
    }

    pub fn reset_latch(&mut self) {
        self.hi_ptr = true;
    }
}

bitflags! {
    pub struct ControlRegister: u8 {
        const NAMETABLE1              = 0b00000001;
        const NAMETABLE2              = 0b00000010;
        const VRAM_ADD_INCREMENT      = 0b00000100;
        const SPRITE_PATTERN_ADDR     = 0b00001000;
        const BACKROUND_PATTERN_ADDR  = 0b00010000;
        const SPRITE_SIZE             = 0b00100000;
        const MASTER_SLAVE_SELECT     = 0b01000000;
        const GENERATE_NMI            = 0b10000000;
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_ppu_vram_writes() {
        let mut ppu = PPU::new_empty_rom();
        ppu.write_ppu_addr(0x23);
        ppu.write_ppu_addr(0x05);
        ppu.write_data(0x66);

        assert_eq!(ppu.vram[0x0305], 0x66);
    }
}
