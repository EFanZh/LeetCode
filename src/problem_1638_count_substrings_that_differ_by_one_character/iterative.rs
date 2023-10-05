pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

// See <https://leetcode.com/problems/count-substrings-that-differ-by-one-character/discuss/917985/JavaC%2B%2BPython-Time-O(nm)-Space-O(1)>.

impl Solution {
    fn helper(s: &[u8], t: &[u8], result: &mut i32) {
        let mut length_1 = 0;
        let mut length_2 = 0;

        for (left, right) in s.iter().zip(t) {
            length_2 += 1;

            if left != right {
                length_1 = length_2;
                length_2 = 0;
            }

            *result += length_1;
        }
    }

    pub fn count_substrings(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut result = 0;

        for i in 0..t.len() {
            Self::helper(s, &t[i..], &mut result);
        }

        for i in 1..s.len() {
            Self::helper(&s[i..], t, &mut result);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_substrings(s: String, t: String) -> i32 {
        Self::count_substrings(s, t)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
