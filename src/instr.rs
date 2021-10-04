use super::*;
use std::fmt::{self, Write};

/// The Machine's only instruction:
/// Invert-and-Branch.
#[derive(Copy, Clone)]
pub struct Instr {
    /// Memory address to invert.
    /// Equal to the instruction address for bits,
    /// possibly different from the instruction address for gear bits.
    pub mem: Addr,

    /// Instruction address to jump to if memory was flipped to 0.
    pub jmp0: Addr,

    /// Instruction address to jump to if memory was flipped to 1.
    pub jmp1: Addr,
}

/// Shorthand constructor.
pub fn ijmp(dst: Addr, jmp0: Addr, jmp1: Addr) -> Instr {
    Instr {
        mem: dst,
        jmp0,
        jmp1,
    }
}

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ijmp {} ", self.mem)?;
        fmt_addr(self.jmp0, f)?;
        f.write_char(' ')?;
        fmt_addr(self.jmp1, f)
    }
}

impl fmt::Debug for Instr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
