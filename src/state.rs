use super::*;
use core::fmt;
use std::ops::Range;

/// A complete board state: input balls, the program, output sequence, ...
#[derive(Clone, Default, Debug)]
pub struct State {
    /// number of blue, red balls at the top of the board.
    pub balls: [u8; 2],

    /// position of the start button (blue or red lever).
    pub start_button: Color,
    /// instruction entrypoint address for blue, red balls.
    pub entry: [Addr; 2],
    pub mem: Vec<bool>,
    pub instr: Vec<Instr>,

    pub intercept: [Option<Color>; 3],

    /// sequence of balls output at the bottom of the board.
    pub out_seq: Vec<Color>,
}

impl State {
    pub fn new(bits: usize) -> Self {
        Self {
            instr: vec![ijmp(0, FALL, FALL); bits],
            mem: vec![false; bits],
            ..Self::default()
        }
    }
    pub fn run(self) -> Self {
        self.run_verbosity(0)
    }

    pub fn run_verbosity(mut self, verbosity: u8) -> Self {
        self.run_mut(verbosity);
        self
    }

    fn run_mut(&mut self, verbosity: u8) {
        let v1 = verbosity >= 1;
        let v2 = verbosity >= 2;

        if v1 {
            self.print_state();
        }

        // current program counter and falling ball color
        let (mut pc, mut cbr) = match self.try_release(self.start_button) {
            None => return, // no balls to start with, immediately halt
            Some(pc_cbr) => pc_cbr,
        };

        loop {
            // tumble down:
            // execute invert-and-branch instructions until we jump to a special address
            while pc < BLUE_LEVER {
                let Instr { mem, jmp0, jmp1 } = self.instr[pc as usize];
                let dst = mem as usize;

                if v2 {
                    print!(
                        "PC {}:  bit {} {} {}:  mem[{}]: {} -> {}",
                        pc, dst, jmp0, jmp1, dst, self.mem[dst], !self.mem[dst]
                    );
                }
                // invert...
                self.mem[dst] = !self.mem[dst];

                // ...and branch
                pc = if self.mem[dst] { jmp1 } else { jmp0 };
                if v2 {
                    println!("  jmp {}", pc);
                }
            }

            // bottom out:
            // special address reached: blue or red lever, or an interceptor.
            match pc {
                addr @ (BLUE_LEVER | RED_LEVER) => {
                    // current falling ball goes to output sequence
                    self.out_seq.push(cbr);
                    if v1 {
                        self.print_state();
                    }
                    // left or right lever determines next ball to be released
                    let next_color = match addr {
                        BLUE_LEVER => Color::Blue,
                        RED_LEVER => Color::Red,
                        _ => unreachable!(),
                    };
                    // try release next ball, if not exhausted.
                    // sets current ball register and program counter.
                    match self.try_release(next_color) {
                        None => return, // out of balls, halt
                        Some((new_pc, new_cbr)) => {
                            pc = new_pc;
                            cbr = new_cbr;
                        }
                    };
                }
                addr @ INTERC0..=INTERC2 => {
                    // jumped to interceptor: store color and halt
                    self.intercept[(addr - INTERC0) as usize] = Some(cbr);
                    if v1 {
                        self.print_state();
                    }
                    return;
                }
                invalid => panic!(
                    "The ball fell off the board! (jumped to invalid PC: {})",
                    invalid
                ),
            }
        }
    }

    fn try_release(&mut self, color: Color) -> Option<(Addr, Color)> {
        match self.balls[color as usize] {
            0 => None, // out of balls, halting.
            n => {
                self.balls[color as usize] = n - 1;
                Some((self.entry[color as usize], color))
            }
        }
    }

    fn print_state(&self) {
        print!(
            "{:2},{:2}  =>  [{}]  =>  [{}]",
            self.balls[0],
            self.balls[1],
            self.mem_str(),
            self.output_str()
        );

        for (i, ball) in self.intercept.iter().enumerate() {
            if let Some(ball) = ball {
                print!(" <I{}: {}>", i, ball);
            }
        }
        println!()
    }

    // ____________________________________ convenience output functions

    /// The output sequence (balls collected at the bottom of the board),
    /// as a string of 'b' for blue and 'r' for red. E.g. `bbrbr`.
    pub fn output_str(&self) -> String {
        self.out_seq.iter().map(Color::as_char).collect()
    }

    pub fn mem_str(&self) -> String {
        self.mem
            .iter()
            .map(|&v| if v { '1' } else { '0' })
            .collect()
    }

    /// Interpret a memory range as a binary number.
    /// mem_range runs from the number's LSB to MSB (exclusive).
    ///
    /// Note that this is the reverse of how we typically write down numbers,
    /// but it is the convention used by the Turing Tumble challenges
    /// ("registers" have their LSB at the top of the board).
    ///
    /// E.g.:
    ///     memory:[1,0,0], range 0..3 => number = 1
    ///     memory:[0,0,1], range 0..3 => number = 8
    ///
    pub fn register(&self, mem_range: Range<usize>) -> u64 {
        //dbg!(&mem_range);
        //dbg!(&self.mem[mem_range.clone()]);
        self.mem[mem_range]
            .iter()
            .enumerate()
            .fold(0, |acc, (i, &bit)| acc | ((bit as u64) << i))
    }

    /// Read the bit in memory at `address`
    pub fn bit(&self, address: Addr) -> bool {
        self.mem[address as usize]
    }

    // __________________________________ convenience constructors

    pub fn with_mem(&self, mem: Vec<bool>) -> Self {
        self.with(|s| s.mem = mem)
    }
    pub fn with_bit(&self, addr: u8, value: bool) -> Self {
        self.with(|s| s.mem[addr as usize] = value)
    }

    pub fn with_balls(&self, blue_red: [u8; 2]) -> Self {
        self.with(|s| s.balls = blue_red)
    }

    pub fn with_start(&self, start_button: Color) -> Self {
        self.with(|s| s.start_button = start_button)
    }

    pub fn with_entry(&self, entry: [Addr; 2]) -> Self {
        self.with(|s| s.entry = entry)
    }

    pub fn with_register(&self, mem_range: Range<usize>, number: u64) -> Self {
        // TODO: check that number is not too big for register
        self.with(|s| {
            for (i, j) in mem_range.into_iter().enumerate() {
                s.mem[j] = (number & (1 << i)) != 0;
            }
        })
    }

    fn with<F: FnOnce(&mut Self)>(&self, f: F) -> Self {
        let mut tmp = self.clone();
        f(&mut tmp);
        tmp
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "start: {}", self.start_button)?;

        write!(f, "start_blue: ")?;
        fmt_addr(self.entry[0], f)?;
        writeln!(f)?;

        write!(f, "start_red: ")?;
        fmt_addr(self.entry[1], f)?;
        writeln!(f)?;

        writeln!(f, "mem:")?;
        for (i, &mem) in self.mem.iter().enumerate() {
            writeln!(f, "\t{}: {}", i, if mem { '1' } else { '0' })?;
        }
        writeln!(f, "instr:")?;
        for (i, instr) in self.instr.iter().enumerate() {
            writeln!(f, "\t{}: {}", i, instr)?;
        }
        Ok(())
    }
}
