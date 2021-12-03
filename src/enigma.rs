use crate::error::Result;
use crate::plugboard::PlugBoard;
use crate::reflector::Reflector;
use crate::rotor::{Direction, Rotor};
use log::debug;

pub struct Enigma {
    rotor_left: Rotor,
    rotor_middle: Rotor,
    rotor_right: Rotor,

    plugboard: PlugBoard,
    reflector: Reflector,
}

impl Enigma {
    pub fn new(
        rotor_left: Rotor,
        rotor_middle: Rotor,
        rotor_right: Rotor,
        reflector: Reflector,
        plugboard: PlugBoard,
    ) -> Self {
        Enigma {
            rotor_right,
            rotor_left,
            rotor_middle,
            reflector,
            plugboard,
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

        result = self.plugboard.swap(&result);
        trace!("Plugboard Encryption: {}", result);

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

        result = self.plugboard.swap(&result);
        trace!("Plugboard Encryption: {}", result);

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
    use crate::plugboard::PlugBoard;
    use crate::reflector::{KnownReflector, Reflector};
    use crate::rotor::{Position, RingSetting, Rotor};
    use std::fmt::Write;

    #[test]
    fn test_simple_rotors() {
        let _ = pretty_env_logger::try_init();
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
            PlugBoard::new(vec![]),
        );
        let cleartext = "ABCDEFGHIJKLMNOPQRSTUVWXYZAAAAAAAAAAAAAAAAAAAAAAAAAABBBBBBBBBBBBBBBBBBBBBBBBBBABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let expected = "BJELRQZVJWARXSNBXORSTNCFMEYHCXTGYJFLINHNXSHIUNTHEORXOPLOVFEKAGADSPNPCMHRVZCYECDAZIHVYGPITMSRZKGGHLSRBLHL";
        assert_eq!(
            enigma.encrypt_string(cleartext.to_string()),
            expected.to_string()
        );
    }

    #[test]
    fn test_varied_rotors() {
        let _ = pretty_env_logger::try_init();
        let mut enigma = Enigma::new(
            Rotor::new(
                PhysicalRotor::new(KnownRotor::VII),
                RingSetting(1),
                Position(10),
            ),
            Rotor::new(
                PhysicalRotor::new(KnownRotor::V),
                RingSetting(2),
                Position(5),
            ),
            Rotor::new(
                PhysicalRotor::new(KnownRotor::IV),
                RingSetting(3),
                Position(12),
            ),
            Reflector::new(KnownReflector::B),
            PlugBoard::new(vec![]),
        );
        let cleartext = "ABCDEFGHIJKLMNOPQRSTUVWXYZAAAAAAAAAAAAAAAAAAAAAAAAAABBBBBBBBBBBBBBBBBBBBBBBBBBABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let expected = "FOTYBPKLBZQSGZBOPUFYPFUSETWKNQQHVNHLKJZZZKHUBEJLGVUNIOYSDTEZJQHHAOYYZSENTGXNJCHEDFHQUCGCGJBURNSEDZSEPLQP";
        assert_eq!(
            enigma.encrypt_string(cleartext.to_string()),
            expected.to_string()
        );
    }

    #[test]
    fn test_long_phrase() {
        let _ = pretty_env_logger::try_init();
        let mut enigma = Enigma::new(
            Rotor::new(
                PhysicalRotor::new(KnownRotor::III),
                RingSetting(11),
                Position(3),
            ),
            Rotor::new(
                PhysicalRotor::new(KnownRotor::VI),
                RingSetting(13),
                Position(5),
            ),
            Rotor::new(
                PhysicalRotor::new(KnownRotor::VIII),
                RingSetting(19),
                Position(9),
            ),
            Reflector::new(KnownReflector::B),
            PlugBoard::new(vec![]),
        );
        let mut cleartext = String::new();
        for _ in 0..500 {
            let _ = write!(&mut cleartext, "A").unwrap();
        }
        let expected = concat!(
            r#"YJKJMFQKPCUOCKTEZQVXYZJWJFROVJMWJVXRCQYFCUVBRELVHRWGPYGCHVLBVJEVTTYVMWKJFOZHLJEXYXRDBEVEHVXKQSBPYZN"#,
            r#"IQDCBGTDDWZQWLHIBQNTYPIEBMNINNGMUPPGLSZCBRJULOLNJSOEDLOBXXGEVTKCOTTLDZPHBUFKLWSFSRKOMXKZELBDJNRUDUCO"#,
            r#"TNCGLIKVKMHHCYDEKFNOECFBWRIEFQQUFXKKGNTSTVHVITVHDFKIJIHOGMDSQUFMZCGGFZMJUKGDNDSNSJKWKENIRQKSUUHJYMIG"#,
            r#"WWNMIESFRCVIBFSOUCLBYEEHMESHSGFDESQZJLTORNFBIFUWIFJTOPVMFQCFCFPYZOJFQRFQZTTTOECTDOOYTGVKEWPSZGHCTQRP"#,
            r#"GZQOVTTOIEGGHEFDOVSUQLLGNOOWGLCLOWSISUGSVIHWCMSIUUSBWQIGWEWRKQFQQRZHMQJNKQTJFDIJYHDFCWTHXUOOCVRCVYOHL"#,
        );
        assert_eq!(enigma.encrypt_string(cleartext), expected.to_string());
    }

    #[test]
    fn test_plugboard_4_plugs() {
        let _ = pretty_env_logger::try_init();
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
            PlugBoard::new(vec![('A', 'C'), ('F', 'G'), ('J', 'Y'), ('L', 'W')]),
        );
        let cleartext = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
        let expected = "QREBNMCYZELKQOJCGJVIVGLYEMUPCURPVPUMDIWXPPWROOQEGI";
        assert_eq!(
            enigma.encrypt_string(cleartext.to_string()),
            expected.to_string()
        );
    }

    #[test]
    fn test_plugboard_6_plugs() {
        let _ = pretty_env_logger::try_init();
        let mut enigma = Enigma::new(
            Rotor::new(
                PhysicalRotor::new(KnownRotor::IV),
                RingSetting(0),
                Position(0),
            ),
            Rotor::new(
                PhysicalRotor::new(KnownRotor::VI),
                RingSetting(0),
                Position(10),
            ),
            Rotor::new(
                PhysicalRotor::new(KnownRotor::III),
                RingSetting(0),
                Position(6),
            ),
            Reflector::new(KnownReflector::B),
            PlugBoard::new(vec![
                ('B', 'M'),
                ('D', 'H'),
                ('R', 'S'),
                ('K', 'N'),
                ('G', 'Z'),
                ('F', 'Q'),
            ]),
        );
        let cleartext = "WRBHFRROSFHBCHVBENQFAGNYCGCRSTQYAJNROJAKVKXAHGUZHZVKWUTDGMBMSCYQSKABUGRVMIUOWAPKCMHYCRTSDEYTNJLVWNQY";
        let expected = "FYTIDQIBHDONUPAUVPNKILDHDJGCWFVMJUFNJSFYZTSPITBURMCJEEAMZAZIJMZAVFCTYTKYORHYDDSXHBLQWPJBMSSWIPSWLENZ";
        assert_eq!(
            enigma.encrypt_string(cleartext.to_string()),
            expected.to_string()
        );
    }

    #[test]
    fn test_plugboard_10_plugs() {
        let _ = pretty_env_logger::try_init();
        let mut enigma = Enigma::new(
            Rotor::new(
                PhysicalRotor::new(KnownRotor::I),
                RingSetting(5),
                Position(0),
            ),
            Rotor::new(
                PhysicalRotor::new(KnownRotor::II),
                RingSetting(5),
                Position(1),
            ),
            Rotor::new(
                PhysicalRotor::new(KnownRotor::III),
                RingSetting(4),
                Position(20),
            ),
            Reflector::new(KnownReflector::B),
            PlugBoard::new(vec![
                ('A', 'G'),
                ('H', 'R'),
                ('Y', 'T'),
                ('K', 'I'),
                ('F', 'L'),
                ('W', 'E'),
                ('N', 'M'),
                ('S', 'D'),
                ('O', 'P'),
                ('Q', 'J'),
            ]),
        );
        let cleartext = "RNXYAZUYTFNQFMBOLNYNYBUYPMWJUQSBYRHPOIRKQSIKBKEKEAJUNNVGUQDODVFQZHASHMQIHSQXICTSJNAUVZYIHVBBARPJADRH";
        let expected = "CFBJTPYXROYGGVTGBUTEBURBXNUZGGRALBNXIQHVBFWPLZQSCEZWTAWCKKPRSWOGNYXLCOTQAWDRRKBCADTKZGPWSTNYIJGLVIUQ";
        assert_eq!(
            enigma.encrypt_string(cleartext.to_string()),
            expected.to_string()
        );
    }
}
