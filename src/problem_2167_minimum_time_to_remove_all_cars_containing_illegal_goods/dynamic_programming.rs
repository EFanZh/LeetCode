pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_time(s: String) -> i32 {
        let n = s.len() as u32;
        let mut left = 0_u32;
        let mut result = n;

        for (i, c) in (1..).zip(s.into_bytes()) {
            left = i.min(left + (u32::from(c) - u32::from(b'0')) * 2);
            result = result.min(left + (n - i));
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_time(s: String) -> i32 {
        Self::minimum_time(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
