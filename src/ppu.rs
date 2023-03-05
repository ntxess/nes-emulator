use core::panic;
use crate::cartridge::Mirroring;

bitflags! {
    pub struct CtrlRegister: u8 {
        const NAMETABLE_1            = 0b00000001;
        const NAMETABLE_2            = 0b00000010;
        const VRAM_ADDR_INCREMENT    = 0b00000100;
        const SPRITE_PATTERN_ADDR    = 0b00001000;
        const BACKROUND_PATTERN_ADDR = 0b00010000;
        const SPRITE_SIZE            = 0b00100000;
        const MASTER_SLAVE_SELECT    = 0b01000000;
        const GENERATE_NMI           = 0b10000000;
    }

    pub struct MaskRegister: u8 {
        const GREYSCALE              = 0b00000001;
        const LEFTMOST_8PXL_BG       = 0b00000010;
        const LEFTMOST_8PXL_SP       = 0b00000100;
        const SHOW_BACKGROUND        = 0b00001000;
        const SHOW_SPRITES           = 0b00010000;
        const EMPHASISE_RED          = 0b00100000;
        const EMPHASISE_GREEN        = 0b01000000;
        const EMPHASISE_BLUE         = 0b10000000;
    }

    pub struct StatRegister: u8 {
        const NOTUSED                = 0b00000001;
        const NOTUSED2               = 0b00000010;
        const NOTUSED3               = 0b00000100;
        const NOTUSED4               = 0b00001000;
        const NOTUSED5               = 0b00010000;
        const SPRITE_OVERFLOW        = 0b00100000;
        const SPRITE_ZERO_HIT        = 0b01000000;
        const VBLANK_STARTED         = 0b10000000;
    }
}

pub enum Color {
    Red,
    Green,
    Blue,
}

pub struct PPU {
    pub reg_ctrl:       CtrlRegister,
    pub reg_mask:       MaskRegister,
    pub reg_status:     StatRegister,
    pub reg_oam_addr:   u8,
    pub reg_oam_data:   [u8; 256],

    pub reg_scroll_x:   u8,
    pub reg_scroll_y:   u8,
    latch_scroll:       bool,

    pub reg_addr:       u16,
    latch_hi_byte:      bool,

    pub palette_tbl:    [u8; 32],
    pub mirroring:      Mirroring,
    pub vram:           [u8; 2048],
    pub chr_rom:        Vec<u8>,
    internal_data_buf:  u8,
}

impl PPU {
    pub fn new_empty_rom()  -> Self {
        PPU::new(vec![0; 2048], Mirroring::HORIZONTAL)
    }

    pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
        PPU {
            reg_ctrl:          CtrlRegister::from_bits_truncate(0b00000000),
            reg_mask:          MaskRegister::from_bits_truncate(0b00000000),
            reg_status:        StatRegister::from_bits_truncate(0b00000000),
            reg_oam_addr:      0,
            reg_oam_data:      [0; 256],
            
            reg_scroll_x:      0,
            reg_scroll_y:      0,
            latch_scroll:      false,

            reg_addr:          0,
            latch_hi_byte:     true,

            palette_tbl:       [0; 32],
            mirroring:         mirroring,
            vram:              [0; 2048],
            chr_rom:           chr_rom,
            internal_data_buf: 0,
        }
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

    pub fn tick(&mut self, cycles: u8) -> bool {
        true
    }

    // pub fn poll_nmi_interrupt(&mut self) -> Option<u8> {
        
    // }
    
    fn increment_vram_addr(&mut self) {

    }

    fn is_sprite_0_hit(&self, cycle: usize) -> bool {
        true
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
        self.addr_write(value);
    } 

    pub fn write_data(&mut self, value: u8) {
        let addr = self.reg_addr;
        match addr {
            0..=0x1fff => println!("attempt to write to chr rom space {}", addr), 

            0x2000..=0x2fff => {
                self.vram[self.mirror_vram_addr(addr) as usize] = value;
            }

            0x3000..=0x3eff => unimplemented!("addr {} shouldn't be used in reality", addr),

            0x3f10 | 0x3f14 | 0x3f18 | 0x3f1c => {
                let add_mirror = addr - 0x10;
                self.palette_tbl[(add_mirror - 0x3f00) as usize] = value;
            }

            0x3f00..=0x3fff => {
                self.palette_tbl[(addr - 0x3f00) as usize] = value;
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

    /// Control Register
    /// Base nametable address
    pub fn ctrl_nametable(&self) -> u16 {
        match self.reg_ctrl.bits & 0b11 {
            0 => 0x2000,
            1 => 0x2400,
            2 => 0x2800,
            3 => 0x2c00,
            _ => 0x0000,
        }
    } 

    /// VRAM address increment per CPU read/write of PPUDATA
    pub fn ctrl_vram_addr(&self) -> u8 {
        if !self.reg_ctrl.contains(CtrlRegister::VRAM_ADDR_INCREMENT) {
            1
        } else {
            32
        }  
    }

    /// Sprite pattern table address for 8x8 sprites
    pub fn ctrl_sprt_addr(&self) -> u16 {
        if !self.reg_ctrl.contains(CtrlRegister::SPRITE_PATTERN_ADDR) {
            0
        } else {
            0x1000
        }
    }

    /// Background pattern table address
    pub fn ctrl_bg_addr(&self) -> u16 {
        if !self.reg_ctrl.contains(CtrlRegister::BACKROUND_PATTERN_ADDR) {
            0
        } else {
            0x1000
        }
    }

    /// Sprite size
    pub fn ctrl_sprt_size(&self) -> u8 {
        if !self.reg_ctrl.contains(CtrlRegister::SPRITE_SIZE) {
            8
        } else {
            16
        }
    }

    /// PPU master/slave select
    pub fn ctrl_ms_select(&self) -> u8 {
        if !self.reg_ctrl.contains(CtrlRegister::SPRITE_SIZE) {
            0
        } else {
            1
        }
    }

    /// Generate an NMI at the start of the vertical blanking interval
    pub fn generate_vblank_nmi(&self) -> bool {
        self.reg_ctrl.contains(CtrlRegister::GENERATE_NMI)
    }

    pub fn ctrl_update(&mut self, data: u8) {
        self.reg_ctrl.bits = data;
    }

    /// Mask Register
    /// Greyscale (0: normal color, 1: produce a greyscale display)
    pub fn mask_greyscale(&self) -> bool {
        self.reg_mask.contains(MaskRegister::GREYSCALE)
    }

    /// 1: Show background in leftmost 8 pixels of screen, 0: Hide
    pub fn mask_leftmost_bg(&self) -> bool {
        self.reg_mask.contains(MaskRegister::LEFTMOST_8PXL_BG)
    }

    /// 1: Show sprites in leftmost 8 pixels of screen, 0: Hide
    pub fn mask_leftmost_sprt(&self) -> bool {
        self.reg_mask.contains(MaskRegister::LEFTMOST_8PXL_SP)
    }

    /// 1: Show background
    pub fn mask_show_bg(&self) -> bool {
        self.reg_mask.contains(MaskRegister::SHOW_BACKGROUND)
    }

    /// 1: Show sprites
    pub fn mask_show_sprt(&self) -> bool {
        self.reg_mask.contains(MaskRegister::SHOW_SPRITES)
    }

    /// Emphasize red || blue || green
    pub fn mask_emphasize_color(&self) -> Vec<Color> {
        let mut result = Vec::<Color>::new();
        if self.reg_mask.contains(MaskRegister::EMPHASISE_RED) {
            result.push(Color::Red);
        }

        if self.reg_mask.contains(MaskRegister::EMPHASISE_BLUE) {
            result.push(Color::Blue);
        }

        if self.reg_mask.contains(MaskRegister::EMPHASISE_GREEN) {
            result.push(Color::Green);
        }

        result
    }

    pub fn mask_write(&mut self, data: u8) {
        self.reg_mask.bits = data;
    }

    /// Status Register
    

    /// Scroll Register
    pub fn scrll_write(&mut self, data: u8) {
        if !self.latch_scroll {
            self.reg_scroll_x = data;
        } else {
            self.reg_scroll_y = data;
        }

        self.latch_scroll = !self.latch_scroll;
    }

    pub fn scrll_reset_latch(&mut self) {
        self.latch_scroll = false;
    }

    /// Address Register
    pub fn addr_write(&mut self, data: u8) {
        if self.latch_hi_byte {
            let hi = (data as u16) << 8;
            let lo = self.reg_addr & 0xff;
            self.reg_addr = hi | lo;
        } else {
            let hi = self.reg_addr & 0xff00;
            let lo = data as u16;
            self.reg_addr = hi | lo;
        }

        self.reg_addr = self.reg_addr & 0b11111111111111;
        self.latch_hi_byte = !self.latch_hi_byte;
    }

    pub fn addr_increment(&mut self, inc: u8) {
        self.reg_addr = self.reg_addr.wrapping_add(inc as u16);
        self.reg_addr = self.reg_addr & 0b11111111111111;
    }

    pub fn addr_reset_latch(&mut self) {
        self.latch_hi_byte = true;
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
