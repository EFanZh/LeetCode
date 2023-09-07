pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_side_jumps(obstacles: Vec<i32>) -> i32 {
        const INVALID: u32 = u32::MAX / 2;

        let mut cache = (INVALID, 0, INVALID);

        for state in obstacles {
            cache = match state {
                0 => (
                    cache.0.min(cache.1.min(cache.2) + 1),
                    cache.1.min(cache.0.min(cache.2) + 1),
                    cache.2.min(cache.0.min(cache.1) + 1),
                ),
                1 => (INVALID, cache.1.min(cache.2 + 1), cache.2.min(cache.1 + 1)),
                2 => (cache.0.min(cache.2 + 1), INVALID, cache.2.min(cache.0 + 1)),
                _ => (cache.0.min(cache.1 + 1), cache.1.min(cache.0 + 1), INVALID),
            };
        }

        cache.0.min(cache.1).min(cache.2) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_side_jumps(obstacles: Vec<i32>) -> i32 {
        Self::min_side_jumps(obstacles)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
