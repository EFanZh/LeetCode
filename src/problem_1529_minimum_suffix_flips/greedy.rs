pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_flips(target: String) -> i32 {
        let mut prev = b'0';
        let mut result = 0;

        for c in target.bytes() {
            result += i32::from(c != prev);
            prev = c;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_flips(target: String) -> i32 {
        Self::min_flips(target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
