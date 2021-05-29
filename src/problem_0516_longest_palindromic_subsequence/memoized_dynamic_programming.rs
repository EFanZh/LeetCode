pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    fn longest_palindrome_subseq_helper(s: &[u8], cache: &mut HashMap<(*const u8, usize), i32>) -> i32 {
        let key = (s.as_ptr(), s.len());

        if let Some(&result) = cache.get(&key) {
            result
        } else {
            let result = if let Some((head, non_head)) = s.split_first() {
                if let Some((tail, body)) = non_head.split_last() {
                    if head == tail {
                        Self::longest_palindrome_subseq_helper(body, cache) + 2
                    } else {
                        Self::longest_palindrome_subseq_helper(non_head, cache)
                            .max(Self::longest_palindrome_subseq_helper(&s[..s.len() - 1], cache))
                    }
                } else {
                    1
                }
            } else {
                0
            };

            cache.insert(key, result);

            result
        }
    }

    pub fn longest_palindrome_subseq(s: String) -> i32 {
        Self::longest_palindrome_subseq_helper(s.as_bytes(), &mut HashMap::new())
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_palindrome_subseq(s: String) -> i32 {
        Self::longest_palindrome_subseq(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
