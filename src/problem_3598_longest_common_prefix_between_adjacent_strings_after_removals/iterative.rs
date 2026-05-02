pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn common_prefix_length(left: &str, right: &str) -> u32 {
        left.bytes()
            .zip(right.bytes())
            .take_while(|(left, right)| left == right)
            .count() as _
    }

    pub fn longest_common_prefix(words: Vec<String>) -> Vec<i32> {
        let n = words.len();

        if n < 2 {
            words.into_iter().map(|_| 0).collect()
        } else {
            let lengths = words
                .windows(2)
                .map(|pair| {
                    let [left, right]: &[String; 2] = pair.try_into().ok().unwrap();

                    Self::common_prefix_length(left, right)
                })
                .collect::<Vec<_>>();

            let mut max_length = 0;

            let suffix_lengths = lengths[1..]
                .iter()
                .rev()
                .map(|&length| {
                    max_length = max_length.max(length);

                    max_length
                })
                .collect::<Vec<_>>();

            let mut max_left_length = 0;

            (0..n)
                .map(|i| {
                    if let Some(&left) = lengths.get(i.wrapping_sub(2)) {
                        max_left_length = max_left_length.max(left);
                    }

                    let mut result = max_left_length;

                    if let (Some(left), Some(right)) = (words.get(i.wrapping_sub(1)), words.get(i + 1)) {
                        result = result.max(Self::common_prefix_length(left, right));
                    }

                    if let Some(&right) = suffix_lengths.get(suffix_lengths.len().wrapping_sub(i + 1)) {
                        result = result.max(right);
                    }

                    result.cast_signed()
                })
                .collect()
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_common_prefix(words: Vec<String>) -> Vec<i32> {
        Self::longest_common_prefix(words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
