pub mod greedy;

pub trait Solution {
    fn max_sum_of_squares(num: i32, sum: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((2, 3), "30"), ((2, 17), "98"), ((1, 10), ""), ((1, 9), "9")];

        for ((num, sum), expected) in test_cases {
            assert_eq!(S::max_sum_of_squares(num, sum), expected);
        }
    }
}
