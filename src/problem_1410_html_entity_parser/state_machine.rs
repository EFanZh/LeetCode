pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

enum State {
    NotAnd,
    And,
    AndA,
    AndAM,
    AndAMP,
    AndAP,
    AndAPO,
    AndAPOS,
    AndF,
    AndFR,
    AndFRA,
    AndFRAS,
    AndFRASL,
    AndG,
    AndGT,
    AndL,
    AndLT,
    AndQ,
    AndQU,
    AndQUO,
    AndQUOT,
}

impl Solution {
    #[allow(clippy::too_many_lines)]
    pub fn entity_parser(text: String) -> String {
        let mut result = Vec::new();
        let mut state = State::NotAnd;
        let mut iter = text.bytes();

        loop {
            state = match state {
                State::NotAnd => match iter.next() {
                    None => break,
                    Some(b'&') => State::And,
                    Some(c) => {
                        result.push(c);

                        State::NotAnd
                    }
                },
                State::And => match iter.next() {
                    None => {
                        result.push(b'&');

                        break;
                    }
                    Some(b'&') => {
                        result.push(b'&');

                        State::And
                    }
                    Some(b'a') => State::AndA,
                    Some(b'f') => State::AndF,
                    Some(b'g') => State::AndG,
                    Some(b'l') => State::AndL,
                    Some(b'q') => State::AndQ,
                    Some(c) => {
                        result.push(b'&');
                        result.push(c);

                        State::NotAnd
                    }
                },
                State::AndA => match iter.next() {
                    None => {
                        result.extend(b"&a");

                        break;
                    }
                    Some(b'&') => {
                        result.extend(b"&a");

                        State::And
                    }
                    Some(b'm') => State::AndAM,
                    Some(b'p') => State::AndAP,
                    Some(c) => {
                        result.extend(b"&a");
                        result.push(c);

                        State::NotAnd
                    }
                },
                State::AndAM => match iter.next() {
                    None => {
                        result.extend(b"&am");

                        break;
                    }
                    Some(b'&') => {
                        result.extend(b"&am");

                        State::And
                    }
                    Some(b'p') => State::AndAMP,
                    Some(c) => {
                        result.extend(b"&am");
                        result.push(c);

                        State::NotAnd
                    }
                },
                State::AndAMP => match iter.next() {
                    None => {
                        result.extend(b"&amp");

                        break;
                    }
                    Some(b'&') => {
                        result.extend(b"&amp");

                        State::And
                    }
                    Some(b';') => {
                        result.push(b'&');

                        State::NotAnd
                    }
                    Some(c) => {
                        result.extend(b"&amp");
                        result.push(c);

                        State::NotAnd
                    }
                },
                State::AndAP => match iter.next() {
                    None => {
                        result.extend(b"&ap");

                        break;
                    }
                    Some(b'&') => {
                        result.extend(b"&ap");

                        State::And
                    }
                    Some(b'o') => State::AndAPO,
                    Some(c) => {
                        result.extend(b"&ap");
                        result.push(c);

                        State::NotAnd
                    }
                },
                State::AndAPO => match iter.next() {
                    None => {
                        result.extend(b"&apo");

                        break;
                    }
                    Some(b'&') => {
                        result.extend(b"&apo");

                        State::And
                    }
                    Some(b's') => State::AndAPOS,
                    Some(c) => {
                        result.extend(b"&apo");
                        result.push(c);

                        State::NotAnd
                    }
                },
                State::AndAPOS => match iter.next() {
                    None => {
                        result.extend(b"&apos");

                        break;
                    }
                    Some(b'&') => {
                        result.extend(b"&apos");

                        State::And
                    }
                    Some(b';') => {
                        result.push(b'\'');

                        State::NotAnd
                    }
                    Some(c) => {
                        result.extend(b"&apos");
                        result.push(c);

                        State::NotAnd
                    }
                },
                State::AndF => match iter.next() {
                    None => {
                        result.extend(b"&f");

                        break;
                    }
                    Some(b'&') => {
                        result.extend(b"&f");

                        State::And
                    }
                    Some(b'r') => State::AndFR,
                    Some(c) => {
                        result.extend(b"&f");
                        result.push(c);

                        State::NotAnd
                    }
                },
                State::AndFR => match iter.next() {
                    None => {
                        result.extend(b"&fr");

                        break;
                    }
                    Some(b'&') => {
                        result.extend(b"&fr");

                        State::And
                    }
                    Some(b'a') => State::AndFRA,
                    Some(c) => {
                        result.extend(b"&fr");
                        result.push(c);

                        State::NotAnd
                    }
                },
                State::AndFRA => match iter.next() {
                    None => {
                        result.extend(b"&fra");

                        break;
                    }
                    Some(b'&') => {
                        result.extend(b"&fra");

                        State::And
                    }
                    Some(b's') => State::AndFRAS,
                    Some(c) => {
                        result.extend(b"&fra");
                        result.push(c);

                        State::NotAnd
                    }
                },
                State::AndFRAS => match iter.next() {
                    None => {
                        result.extend(b"&fras");

                        break;
                    }
                    Some(b'&') => {
                        result.extend(b"&fras");

                        State::And
                    }
                    Some(b'l') => State::AndFRASL,
                    Some(c) => {
                        result.extend(b"&fras");
                        result.push(c);

                        State::NotAnd
                    }
                },
                State::AndFRASL => match iter.next() {
                    None => {
                        result.extend(b"&frasl");

                        break;
                    }
                    Some(b'&') => {
                        result.extend(b"&frasl");

                        State::And
                    }
                    Some(b';') => {
                        result.push(b'/');

                        State::NotAnd
                    }
                    Some(c) => {
                        result.extend(b"&frasl");
                        result.push(c);

                        State::NotAnd
                    }
                },
                State::AndG => match iter.next() {
                    None => {
                        result.extend(b"&g");

                        break;
                    }
                    Some(b'&') => {
                        result.extend(b"&g");

                        State::And
                    }
                    Some(b't') => State::AndGT,
                    Some(c) => {
                        result.extend(b"&g");
                        result.push(c);

                        State::NotAnd
                    }
                },
                State::AndGT => match iter.next() {
                    None => {
                        result.extend(b"&gt");

                        break;
                    }
                    Some(b'&') => {
                        result.extend(b"&gt");

                        State::And
                    }
                    Some(b';') => {
                        result.push(b'>');

                        State::NotAnd
                    }
                    Some(c) => {
                        result.extend(b"&gt");
                        result.push(c);

                        State::NotAnd
                    }
                },
                State::AndL => match iter.next() {
                    None => {
                        result.extend(b"&l");

                        break;
                    }
                    Some(b'&') => {
                        result.extend(b"&l");

                        State::And
                    }
                    Some(b't') => State::AndLT,
                    Some(c) => {
                        result.extend(b"&l");
                        result.push(c);

                        State::NotAnd
                    }
                },
                State::AndLT => match iter.next() {
                    None => {
                        result.extend(b"&lt");

                        break;
                    }
                    Some(b'&') => {
                        result.extend(b"&lt");

                        State::And
                    }
                    Some(b';') => {
                        result.push(b'<');

                        State::NotAnd
                    }
                    Some(c) => {
                        result.extend(b"&lt");
                        result.push(c);

                        State::NotAnd
                    }
                },
                State::AndQ => match iter.next() {
                    None => {
                        result.extend(b"&q");

                        break;
                    }
                    Some(b'&') => {
                        result.extend(b"&q");

                        State::And
                    }
                    Some(b'u') => State::AndQU,
                    Some(c) => {
                        result.extend(b"&q");
                        result.push(c);

                        State::NotAnd
                    }
                },
                State::AndQU => match iter.next() {
                    None => {
                        result.extend(b"&qu");

                        break;
                    }
                    Some(b'&') => {
                        result.extend(b"&qu");

                        State::And
                    }
                    Some(b'o') => State::AndQUO,
                    Some(c) => {
                        result.extend(b"&qu");
                        result.push(c);

                        State::NotAnd
                    }
                },
                State::AndQUO => match iter.next() {
                    None => {
                        result.extend(b"&quo");

                        break;
                    }
                    Some(b'&') => {
                        result.extend(b"&quo");

                        State::And
                    }
                    Some(b't') => State::AndQUOT,
                    Some(c) => {
                        result.extend(b"&quo");
                        result.push(c);

                        State::NotAnd
                    }
                },
                State::AndQUOT => match iter.next() {
                    None => {
                        result.extend(b"&quot");

                        break;
                    }
                    Some(b'&') => {
                        result.extend(b"&quot");

                        State::And
                    }
                    Some(b';') => {
                        result.push(b'"');

                        State::NotAnd
                    }
                    Some(c) => {
                        result.extend(b"&quot");
                        result.push(c);

                        State::NotAnd
                    }
                },
            };
        }

        String::from_utf8(result).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn entity_parser(text: String) -> String {
        Self::entity_parser(text)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
