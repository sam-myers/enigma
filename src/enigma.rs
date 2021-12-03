use crate::error::Result;
use crate::reflector::Reflector;
use crate::rotor::{Direction, Rotor};
use log::debug;

pub struct Enigma {
    rotor_left: Rotor,
    rotor_middle: Rotor,
    rotor_right: Rotor,
    reflector: Reflector,
}

impl Enigma {
    pub fn new(
        rotor_left: Rotor,
        rotor_middle: Rotor,
        rotor_right: Rotor,
        reflector: Reflector,
    ) -> Self {
        Enigma {
            rotor_right,
            rotor_left,
            rotor_middle,
            reflector,
        }
    }

    fn rotate(&mut self) {
        if self.rotor_middle.is_at_notch() {
            self.rotor_middle.rotate();
            self.rotor_left.rotate();
        } else if self.rotor_right.is_at_notch() {
            self.rotor_middle.rotate();
        }
        self.rotor_right.rotate();
    }

    pub fn rotor_positions(&self) -> String {
        format!(
            "{}{}{}",
            self.rotor_left.get_position(),
            self.rotor_middle.get_position(),
            self.rotor_right.get_position(),
        )
    }

    pub fn encrypt_char(&mut self, c: char) -> Result<char> {
        debug!("Keyboard Input: {}", c);
        self.rotate();
        trace!("Rotor Position: {}", self.rotor_positions());

        let mut result = c;

        result = self.rotor_right.encipher(result, Direction::Forward)?;
        trace!("Wheel 3 Encryption: {}", result);
        result = self.rotor_middle.encipher(result, Direction::Forward)?;
        trace!("Wheel 2 Encryption: {}", result);
        result = self.rotor_left.encipher(result, Direction::Forward)?;
        trace!("Wheel 1 Encryption: {}", result);

        result = self.reflector.reflect(result);
        trace!("Reflector Encryption: {}", result);

        result = self.rotor_left.encipher(result, Direction::Backward)?;
        trace!("Wheel 1 Encryption: {}", result);
        result = self.rotor_middle.encipher(result, Direction::Backward)?;
        trace!("Wheel 2 Encryption: {}", result);
        result = self.rotor_right.encipher(result, Direction::Backward)?;
        trace!("Wheel 3 Encryption: {}", result);

        debug!("Output (Lampboard): {}", result);
        debug!("-----------------------------");
        Ok(result)
    }

    pub fn encrypt_string(&mut self, input: String) -> String {
        input
            .chars()
            .into_iter()
            .map(|c| self.encrypt_char(c).unwrap_or('*'))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    // Testing against https://www.101computing.net/enigma-machine-emulator/
    use crate::enigma::Enigma;
    use crate::physical_rotor::{KnownRotor, PhysicalRotor};
    use crate::reflector::{KnownReflector, Reflector};
    use crate::rotor::{Position, RingSetting, Rotor};

    #[test]
    fn test_simple() {
        pretty_env_logger::init();

        let mut enigma = Enigma::new(
            Rotor::new(
                PhysicalRotor::new(KnownRotor::I),
                RingSetting(0),
                Position(0),
            ),
            Rotor::new(
                PhysicalRotor::new(KnownRotor::II),
                RingSetting(0),
                Position(0),
            ),
            Rotor::new(
                PhysicalRotor::new(KnownRotor::III),
                RingSetting(0),
                Position(0),
            ),
            Reflector::new(KnownReflector::B),
        );

        let cleartext = "AAAAAAAAAAAAAA";
        let expected = "BDZGOWCXLTKSBT";
        assert_eq!(
            enigma.encrypt_string(cleartext.to_string()),
            expected.to_string()
        );
    }
}
