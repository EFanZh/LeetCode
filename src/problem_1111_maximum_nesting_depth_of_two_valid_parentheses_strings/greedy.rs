pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_depth_after_split(seq: String) -> Vec<i32> {
        let mut stack_1 = 0;
        let mut stack_2 = 0;

        seq.bytes()
            .map(|c| {
                if c == b'(' {
                    if stack_2 < stack_1 {
                        stack_2 += 1;

                        1
                    } else {
                        stack_1 += 1;

                        0
                    }
                } else if stack_2 > stack_1 {
                    stack_2 -= 1;

                    1
                } else {
                    stack_1 -= 1;

                    0
                }
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_depth_after_split(seq: String) -> Vec<i32> {
        Self::max_depth_after_split(seq)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
