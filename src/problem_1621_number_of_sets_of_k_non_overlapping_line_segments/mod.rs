pub mod combinations;

pub trait Solution {
    fn number_of_sets(n: i32, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((4, 2), 5), ((3, 1), 3), ((30, 7), 796_297_179)];

        for ((n, k), expected) in test_cases {
            assert_eq!(S::number_of_sets(n, k), expected);
        }
    }
}
