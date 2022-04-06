pub mod find_cycle;

pub trait Solution {
    fn prison_after_n_days(cells: Vec<i32>, n: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (([0, 1, 0, 1, 1, 0, 0, 1], 7), [0, 0, 1, 1, 0, 0, 0, 0]),
            (([1, 0, 0, 1, 0, 0, 1, 0], 1_000_000_000), [0, 0, 1, 1, 1, 1, 1, 0]),
        ];

        for ((cells, n), expected) in test_cases {
            assert_eq!(S::prison_after_n_days(cells.to_vec(), n), expected);
        }
    }
}
