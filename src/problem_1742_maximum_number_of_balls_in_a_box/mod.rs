pub mod iterative;

pub trait Solution {
    fn count_balls(low_limit: i32, high_limit: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((1, 10), 2), ((5, 15), 2), ((19, 28), 2), ((11, 104), 9)];

        for ((low_limit, high_limit), expected) in test_cases {
            assert_eq!(S::count_balls(low_limit, high_limit), expected);
        }
    }
}
