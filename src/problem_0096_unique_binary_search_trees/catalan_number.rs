pub struct Solution {}

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        // https://en.wikipedia.org/wiki/Catalan_number.

        let mut result = 1_i64;

        for i in 0..i64::from(n) {
            result *= 2;
            result *= i * 2 + 1;
            result /= i + 2;
        }

        result as _
    }
}

impl super::Solution for Solution {
    fn num_trees(n: i32) -> i32 {
        Self::num_trees(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
