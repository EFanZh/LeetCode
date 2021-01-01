pub struct Solution;

impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        if n == 1 {
            1
        } else {
            (n / 2 - Self::last_remaining(n / 2) + 1) * 2
        }
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
