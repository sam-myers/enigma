use crate::error::{EnigmaError, Result};
use crate::physical_rotor::{Notches, PhysicalRotor};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Position(pub u8);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RingSetting(pub u8);
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Direction {
    Forward,
    Backward,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rotor {
    physical: PhysicalRotor,
    ring_setting: RingSetting,
    position: Position,
}

impl Rotor {
    pub fn new(physical: PhysicalRotor, ring_setting: RingSetting, position: Position) -> Rotor {
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
        let shift = (26 + self.position.0 - self.ring_setting.0) % 26;
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
        self.position = Position((self.position.0 + 1) % 26);
    }

    pub fn get_position(&self) -> char {
        (self.position.0 + 65) as char
    }
}

mod test {
    use crate::error::EnigmaError;
    use crate::physical_rotor::test::physical_rotor_strategy;
    use crate::rotor::{Direction, Position, RingSetting, Rotor};
    use proptest::prelude::*;

    prop_compose! {
        fn rotor_strategy()(setting in 0..26u8, position in 0..26u8, physical in physical_rotor_strategy()) -> Rotor {
            Rotor::new(physical, RingSetting(setting), Position(position))
        }
    }

    prop_compose! {
        fn character()(c_int in 0..26u8) -> char {
            (c_int + 65) as char
        }
    }

    proptest! {
        #[test]
        fn rotate_26_times(mut rotor in rotor_strategy()) {
            let saved_rotor = rotor.clone();
            prop_assert_eq!(&rotor, &saved_rotor);
            for _ in 0..25 {
                rotor.rotate();
                prop_assert_ne!(&rotor, &saved_rotor);
            }
            rotor.rotate();
            prop_assert_eq!(&rotor, &saved_rotor);
        }
    }

    proptest! {
        #[test]
        fn encypher_decypher(rotor in rotor_strategy(), plaintext in character()) {
            let enciphered = rotor.encipher(plaintext, Direction::Forward).unwrap();
            let deciphered = rotor.encipher(enciphered, Direction::Backward).unwrap();
            prop_assert_eq!(plaintext, deciphered);
        }
    }

    proptest! {
        #[test]
        fn encypher_arbitrary_no_panic(rotor in rotor_strategy(), plaintext: char) {
            let enciphered = rotor.encipher(plaintext, Direction::Forward);
            match plaintext {
                'A'..='Z' => prop_assert!(enciphered.is_ok()),
                _ => prop_assert!(matches!(enciphered, Err(EnigmaError::InvalidChar(_)))),
            }
        }
    }
}
