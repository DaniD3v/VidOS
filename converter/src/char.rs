use crate::constants::{BACKGROUND, CHARACTERS, FOREGROUND};

pub struct Char<'a> {
    bitmap: &'a [u16; 9],
    foreground_color: &'a [u8; 3],
    background_color: &'a [u8; 3],
}

impl Char<'_> {
    fn new(char: usize, foreground: usize, background: usize) -> Self {
        Char {
            bitmap: &CHARACTERS[char],
            foreground_color: &FOREGROUND[foreground],
            background_color: &BACKGROUND[background],
        }
    }

    fn get_color(&self, x: u8, y: u8) -> [u8; 3] {
        match self.bitmap[y as usize] & (1 << x) {
            1 => *self.foreground_color,
            _ => *self.background_color,
        }
    }
}

struct CharID {
    id: u16,
}

impl CharID {
    fn new() -> Self {
        CharID { id: 0 }
    }
}

impl Iterator for CharID {
    type Item = Char<'static>;

    fn next(&mut self) -> Option<Self::Item> {
        let char = (self.id % 256) as usize;
        let foreground = ((self.id / 256) % 16) as usize;
        let background = ((self.id / 4096) % 8) as usize;

        self.id += 1;

        Some(Char::new(char, foreground, background))
    }
}
