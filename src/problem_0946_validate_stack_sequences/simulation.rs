pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut pushed = pushed;
        let mut stack_length = 0_usize;
        let mut i = 0;

        for expected in popped {
            if pushed
                .get(stack_length.wrapping_sub(1))
                .map_or(false, |&last| last == expected)
            {
                stack_length -= 1;
            } else {
                loop {
                    if let Some(&candidate) = pushed.get(i) {
                        i += 1;

                        if candidate == expected {
                            break;
                        }

                        pushed[stack_length] = candidate;
                        stack_length += 1;
                    } else {
                        return false;
                    }
                }
            }
        }

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        Self::validate_stack_sequences(pushed, popped)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
