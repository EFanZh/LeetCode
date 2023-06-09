pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn interpret(command: String) -> String {
        let mut result = command.into_bytes();
        let buffer = result.as_mut_slice();
        let mut writer = 0;
        let mut reader = 0;

        while let Some(&c) = buffer.get(reader) {
            reader += 1;

            let decoded = if c == b'G' {
                b'G'
            } else if buffer[reader] == b')' {
                reader += 1;

                b'o'
            } else {
                reader += 3;

                buffer[writer] = b'a';
                writer += 1;

                b'l'
            };

            buffer[writer] = decoded;
            writer += 1;
        }

        result.truncate(writer);

        String::from_utf8(result).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn interpret(command: String) -> String {
        Self::interpret(command)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
