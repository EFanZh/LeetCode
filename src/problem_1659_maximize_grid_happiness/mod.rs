pub mod dynamic_programming;

pub trait Solution {
    fn get_max_grid_happiness(m: i32, n: i32, introverts_count: i32, extroverts_count: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((2, 3, 1, 2), 240), ((3, 1, 2, 1), 260), ((2, 2, 4, 0), 240)];

        for ((m, n, introverts_count, extroverts_count), expected) in test_cases {
            assert_eq!(
                S::get_max_grid_happiness(m, n, introverts_count, extroverts_count),
                expected,
            );
        }
    }
}
