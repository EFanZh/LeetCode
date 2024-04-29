pub mod iterative;

pub trait Solution {
    fn rearrange_sticks(n: i32, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((3, 2), 3), ((5, 5), 1)];

        for ((n, k), expected) in test_cases {
            assert_eq!(S::rearrange_sticks(n, k), expected);
        }
    }
}
