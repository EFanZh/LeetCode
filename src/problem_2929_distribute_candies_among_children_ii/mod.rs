pub mod mathematical;

pub trait Solution {
    fn distribute_candies(n: i32, limit: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((5, 2), 3), ((3, 3), 10)];

        for ((n, limit), expected) in test_cases {
            assert_eq!(S::distribute_candies(n, limit), expected);
        }
    }
}
