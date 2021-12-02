pub struct Reflector {
    pub name: &'static str,

    pub(crate) wiring: Vec<u8>,
}

#[allow(dead_code)]
pub enum KnownReflector {
    B,
    C,
}

impl Reflector {
    pub fn new(known: KnownReflector) -> Self {
        match known {
            KnownReflector::B => Reflector::build("B", "YRUHQSLDPXNGOKMIEBFZCWVJAT"),
            KnownReflector::C => Reflector::build("C", "FVPJIAOYEDRZXWGCTKUQSBNMHL"),
        }
    }

    fn build(name: &'static str, encoding: &'static str) -> Self {
        Self {
            name,
            wiring: encoding.chars().into_iter().map(|c| c as u8 - 65).collect(),
        }
    }

    pub fn reflect(&self, c: char) -> char {
        let input: usize = (c as u8 - 65) as usize;
        (self.wiring[input] + 65) as char
    }
}
