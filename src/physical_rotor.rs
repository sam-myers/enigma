#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalRotor {
    pub name: &'static str,
    pub(crate) notches: Notches,
    pub(crate) wiring: Vec<u8>,
    pub(crate) wiring_reversed: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Notches {
    Single(u8),
    Double(u8, u8),
}

#[allow(dead_code)]
pub enum KnownRotor {
    I,
    II,
    III,
    IV,
    V,
    VI,
    VII,
    VIII,
    IDENTITY,
}

impl PhysicalRotor {
    pub fn new(known: KnownRotor) -> Self {
        match known {
            KnownRotor::I => {
                PhysicalRotor::build("I", "EKMFLGDQVZNTOWYHXUSPAIBRCJ", Notches::Single(16))
            }
            KnownRotor::II => {
                PhysicalRotor::build("II", "AJDKSIRUXBLHWTMCQGZNPYFVOE", Notches::Single(4))
            }
            KnownRotor::III => {
                PhysicalRotor::build("III", "BDFHJLCPRTXVZNYEIWGAKMUSQO", Notches::Single(21))
            }
            KnownRotor::IV => {
                PhysicalRotor::build("IV", "ESOVPZJAYQUIRHXLNFTGKDCMWB", Notches::Single(9))
            }
            KnownRotor::V => {
                PhysicalRotor::build("V", "VZBRGITYUPSDNHLXAWMJQOFECK", Notches::Single(25))
            }
            KnownRotor::VI => {
                PhysicalRotor::build("VI", "JPGVOUMFYQBENHZRDKASXLICTW", Notches::Double(12, 25))
            }
            KnownRotor::VII => {
                PhysicalRotor::build("VII", "NZJHGRCXMYSWBOUFAIVLPEKQDT", Notches::Double(12, 25))
            }
            KnownRotor::VIII => PhysicalRotor::build(
                "VIII",
                "FKQHTLXOCBJSPDZRAMEWNIUYGV",
                Notches::Double(12, 25),
            ),
            KnownRotor::IDENTITY => {
                PhysicalRotor::build("Identity", "ABCDEFGHIJKLMNOPQRSTUVWXYZ", Notches::Single(0))
            }
        }
    }

    fn build(name: &'static str, encoding: &'static str, notches: Notches) -> Self {
        let wiring: Vec<u8> = encoding.chars().into_iter().map(|c| c as u8 - 65).collect();
        let mut reversed: [u8; 26] = [0; 26];
        let _ = &wiring
            .iter()
            .enumerate()
            .for_each(|(index, wire)| reversed[*wire as usize] = index as u8);

        Self {
            name,
            notches,
            wiring,
            wiring_reversed: reversed.to_vec(),
        }
    }
}

pub(crate) mod test {
    use crate::physical_rotor::{KnownRotor, PhysicalRotor};
    use proptest::prelude::*;

    pub(crate) fn physical_rotor_strategy() -> impl Strategy<Value = PhysicalRotor> {
        prop_oneof![
            Just(PhysicalRotor::new(KnownRotor::I)),
            Just(PhysicalRotor::new(KnownRotor::II)),
            Just(PhysicalRotor::new(KnownRotor::III)),
            Just(PhysicalRotor::new(KnownRotor::IV)),
            Just(PhysicalRotor::new(KnownRotor::V)),
            Just(PhysicalRotor::new(KnownRotor::VI)),
            Just(PhysicalRotor::new(KnownRotor::VII)),
            Just(PhysicalRotor::new(KnownRotor::VIII)),
        ]
    }
}
