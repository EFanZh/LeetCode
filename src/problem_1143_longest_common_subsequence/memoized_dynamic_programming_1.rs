pub struct Solution;

impl Solution {
    fn helper<T: Eq>(text_1: &[T], text_2: &[T], cache: &mut [Option<i32>], columns: usize) -> i32 {
        let cache_index = columns * text_1.len() + text_2.len();

        if let Some(result) = cache[cache_index] {
            result
        } else {
            let result = if let Some((tail_1, head_1)) = text_1.split_last() {
                if let Some((tail_2, head_2)) = text_2.split_last() {
                    if tail_1 == tail_2 {
                        Self::helper(head_1, head_2, cache, columns) + 1
                    } else {
                        Self::helper(head_1, text_2, cache, columns).max(Self::helper(text_1, head_2, cache, columns))
                    }
                } else {
                    0
                }
            } else {
                0
            };

            cache[cache_index] = Some(result);

            result
        }
    }

    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let columns = text2.len() + 1;
        let mut cache = vec![None; columns * (text1.len() + 1)];

        Self::helper(text1.as_bytes(), text2.as_bytes(), &mut cache, columns)
    }
}

impl super::Solution for Solution {
    fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        Self::longest_common_subsequence(text1, text2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
