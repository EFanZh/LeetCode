pub mod mathematical;

pub trait Solution {
    fn minimize_set(divisor1: i32, divisor2: i32, unique_cnt1: i32, unique_cnt2: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((2, 7, 1, 3), 4), ((3, 5, 2, 1), 3), ((2, 4, 8, 2), 15)];

        for ((divisor1, divisor2, unique_cnt1, unique_cnt2), expected) in test_cases {
            assert_eq!(S::minimize_set(divisor1, divisor2, unique_cnt1, unique_cnt2), expected);
        }
    }
}
