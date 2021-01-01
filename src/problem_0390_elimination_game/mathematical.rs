pub struct Solution;

impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        (((1 << (31 - n.leading_zeros())) - 1) & (n | 0x5555_5555)) + 1
    }
}

impl super::Solution for Solution {
    fn last_remaining(n: i32) -> i32 {
        Self::last_remaining(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
