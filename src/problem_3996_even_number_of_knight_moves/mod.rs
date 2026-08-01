pub mod mathematical;

pub trait Solution {
    fn can_reach(start: Vec<i32>, target: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(([1, 1], [2, 2]), true), (([4, 5], [6, 6]), false)];

        for ((start, target), expected) in test_cases {
            assert_eq!(S::can_reach(start.to_vec(), target.to_vec()), expected);
        }
    }
}
