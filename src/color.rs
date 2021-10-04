use std::fmt;
use std::fmt::Write;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Color {
    Blue = 0,
    Red = 1,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_char(self.as_char())
    }
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self)
    }
}

impl Color {
    pub fn as_char(&self) -> char {
        match self {
            Self::Blue => 'b',
            Self::Red => 'r',
        }
    }
}

impl Into<usize> for Color {
    fn into(self) -> usize {
        self as usize
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::Blue
    }
}
