pub mod mathematical;

pub trait Solution {
    fn min_bishop_moves(source: Vec<i32>, target: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(([8, 1], [1, 8]), 1), (([4, 2], [1, 3]), 2), (([1, 1], [3, 4]), -1)];

        for ((source, target), expected) in test_cases {
            assert_eq!(S::min_bishop_moves(source.to_vec(), target.to_vec()), expected);
        }
    }
}
