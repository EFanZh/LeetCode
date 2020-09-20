pub struct Solution;

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        (num - 1) % 9 + 1
    }
}

impl super::Solution for Solution {
    fn add_digits(num: i32) -> i32 {
        Self::add_digits(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
