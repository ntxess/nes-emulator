#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    NoneAddressing,
}

pub struct OpCodes {
    pub mnemonic: &'static str,
    pub len: u8,
    pub mode: AddressingMode,
    pub cycles: u8,
}

impl OpCodes {
    fn new(mnemonic: &'static str, len: u8, mode: AddressingMode, cycles: u8) -> Self {
        OpCodes {
            mnemonic: mnemonic,
            len: len,
            mode: mode,
            cycles: cycles,
        }
    }
}

lazy_static! {
    pub static ref DEBUG_OPCODES: Vec<OpCodes> = vec![
        OpCodes{mnemonic: "brk", len: 1, mode: AddressingMode::NoneAddressing, cycles: 7},
        OpCodes{mnemonic: "ora", len: 2, mode: AddressingMode::Indirect_X, cycles: 6},
        OpCodes{mnemonic: "*nop", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "*slo", len: 2, mode: AddressingMode::Indirect_X, cycles: 8},
        OpCodes{mnemonic: "*nop", len: 2, mode: AddressingMode::ZeroPage, cycles: 3},
        OpCodes{mnemonic: "ora", len: 2, mode: AddressingMode::ZeroPage, cycles: 3},
        OpCodes{mnemonic: "asl", len: 2, mode: AddressingMode::ZeroPage, cycles: 5},
        OpCodes{mnemonic: "*slo", len: 2, mode: AddressingMode::ZeroPage, cycles: 5},
        OpCodes{mnemonic: "php", len: 1, mode: AddressingMode::NoneAddressing, cycles: 3},
        OpCodes{mnemonic: "ora", len: 2, mode: AddressingMode::Immediate, cycles: 2},
        OpCodes{mnemonic: "asl", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "*anc", len: 2, mode: AddressingMode::Immediate, cycles: 2},
        OpCodes{mnemonic: "*nop", len: 3, mode: AddressingMode::Absolute, cycles: 4},
        OpCodes{mnemonic: "ora", len: 3, mode: AddressingMode::Absolute, cycles: 4},
        OpCodes{mnemonic: "asl", len: 3, mode: AddressingMode::Absolute, cycles: 6},
        OpCodes{mnemonic: "*slo", len: 3, mode: AddressingMode::Absolute, cycles: 6},
        OpCodes{mnemonic: "bpl", len: 2, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "ora", len: 2, mode: AddressingMode::Indirect_Y, cycles: 5},
        OpCodes{mnemonic: "*nop", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "*slo", len: 2, mode: AddressingMode::Indirect_Y, cycles: 8},
        OpCodes{mnemonic: "*nop", len: 2, mode: AddressingMode::ZeroPage_X, cycles: 4},
        OpCodes{mnemonic: "ora", len: 2, mode: AddressingMode::ZeroPage_X, cycles: 4},
        OpCodes{mnemonic: "asl", len: 2, mode: AddressingMode::ZeroPage_X, cycles: 6},
        OpCodes{mnemonic: "*slo", len: 2, mode: AddressingMode::ZeroPage_X, cycles: 6},
        OpCodes{mnemonic: "clc", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "ora", len: 3, mode: AddressingMode::Absolute_Y, cycles: 4},
        OpCodes{mnemonic: "*nop", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "*slo", len: 3, mode: AddressingMode::Absolute_Y, cycles: 7},
        OpCodes{mnemonic: "*nop", len: 3, mode: AddressingMode::Absolute_X, cycles: 4},
        OpCodes{mnemonic: "ora", len: 3, mode: AddressingMode::Absolute_X, cycles: 4},
        OpCodes{mnemonic: "asl", len: 3, mode: AddressingMode::Absolute_X, cycles: 7},
        OpCodes{mnemonic: "*slo", len: 3, mode: AddressingMode::Absolute_X, cycles: 7},
        OpCodes{mnemonic: "jsr", len: 3, mode: AddressingMode::NoneAddressing, cycles: 6},
        OpCodes{mnemonic: "and", len: 2, mode: AddressingMode::Indirect_X, cycles: 6},
        OpCodes{mnemonic: "*nop", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "*rla", len: 2, mode: AddressingMode::Indirect_X, cycles: 8},
        OpCodes{mnemonic: "bit", len: 2, mode: AddressingMode::ZeroPage, cycles: 3},
        OpCodes{mnemonic: "and", len: 2, mode: AddressingMode::ZeroPage, cycles: 3},
        OpCodes{mnemonic: "rol", len: 2, mode: AddressingMode::ZeroPage, cycles: 5},
        OpCodes{mnemonic: "*rla", len: 2, mode: AddressingMode::ZeroPage, cycles: 5},
        OpCodes{mnemonic: "plp", len: 1, mode: AddressingMode::NoneAddressing, cycles: 4},
        OpCodes{mnemonic: "and", len: 2, mode: AddressingMode::Immediate, cycles: 2},
        OpCodes{mnemonic: "rol", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "*anc", len: 2, mode: AddressingMode::Immediate, cycles: 2},
        OpCodes{mnemonic: "bit", len: 3, mode: AddressingMode::Absolute, cycles: 4},
        OpCodes{mnemonic: "and", len: 3, mode: AddressingMode::Absolute, cycles: 4},
        OpCodes{mnemonic: "rol", len: 3, mode: AddressingMode::Absolute, cycles: 6},
        OpCodes{mnemonic: "*rla", len: 3, mode: AddressingMode::Absolute, cycles: 6},
        OpCodes{mnemonic: "bmi", len: 2, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "and", len: 2, mode: AddressingMode::Indirect_Y, cycles: 5},
        OpCodes{mnemonic: "*nop", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "*rla", len: 2, mode: AddressingMode::Indirect_Y, cycles: 8},
        OpCodes{mnemonic: "*nop", len: 2, mode: AddressingMode::ZeroPage_X, cycles: 4},
        OpCodes{mnemonic: "and", len: 2, mode: AddressingMode::ZeroPage_X, cycles: 4},
        OpCodes{mnemonic: "rol", len: 2, mode: AddressingMode::ZeroPage_X, cycles: 6},
        OpCodes{mnemonic: "*rla", len: 2, mode: AddressingMode::ZeroPage_X, cycles: 6},
        OpCodes{mnemonic: "sec", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "and", len: 3, mode: AddressingMode::Absolute_Y, cycles: 4},
        OpCodes{mnemonic: "*nop", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "*rla", len: 3, mode: AddressingMode::Absolute_Y, cycles: 7},
        OpCodes{mnemonic: "*nop", len: 3, mode: AddressingMode::Absolute_X, cycles: 4},
        OpCodes{mnemonic: "and", len: 3, mode: AddressingMode::Absolute_X, cycles: 4},
        OpCodes{mnemonic: "rol", len: 3, mode: AddressingMode::Absolute_X, cycles: 7},
        OpCodes{mnemonic: "*rla", len: 3, mode: AddressingMode::Absolute_X, cycles: 7},
        OpCodes{mnemonic: "rti", len: 1, mode: AddressingMode::NoneAddressing, cycles: 6},
        OpCodes{mnemonic: "eor", len: 2, mode: AddressingMode::Indirect_X, cycles: 6},
        OpCodes{mnemonic: "*nop", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "*sre", len: 2, mode: AddressingMode::Indirect_X, cycles: 8},
        OpCodes{mnemonic: "*nop", len: 2, mode: AddressingMode::ZeroPage, cycles: 3},
        OpCodes{mnemonic: "eor", len: 2, mode: AddressingMode::ZeroPage, cycles: 3},
        OpCodes{mnemonic: "lsr", len: 2, mode: AddressingMode::ZeroPage, cycles: 5},
        OpCodes{mnemonic: "*sre", len: 2, mode: AddressingMode::ZeroPage, cycles: 5},
        OpCodes{mnemonic: "pha", len: 1, mode: AddressingMode::NoneAddressing, cycles: 3},
        OpCodes{mnemonic: "eor", len: 2, mode: AddressingMode::Immediate, cycles: 2},
        OpCodes{mnemonic: "lsr", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "*alr", len: 2, mode: AddressingMode::Immediate, cycles: 2},
        OpCodes{mnemonic: "jmp", len: 3, mode: AddressingMode::NoneAddressing, cycles: 3},
        OpCodes{mnemonic: "eor", len: 3, mode: AddressingMode::Absolute, cycles: 4},
        OpCodes{mnemonic: "lsr", len: 3, mode: AddressingMode::Absolute, cycles: 6},
        OpCodes{mnemonic: "*sre", len: 3, mode: AddressingMode::Absolute, cycles: 6},
        OpCodes{mnemonic: "bvc", len: 2, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "eor", len: 2, mode: AddressingMode::Indirect_Y, cycles: 5},
        OpCodes{mnemonic: "*nop", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "*sre", len: 2, mode: AddressingMode::Indirect_Y, cycles: 8},
        OpCodes{mnemonic: "*nop", len: 2, mode: AddressingMode::ZeroPage_X, cycles: 4},
        OpCodes{mnemonic: "eor", len: 2, mode: AddressingMode::ZeroPage_X, cycles: 4},
        OpCodes{mnemonic: "lsr", len: 2, mode: AddressingMode::ZeroPage_X, cycles: 6},
        OpCodes{mnemonic: "*sre", len: 2, mode: AddressingMode::ZeroPage_X, cycles: 6},
        OpCodes{mnemonic: "cli", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "eor", len: 3, mode: AddressingMode::Absolute_Y, cycles: 4},
        OpCodes{mnemonic: "*nop", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "*sre", len: 3, mode: AddressingMode::Absolute_Y, cycles: 7},
        OpCodes{mnemonic: "*nop", len: 3, mode: AddressingMode::Absolute_X, cycles: 4},
        OpCodes{mnemonic: "eor", len: 3, mode: AddressingMode::Absolute_X, cycles: 4},
        OpCodes{mnemonic: "lsr", len: 3, mode: AddressingMode::Absolute_X, cycles: 7},
        OpCodes{mnemonic: "*sre", len: 3, mode: AddressingMode::Absolute_X, cycles: 7},
        OpCodes{mnemonic: "rts", len: 1, mode: AddressingMode::NoneAddressing, cycles: 6},
        OpCodes{mnemonic: "adc", len: 2, mode: AddressingMode::Indirect_X, cycles: 6},
        OpCodes{mnemonic: "*nop", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "*rra", len: 2, mode: AddressingMode::Indirect_X, cycles: 8},
        OpCodes{mnemonic: "*nop", len: 2, mode: AddressingMode::ZeroPage, cycles: 3},
        OpCodes{mnemonic: "adc", len: 2, mode: AddressingMode::ZeroPage, cycles: 3},
        OpCodes{mnemonic: "ror", len: 2, mode: AddressingMode::ZeroPage, cycles: 5},
        OpCodes{mnemonic: "*rra", len: 2, mode: AddressingMode::ZeroPage, cycles: 5},
        OpCodes{mnemonic: "pla", len: 1, mode: AddressingMode::NoneAddressing, cycles: 4},
        OpCodes{mnemonic: "adc", len: 2, mode: AddressingMode::Immediate, cycles: 2},
        OpCodes{mnemonic: "ror", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "*arr", len: 2, mode: AddressingMode::Immediate, cycles: 2},
        OpCodes{mnemonic: "jmp", len: 3, mode: AddressingMode::NoneAddressing, cycles: 5},
        OpCodes{mnemonic: "adc", len: 3, mode: AddressingMode::Absolute, cycles: 4},
        OpCodes{mnemonic: "ror", len: 3, mode: AddressingMode::Absolute, cycles: 6},
        OpCodes{mnemonic: "*rra", len: 3, mode: AddressingMode::Absolute, cycles: 6},
        OpCodes{mnemonic: "bvs", len: 2, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "adc", len: 2, mode: AddressingMode::Indirect_Y, cycles: 5},
        OpCodes{mnemonic: "*nop", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "*rra", len: 2, mode: AddressingMode::Indirect_Y, cycles: 8},
        OpCodes{mnemonic: "*nop", len: 2, mode: AddressingMode::ZeroPage_X, cycles: 4},
        OpCodes{mnemonic: "adc", len: 2, mode: AddressingMode::ZeroPage_X, cycles: 4},
        OpCodes{mnemonic: "ror", len: 2, mode: AddressingMode::ZeroPage_X, cycles: 6},
        OpCodes{mnemonic: "*rra", len: 2, mode: AddressingMode::ZeroPage_X, cycles: 6},
        OpCodes{mnemonic: "sei", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "adc", len: 3, mode: AddressingMode::Absolute_Y, cycles: 4},
        OpCodes{mnemonic: "*nop", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "*rra", len: 3, mode: AddressingMode::Absolute_Y, cycles: 7},
        OpCodes{mnemonic: "*nop", len: 3, mode: AddressingMode::Absolute_X, cycles: 4},
        OpCodes{mnemonic: "adc", len: 3, mode: AddressingMode::Absolute_X, cycles: 4},
        OpCodes{mnemonic: "ror", len: 3, mode: AddressingMode::Absolute_X, cycles: 7},
        OpCodes{mnemonic: "*rra", len: 3, mode: AddressingMode::Absolute_X, cycles: 7},
        OpCodes{mnemonic: "*nop", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "sta", len: 2, mode: AddressingMode::Indirect_X, cycles: 6},
        OpCodes{mnemonic: "*nop", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "*sax", len: 2, mode: AddressingMode::Indirect_X, cycles: 6},
        OpCodes{mnemonic: "sty", len: 2, mode: AddressingMode::ZeroPage, cycles: 3},
        OpCodes{mnemonic: "sta", len: 2, mode: AddressingMode::ZeroPage, cycles: 3},
        OpCodes{mnemonic: "stx", len: 2, mode: AddressingMode::ZeroPage, cycles: 3},
        OpCodes{mnemonic: "*sax", len: 2, mode: AddressingMode::ZeroPage, cycles: 3},
        OpCodes{mnemonic: "dey", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "*nop", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "txa", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "*xaa", len: 2, mode: AddressingMode::Immediate, cycles: 3},
        OpCodes{mnemonic: "sty", len: 3, mode: AddressingMode::Absolute, cycles: 4},
        OpCodes{mnemonic: "sta", len: 3, mode: AddressingMode::Absolute, cycles: 4},
        OpCodes{mnemonic: "stx", len: 3, mode: AddressingMode::Absolute, cycles: 4},
        OpCodes{mnemonic: "*sax", len: 3, mode: AddressingMode::Absolute, cycles: 4},
        OpCodes{mnemonic: "bcc", len: 2, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "sta", len: 2, mode: AddressingMode::Indirect_Y, cycles: 6},
        OpCodes{mnemonic: "*nop", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "*ahx", len: 2, mode: AddressingMode::Indirect_Y, cycles: 8},
        OpCodes{mnemonic: "sty", len: 2, mode: AddressingMode::ZeroPage_X, cycles: 4},
        OpCodes{mnemonic: "sta", len: 2, mode: AddressingMode::ZeroPage_X, cycles: 4},
        OpCodes{mnemonic: "stx", len: 2, mode: AddressingMode::ZeroPage_Y, cycles: 4},
        OpCodes{mnemonic: "*sax", len: 2, mode: AddressingMode::ZeroPage_Y, cycles: 4},
        OpCodes{mnemonic: "tya", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "sta", len: 3, mode: AddressingMode::Absolute_Y, cycles: 5},
        OpCodes{mnemonic: "txs", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "*tas", len: 3, mode: AddressingMode::Absolute_Y, cycles: 2},
        OpCodes{mnemonic: "*shy", len: 3, mode: AddressingMode::Absolute_X, cycles: 4},
        OpCodes{mnemonic: "sta", len: 3, mode: AddressingMode::Absolute_X, cycles: 5},
        OpCodes{mnemonic: "*shx", len: 3, mode: AddressingMode::Absolute_Y, cycles: 4},
        OpCodes{mnemonic: "*ahx", len: 3, mode: AddressingMode::Absolute_Y, cycles: 4},
        OpCodes{mnemonic: "ldy", len: 2, mode: AddressingMode::Immediate, cycles: 2},
        OpCodes{mnemonic: "lda", len: 2, mode: AddressingMode::Indirect_X, cycles: 6},
        OpCodes{mnemonic: "ldx", len: 2, mode: AddressingMode::Immediate, cycles: 2},
        OpCodes{mnemonic: "*lax", len: 2, mode: AddressingMode::Indirect_X, cycles: 6},
        OpCodes{mnemonic: "ldy", len: 2, mode: AddressingMode::ZeroPage, cycles: 3},
        OpCodes{mnemonic: "lda", len: 2, mode: AddressingMode::ZeroPage, cycles: 3},
        OpCodes{mnemonic: "ldx", len: 2, mode: AddressingMode::ZeroPage, cycles: 3},
        OpCodes{mnemonic: "*lax", len: 2, mode: AddressingMode::ZeroPage, cycles: 3},
        OpCodes{mnemonic: "tay", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "lda", len: 2, mode: AddressingMode::Immediate, cycles: 2},
        OpCodes{mnemonic: "tax", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "*lxa", len: 2, mode: AddressingMode::Immediate, cycles: 3},
        OpCodes{mnemonic: "ldy", len: 3, mode: AddressingMode::Absolute, cycles: 4},
        OpCodes{mnemonic: "lda", len: 3, mode: AddressingMode::Absolute, cycles: 4},
        OpCodes{mnemonic: "ldx", len: 3, mode: AddressingMode::Absolute, cycles: 4},
        OpCodes{mnemonic: "*lax", len: 3, mode: AddressingMode::Absolute, cycles: 4},
        OpCodes{mnemonic: "bcs", len: 2, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "lda", len: 2, mode: AddressingMode::Indirect_Y, cycles: 5},
        OpCodes{mnemonic: "*nop", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "*lax", len: 2, mode: AddressingMode::Indirect_Y, cycles: 5},
        OpCodes{mnemonic: "ldy", len: 2, mode: AddressingMode::ZeroPage_X, cycles: 4},
        OpCodes{mnemonic: "lda", len: 2, mode: AddressingMode::ZeroPage_X, cycles: 4},
        OpCodes{mnemonic: "ldx", len: 2, mode: AddressingMode::ZeroPage_Y, cycles: 4},
        OpCodes{mnemonic: "*lax", len: 2, mode: AddressingMode::ZeroPage_Y, cycles: 4},
        OpCodes{mnemonic: "clv", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "lda", len: 3, mode: AddressingMode::Absolute_Y, cycles: 4},
        OpCodes{mnemonic: "tsx", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "*las", len: 3, mode: AddressingMode::Absolute_Y, cycles: 2},
        OpCodes{mnemonic: "ldy", len: 3, mode: AddressingMode::Absolute_X, cycles: 4},
        OpCodes{mnemonic: "lda", len: 3, mode: AddressingMode::Absolute_X, cycles: 4},
        OpCodes{mnemonic: "ldx", len: 3, mode: AddressingMode::Absolute_Y, cycles: 4},
        OpCodes{mnemonic: "*lax", len: 3, mode: AddressingMode::Absolute_Y, cycles: 4},
        OpCodes{mnemonic: "cpy", len: 2, mode: AddressingMode::Immediate, cycles: 2},
        OpCodes{mnemonic: "cmp", len: 2, mode: AddressingMode::Indirect_X, cycles: 6},
        OpCodes{mnemonic: "*nop", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "*dcp", len: 2, mode: AddressingMode::Indirect_X, cycles: 8},
        OpCodes{mnemonic: "cpy", len: 2, mode: AddressingMode::ZeroPage, cycles: 3},
        OpCodes{mnemonic: "cmp", len: 2, mode: AddressingMode::ZeroPage, cycles: 3},
        OpCodes{mnemonic: "dec", len: 2, mode: AddressingMode::ZeroPage, cycles: 5},
        OpCodes{mnemonic: "*dcp", len: 2, mode: AddressingMode::ZeroPage, cycles: 5},
        OpCodes{mnemonic: "iny", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "cmp", len: 2, mode: AddressingMode::Immediate, cycles: 2},
        OpCodes{mnemonic: "dex", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "*axs", len: 2, mode: AddressingMode::Immediate, cycles: 2},
        OpCodes{mnemonic: "cpy", len: 3, mode: AddressingMode::Absolute, cycles: 4},
        OpCodes{mnemonic: "cmp", len: 3, mode: AddressingMode::Absolute, cycles: 4},
        OpCodes{mnemonic: "dec", len: 3, mode: AddressingMode::Absolute, cycles: 6},
        OpCodes{mnemonic: "*dcp", len: 3, mode: AddressingMode::Absolute, cycles: 6},
        OpCodes{mnemonic: "bne", len: 2, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "cmp", len: 2, mode: AddressingMode::Indirect_Y, cycles: 5},
        OpCodes{mnemonic: "*nop", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "*dcp", len: 2, mode: AddressingMode::Indirect_Y, cycles: 8},
        OpCodes{mnemonic: "*nop", len: 2, mode: AddressingMode::ZeroPage_X, cycles: 4},
        OpCodes{mnemonic: "cmp", len: 2, mode: AddressingMode::ZeroPage_X, cycles: 4},
        OpCodes{mnemonic: "dec", len: 2, mode: AddressingMode::ZeroPage_X, cycles: 6},
        OpCodes{mnemonic: "*dcp", len: 2, mode: AddressingMode::ZeroPage_X, cycles: 6},
        OpCodes{mnemonic: "cld", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "cmp", len: 3, mode: AddressingMode::Absolute_Y, cycles: 4},
        OpCodes{mnemonic: "*nop", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "*dcp", len: 3, mode: AddressingMode::Absolute_Y, cycles: 7},
        OpCodes{mnemonic: "*nop", len: 3, mode: AddressingMode::Absolute_X, cycles: 4},
        OpCodes{mnemonic: "cmp", len: 3, mode: AddressingMode::Absolute_X, cycles: 4},
        OpCodes{mnemonic: "dec", len: 3, mode: AddressingMode::Absolute_X, cycles: 7},
        OpCodes{mnemonic: "*dcp", len: 3, mode: AddressingMode::Absolute_X, cycles: 7},
        OpCodes{mnemonic: "cpx", len: 2, mode: AddressingMode::Immediate, cycles: 2},
        OpCodes{mnemonic: "sbc", len: 2, mode: AddressingMode::Indirect_X, cycles: 6},
        OpCodes{mnemonic: "*nop", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "*isb", len: 2, mode: AddressingMode::Indirect_X, cycles: 8},
        OpCodes{mnemonic: "cpx", len: 2, mode: AddressingMode::ZeroPage, cycles: 3},
        OpCodes{mnemonic: "sbc", len: 2, mode: AddressingMode::ZeroPage, cycles: 3},
        OpCodes{mnemonic: "inc", len: 2, mode: AddressingMode::ZeroPage, cycles: 5},
        OpCodes{mnemonic: "*isb", len: 2, mode: AddressingMode::ZeroPage, cycles: 5},
        OpCodes{mnemonic: "inx", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "sbc", len: 2, mode: AddressingMode::Immediate, cycles: 2},
        OpCodes{mnemonic: "nop", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "*sbc", len: 2, mode: AddressingMode::Immediate, cycles: 2},
        OpCodes{mnemonic: "cpx", len: 3, mode: AddressingMode::Absolute, cycles: 4},
        OpCodes{mnemonic: "sbc", len: 3, mode: AddressingMode::Absolute, cycles: 4},
        OpCodes{mnemonic: "inc", len: 3, mode: AddressingMode::Absolute, cycles: 6},
        OpCodes{mnemonic: "*isb", len: 3, mode: AddressingMode::Absolute, cycles: 6},
        OpCodes{mnemonic: "beq", len: 2, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "sbc", len: 2, mode: AddressingMode::Indirect_Y, cycles: 5},
        OpCodes{mnemonic: "*nop", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "*isb", len: 2, mode: AddressingMode::Indirect_Y, cycles: 8},
        OpCodes{mnemonic: "*nop", len: 2, mode: AddressingMode::ZeroPage_X, cycles: 4},
        OpCodes{mnemonic: "sbc", len: 2, mode: AddressingMode::ZeroPage_X, cycles: 4},
        OpCodes{mnemonic: "inc", len: 2, mode: AddressingMode::ZeroPage_X, cycles: 6},
        OpCodes{mnemonic: "*isb", len: 2, mode: AddressingMode::ZeroPage_X, cycles: 6},
        OpCodes{mnemonic: "sed", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "sbc", len: 3, mode: AddressingMode::Absolute_Y, cycles: 4},
        OpCodes{mnemonic: "*nop", len: 1, mode: AddressingMode::NoneAddressing, cycles: 2},
        OpCodes{mnemonic: "*isb", len: 3, mode: AddressingMode::Absolute_Y, cycles: 7},
        OpCodes{mnemonic: "*nop", len: 3, mode: AddressingMode::Absolute_X, cycles: 4},
        OpCodes{mnemonic: "sbc", len: 3, mode: AddressingMode::Absolute_X, cycles: 4},
        OpCodes{mnemonic: "inc", len: 3, mode: AddressingMode::Absolute_X, cycles: 7},
        OpCodes{mnemonic: "*isb", len: 3, mode: AddressingMode::Absolute_X, cycles: 7},
    ];
}