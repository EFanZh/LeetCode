pub struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        let mut cache = (1, 0);

        for _ in 0..n {
            cache = (cache.1, cache.0 + cache.1);
        }

        cache.1
    }
}

impl super::Solution for Solution {
    fn fib(n: i32) -> i32 {
        Self::fib(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
