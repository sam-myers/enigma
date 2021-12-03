pub struct PlugBoard(Vec<(char, char)>);

impl PlugBoard {
    pub fn new(connections: Vec<(char, char)>) -> Self {
        PlugBoard(connections)
    }

    pub fn swap(&self, c: &char) -> char {
        let mut result = c;
        self.0.iter().for_each(|(c1, c2)| match (c1, c2) {
            (input, out) if c == input => result = out,
            (out, input) if c == input => result = out,
            (_, _) => (),
        });
        *result
    }
}

#[cfg(test)]
mod test {
    use crate::plugboard::PlugBoard;

    #[test]
    fn test_switch_forward() {
        let board = PlugBoard(vec![('A', 'B')]);
        let input = 'A';
        assert_eq!(board.swap(&input), 'B');
    }

    #[test]
    fn test_switch_backward() {
        let board = PlugBoard(vec![('Y', 'X')]);
        let input = 'X';
        assert_eq!(board.swap(&input), 'Y');
    }

    #[test]
    fn test_switch_no_swap() {
        let board = PlugBoard(vec![('A', 'B')]);
        let input = 'X';
        assert_eq!(board.swap(&input), 'X');
    }
}
