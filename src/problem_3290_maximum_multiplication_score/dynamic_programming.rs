pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_score(a: Vec<i32>, b: Vec<i32>) -> i64 {
        let a = <(_, _, _, _)>::from(<[_; 4]>::map(a.try_into().ok().unwrap(), i64::from));
        let mut cache = (i64::MIN / 2, i64::MIN / 2, i64::MIN / 2, i64::MIN / 2);

        for x in b {
            let x = i64::from(x);

            cache = (
                cache.0.max(a.0 * x),
                cache.1.max(cache.0 + a.1 * x),
                cache.2.max(cache.1 + a.2 * x),
                cache.3.max(cache.2 + a.3 * x),
            );
        }

        cache.3
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_score(a: Vec<i32>, b: Vec<i32>) -> i64 {
        Self::max_score(a, b)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
