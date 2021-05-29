pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn compute_prefix_function(mut s: impl FnMut(usize) -> Option<u8>, length: usize) -> usize {
        let mut result = vec![0; length];
        let mut j = 0;

        for i in 1..length {
            let c = s(i);

            loop {
                if s(j) == c {
                    j += 1;

                    break;
                } else if let Some(next_j) = result.get(j.wrapping_sub(1)) {
                    j = *next_j;
                } else {
                    break;
                }
            }

            result[i] = j;
        }

        *result.last().unwrap()
    }

    pub fn shortest_palindrome(s: String) -> String {
        let s = s.as_bytes();
        let length = s.len();
        let length_2 = length + length;

        let split = Self::compute_prefix_function(
            |i| s.get(if i < length { i } else { length_2 - i }).copied(),
            length_2 + 1,
        );

        let mut result = s[split..].iter().copied().rev().collect::<Vec<_>>();

        result.extend(s);

        String::from_utf8(result).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn shortest_palindrome(s: String) -> String {
        Self::shortest_palindrome(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
