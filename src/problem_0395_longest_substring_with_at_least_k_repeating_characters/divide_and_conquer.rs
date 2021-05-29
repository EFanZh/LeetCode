pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn helper(s: &[u8], k: i32) -> usize {
        let mut counts = [0; 26];

        for c in s {
            counts[usize::from(c - b'a')] += 1;
        }

        let mut iter = s.split(|c| counts[usize::from(c - b'a')] < k);
        let first = iter.next().unwrap();

        if first.len() == s.len() {
            first.len()
        } else {
            let mut result = Self::helper(first, k);

            for item in iter {
                result = result.max(Self::helper(item, k));
            }

            result
        }
    }

    pub fn longest_substring(s: String, k: i32) -> i32 {
        Self::helper(s.as_bytes(), k) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_substring(s: String, k: i32) -> i32 {
        Self::longest_substring(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
