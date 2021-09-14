pub mod find_cycles;

pub trait Solution {
    fn min_swaps_couples(row: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[0, 2, 1, 3] as &[_], 1), (&[3, 2, 0, 1], 0)];

        for (row, expected) in test_cases {
            assert_eq!(S::min_swaps_couples(row.to_vec()), expected);
        }
    }
}
