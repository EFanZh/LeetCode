pub struct Solution;

enum State {
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
}

impl Solution {
    fn next_state(stack: &mut Vec<State>, frame: usize) -> bool {
        let state = &mut stack[frame];

        match state {
            State::S0 => {
                *state = State::S1;

                true
            }
            State::S1 => {
                *state = State::S2;

                stack.push(State::S0);

                false
            }
            State::S2 => {
                if Self::next_state(stack, frame + 1) {
                    stack[frame] = State::S3;

                    false
                } else {
                    stack[frame] = State::S4;

                    true
                }
            }
            State::S3 => {
                *state = State::S4;

                true
            }
            State::S4 => {
                if Self::next_state(stack, frame + 1) {
                    stack[frame] = State::S5;

                    true
                } else {
                    stack[frame] = State::S2;

                    false
                }
            }
            State::S5 => {
                *state = State::S2;

                false
            }
        }
    }

    pub fn magical_string(n: i32) -> i32 {
        if n < 4 {
            1
        } else {
            let mut stack = vec![State::S0];

            (2..n).map(|_| i32::from(!Self::next_state(&mut stack, 0))).sum::<i32>() + 1
        }
    }
}

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
