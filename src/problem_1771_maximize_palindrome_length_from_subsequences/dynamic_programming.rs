pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn longest_palindrome(word1: String, word2: String) -> i32 {
        let left_length = word1.len();
        let n = left_length + word2.len();
        let mut s = Vec::<u8>::with_capacity(n);

        s.extend(word1.as_bytes());
        s.extend(word2.as_bytes());

        let mut buffer = vec![0_u16; n * 2].into_boxed_slice();
        let (mut cache_1, mut cache_2) = buffer.split_at_mut(n);

        cache_2.fill(1);

        let mut result = 0;

        for length in 2..=n {
            let mut iter = cache_1.iter_mut().zip(&mut *cache_2);
            let (mut target, mut top) = iter.next().unwrap();

            for (start, ((left, right), (top_top_right, top_right))) in
                s.iter().zip(&s[length - 1..]).zip(iter).enumerate()
            {
                *target = if left == right {
                    let target_length = *top_top_right + 2;

                    if start < left_length && start + length > left_length {
                        result = result.max(target_length);
                    }

                    target_length
                } else {
                    (*top).max(*top_right)
                };

                target = top_top_right;
                top = top_right;
            }

            mem::swap(&mut cache_1, &mut cache_2);
        }

        i32::from(result)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_palindrome(word1: String, word2: String) -> i32 {
        Self::longest_palindrome(word1, word2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
