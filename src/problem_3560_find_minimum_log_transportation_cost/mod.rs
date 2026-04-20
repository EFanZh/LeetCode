pub mod greedy;

pub trait Solution {
    fn min_cutting_cost(n: i32, m: i32, k: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((6, 5, 5), 5), ((4, 4, 6), 0)];

        for ((n, m, k), expected) in test_cases {
            assert_eq!(S::min_cutting_cost(n, m, k), expected);
        }
    }
}
