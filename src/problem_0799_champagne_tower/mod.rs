pub mod dynamic_programming;

pub trait Solution {
    fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[allow(clippy::manual_assert)]
    pub fn run<S: Solution>() {
        let test_cases = [
            ((1, 1, 1), 0.0),
            ((2, 1, 1), 0.5),
            ((100_000_009, 33, 17), 1.0),
            ((25, 6, 1), 0.18750),
            ((1, 2, 1), 0.0),
            ((1, 3, 1), 0.0),
            ((4, 2, 1), 0.5),
            ((0, 0, 0), 0.0),
            ((1, 0, 0), 1.0),
        ];

        for ((poured, query_row, query_glass), expected) in test_cases {
            approx::assert_ulps_eq!(S::champagne_tower(poured, query_row, query_glass), expected);
        }
    }
}
