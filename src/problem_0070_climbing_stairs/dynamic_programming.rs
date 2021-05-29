pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut cache = (0, 1);

        for _ in 0..n {
            cache = (cache.1, cache.0 + cache.1);
        }

        cache.1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn climb_stairs(n: i32) -> i32 {
        Self::climb_stairs(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
