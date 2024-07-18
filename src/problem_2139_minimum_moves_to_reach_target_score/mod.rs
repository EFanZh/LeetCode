pub mod greedy;

pub trait Solution {
    fn min_moves(target: i32, max_doubles: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((5, 0), 4), ((19, 2), 7), ((10, 4), 4), ((1, 100), 0)];

        for ((target, max_doubles), expected) in test_cases {
            assert_eq!(S::min_moves(target, max_doubles), expected);
        }
    }
}
