pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    fn helper<T: Eq>(text_1: &[T], text_2: &[T], cache: &mut HashMap<(usize, usize), i32>) -> i32 {
        let key = (text_1.len(), text_2.len());

        if let Some(result) = cache.get(&key) {
            *result
        } else {
            let result = if let Some((tail_1, head_1)) = text_1.split_last() {
                if let Some((tail_2, head_2)) = text_2.split_last() {
                    if tail_1 == tail_2 {
                        Self::helper(head_1, head_2, cache) + 1
                    } else {
                        Self::helper(head_1, text_2, cache).max(Self::helper(text_1, head_2, cache))
                    }
                } else {
                    0
                }
            } else {
                0
            };

            cache.insert(key, result);

            result
        }
    }

    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        Self::helper(text1.as_bytes(), text2.as_bytes(), &mut HashMap::new())
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
