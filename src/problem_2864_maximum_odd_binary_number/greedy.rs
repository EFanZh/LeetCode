pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let mut s = s.into_bytes();
        let ones = s.iter().filter(|&&c| c == b'1').count();
        let (left, right) = s.split_at_mut(ones - 1);
        let (last, rest) = right.split_last_mut().unwrap();

        left.fill(b'1');
        rest.fill(b'0');
        *last = b'1';

        String::from_utf8(s).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_odd_binary_number(s: String) -> String {
        Self::maximum_odd_binary_number(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
