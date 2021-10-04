use std::fmt;

/// Instruction or memory address.
/// (Instructions and memory have separate memory spaces.
/// for clarity, it is common for instructions to flip the memory at
//  memory address equal to their own instruction address).
pub type Addr = u8;

/// Instruction address of the blue exit.
/// Jumping here transfers control back to the board,
/// which may release a new blue ball.
pub const BLUE_LEVER: Addr = 128;

/// Instruction address of the red exit.
/// Jumping here transfers control back to the board,
/// which may release a new red ball.
pub const RED_LEVER: Addr = 129;

/// Instruct address of interceptor 0.
/// Jumping here halts the machine.
pub const INTERC0: Addr = 130;

/// Instruct address of interceptor 1.
/// Jumping here halts the machine.
pub const INTERC1: Addr = INTERC0 + 1;

/// Instruct address of interceptor 2.
/// Jumping here halts the machine.
pub const INTERC2: Addr = INTERC0 + 2;

/// An invalid address used to assert unreachable conditions.
/// Jumping here will cause the Machine to halt with
/// "A ball fell off the board".
pub const FALL: Addr = 133;

pub fn fmt_addr(a: Addr, f: &mut fmt::Formatter) -> fmt::Result {
    match a {
        BLUE_LEVER => f.write_str("B"),
        RED_LEVER => f.write_str("R"),
        i @ INTERC0..=INTERC2 => write!(f, "INTERC{}", i - INTERC0),
        FALL => f.write_str("FALL"),
        addr => write!(f, "{}", addr),
    }
}
