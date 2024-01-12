pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        let answer_key = answer_key.into_bytes();
        let k = k as u32;
        let mut window_start = 0;
        let mut window_false_count = 0;
        let mut window_true_count = 0;

        for &c in &answer_key {
            if c == b'F' {
                window_false_count += 1;
            } else {
                window_true_count += 1;
            }

            if window_false_count.min(window_true_count) > k {
                if answer_key[window_start] == b'F' {
                    window_false_count -= 1;
                } else {
                    window_true_count -= 1;
                }

                window_start += 1;
            }
        }

        (answer_key.len() - window_start) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        Self::max_consecutive_answers(answer_key, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
