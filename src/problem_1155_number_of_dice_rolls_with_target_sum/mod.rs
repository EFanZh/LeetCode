pub mod dynamic_programming;

pub trait Solution {
    fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((1, 6, 3), 1), ((2, 6, 7), 6), ((30, 30, 500), 222_616_187)];

        for ((n, k, target), expected) in test_cases {
            assert_eq!(S::num_rolls_to_target(n, k, target), expected);
        }
    }
}
