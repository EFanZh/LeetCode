pub mod greedy;
pub mod group_by_even_odd;

pub trait Solution {
    fn max_depth_after_split(seq: String) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("(()())", 1), ("()(())()", 1)];

        for (seq, expected) in test_cases {
            let result = S::max_depth_after_split(seq.to_string());

            assert_eq!(result.len(), seq.len());

            let mut stack_size_1 = 0_u32;
            let mut max_stack_size_1 = 0_u32;
            let mut stack_size_2 = 0_u32;
            let mut max_stack_size_2 = 0_u32;

            for (c, group) in seq.bytes().zip(result) {
                let (stack_size, max_stack_size) = if group == 0 {
                    (&mut stack_size_1, &mut max_stack_size_1)
                } else {
                    assert_eq!(group, 1);

                    (&mut stack_size_2, &mut max_stack_size_2)
                };

                if c == b'(' {
                    *stack_size += 1;
                    *max_stack_size = (*max_stack_size).max(*stack_size);
                } else {
                    *stack_size = stack_size.checked_sub(1).unwrap();
                }
            }

            assert_eq!(stack_size_1, 0);
            assert_eq!(stack_size_2, 0);
            assert_eq!(max_stack_size_1.max(max_stack_size_2), expected);
        }
    }
}
