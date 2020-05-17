pub struct Solution {}

impl Solution {
    fn binomial(n: i32, k: i32) -> i32 {
        let n = i64::from(n);
        let k = i64::from(k);
        let mut result = 1;

        for (i, j) in ((k.max(n - k) + 1)..=n).rev().zip(1..) {
            result *= i;
            result /= j;
        }

        result as _
    }

    pub fn unique_paths(m: i32, n: i32) -> i32 {
        Self::binomial(m + n - 2, m - 1)
    }
}

impl super::Solution for Solution {
    fn unique_paths(m: i32, n: i32) -> i32 {
        Self::unique_paths(m, n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
