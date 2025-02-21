pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

enum State {
    Start,
    Empty,
    Hamster,
    EmptyHamster,
    Food,
}

impl Solution {
    pub fn minimum_buckets(hamsters: String) -> i32 {
        let mut result = 0;
        let mut state = State::Start;

        for c in hamsters.into_bytes() {
            let is_empty = c == b'.';

            state = match state {
                State::Start => {
                    if is_empty {
                        State::Empty
                    } else {
                        State::Hamster
                    }
                }
                State::Empty => {
                    if is_empty {
                        State::Empty
                    } else {
                        State::EmptyHamster
                    }
                }
                State::Hamster => {
                    if is_empty {
                        result += 1;

                        State::Food
                    } else {
                        return -1;
                    }
                }
                State::EmptyHamster => {
                    result += 1;

                    if is_empty { State::Food } else { State::Hamster }
                }
                State::Food => {
                    if is_empty {
                        State::Empty
                    } else {
                        State::Start
                    }
                }
            };
        }

        result += match state {
            State::Start | State::Empty | State::Food => 0,
            State::Hamster => return -1,
            State::EmptyHamster => 1,
        };

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_buckets(hamsters: String) -> i32 {
        Self::minimum_buckets(hamsters)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
