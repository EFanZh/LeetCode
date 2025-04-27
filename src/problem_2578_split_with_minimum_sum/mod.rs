pub mod greedy;

pub trait Solution {
    fn split_num(num: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(4325, 59), (687, 75)];

        for (num, expected) in test_cases {
            assert_eq!(S::split_num(num), expected);
        }
    }
}
