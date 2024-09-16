pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    fn is_subsequence(s: &[u8], k: u16, pattern: &[u8]) -> bool {
        let mut iter = s.iter();

        (0..k).all(|_| pattern.iter().all(|&lhs| iter.any(|&rhs| lhs == rhs)))
    }

    pub fn longest_subsequence_repeated_k(s: String, k: i32) -> String {
        let k = k as u16;
        let mut counts = [0_u16; 26];

        for c in s.bytes() {
            counts[usize::from(c) - usize::from(b'a')] += 1;
        }

        let mut candidates = [0_u8; 26];
        let mut i = 0;

        for (c, &count) in (b'a'..).zip(&counts) {
            if count >= k {
                candidates[i] = c;
                i += 1;
            }
        }

        let candidates = &candidates[..i];
        let mut queue = VecDeque::from([[0; 8]]);
        let mut result_bytes = [0; 8];
        let mut length = 0;

        loop {
            for _ in 0..queue.len() {
                let current = queue.pop_front().unwrap();
                let mut current_counts = [0_u8; 26];

                for &c in &current[..length] {
                    current_counts[usize::from(c) - usize::from(b'a')] += 1;
                }

                for &append in candidates {
                    if counts[usize::from(append) - usize::from(b'a')]
                        - u16::from(current_counts[usize::from(append) - usize::from(b'a')])
                        >= k
                    {
                        let mut next = current;

                        next[length] = append;

                        if Self::is_subsequence(s.as_bytes(), k, &next[..=length]) {
                            queue.push_back(next);
                        }
                    }
                }
            }

            if let Some(&front) = queue.back() {
                result_bytes = front;
                length += 1;
            } else {
                break;
            }
        }

        let mut result = s.into_bytes();

        result.truncate(length);
        result.copy_from_slice(&result_bytes[..length]);

        String::from_utf8(result).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_subsequence_repeated_k(s: String, k: i32) -> String {
        Self::longest_subsequence_repeated_k(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
