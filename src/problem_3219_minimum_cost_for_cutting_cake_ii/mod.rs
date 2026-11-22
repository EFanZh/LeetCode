pub mod dynamic_programming;

pub trait Solution {
    fn minimum_cost(m: i32, n: i32, horizontal_cut: Vec<i32>, vertical_cut: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((3, 2, &[1, 3] as &[_], &[5]), 13), ((2, 2, &[7] as &[_], &[4]), 15)];

        for ((m, n, horizontal_cut, vertical_cut), expected) in test_cases {
            assert_eq!(
                S::minimum_cost(m, n, horizontal_cut.to_vec(), vertical_cut.to_vec()),
                expected,
            );
        }
    }
}
