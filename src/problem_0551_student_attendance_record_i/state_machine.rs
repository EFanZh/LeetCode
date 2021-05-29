pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

enum State {
    A0L0,
    A0L1,
    A0L2,
    A1L0,
    A1L1,
    A1L2,
}

impl Solution {
    pub fn check_record(s: String) -> bool {
        let mut state = State::A0L0;

        for c in s.bytes() {
            state = match (state, c) {
                (State::A0L0, b'A')
                | (State::A0L1, b'A')
                | (State::A0L2, b'A')
                | (State::A1L0, b'P')
                | (State::A1L1, b'P')
                | (State::A1L2, b'P') => State::A1L0,
                (State::A0L0, b'L') => State::A0L1,
                (State::A0L0, b'P') | (State::A0L1, b'P') | (State::A0L2, b'P') => State::A0L0,
                (State::A0L1, b'L') => State::A0L2,
                (State::A1L0, b'L') => State::A1L1,
                (State::A1L1, b'L') => State::A1L2,
                _ => return false,
            };
        }

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn check_record(s: String) -> bool {
        Self::check_record(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
