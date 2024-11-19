pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn longest_subsequence(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let k = k as u32;
        let bits = 32 - k.leading_zeros() as usize;

        if n < bits {
            return n as _;
        }

        let (left, right) = s.split_at(n - bits);
        let right = right.iter().fold(0, |right, &c| (right << 1) + u32::from(c & 1));
        let length = bits as u16 - u16::from(right > k);

        i32::from(left.iter().fold(length, |length, &c| length + u16::from(b'1' - c)))
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_subsequence(s: String, k: i32) -> i32 {
        Self::longest_subsequence(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
