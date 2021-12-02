use crate::error::{EnigmaError, Result};
use crate::physical_rotor::{Notches, PhysicalRotor};

pub struct RotorPosition(pub u8);
pub struct RotorRingSetting(pub u8);
pub enum Direction {
    Forward,
    Backward,
}

pub struct Rotor {
    physical: PhysicalRotor,
    ring_setting: RotorRingSetting,
    position: RotorPosition,
}

impl Rotor {
    pub fn new(
        physical: PhysicalRotor,
        ring_setting: RotorRingSetting,
        position: RotorPosition,
    ) -> Rotor {
        Rotor {
            ring_setting,
            physical,
            position,
        }
    }

    pub fn encipher(&self, c: char, direction: Direction) -> Result<char> {
        match c {
            'A'..='Z' => (),
            _ => return Err(EnigmaError::InvalidChar(c)),
        };

        let wiring = match direction {
            Direction::Forward => &self.physical.wiring,
            Direction::Backward => &self.physical.wiring_reversed,
        };
        let char_num: u8 = c as u8 - 65;
        let shift = self.position.0 - self.ring_setting.0;
        let index: usize = ((char_num + shift + 26) % 26) as usize;
        let result_num = (wiring[index] + 26 - shift) % 26;
        Ok((result_num + 65) as char)
    }

    pub fn is_at_notch(&self) -> bool {
        match self.physical.notches {
            Notches::Single(n) => n == self.position.0,
            Notches::Double(n1, n2) => n1 == self.position.0 || n2 == self.position.0,
        }
    }

    pub fn rotate(&mut self) {
        self.position = RotorPosition((self.position.0 + 1) % 26);
    }

    pub fn get_position(&self) -> char {
        (self.position.0 + 65) as char
    }
}
