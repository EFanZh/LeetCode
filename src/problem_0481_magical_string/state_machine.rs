pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

enum State {
    S0,
    S1,
    S2(Box<State>),
    S3(Box<State>),
    S4(Box<State>),
    S5(Box<State>),
}

impl State {
    fn next(&mut self) -> bool {
        match mem::replace(self, Self::S0) {
            State::S0 => {
                *self = Self::S1;

                true
            }
            State::S1 => {
                *self = Self::S2(Box::new(Self::S0));

                false
            }
            State::S2(mut inner) => {
                if inner.next() {
                    *self = Self::S3(inner);

                    false
                } else {
                    *self = Self::S4(inner);

                    true
                }
            }
            State::S3(inner) => {
                *self = Self::S4(inner);

                true
            }
            State::S4(mut inner) => {
                if inner.next() {
                    *self = Self::S5(inner);

                    true
                } else {
                    *self = Self::S2(inner);

                    false
                }
            }
            State::S5(inner) => {
                *self = Self::S2(inner);

                false
            }
        }
    }
}

impl Solution {
    pub fn magical_string(n: i32) -> i32 {
        if n < 4 {
            1
        } else {
            let mut state = State::S0;

            (2..n).map(|_| i32::from(!state.next())).sum::<i32>() + 1
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn magical_string(n: i32) -> i32 {
        Self::magical_string(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
