pub mod dynamic_programming;

pub trait Solution {
    fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[-2, -3, 3], [-5, -10, 1], [10, 30, -5]] as &dyn Matrix<_>, 7),
            (&[[1, -2, 3], [2, -2, -2]], 2),
        ];

        for (dungeon, expected) in test_cases {
            assert_eq!(S::calculate_minimum_hp(dungeon.to_vec()), expected);
        }
    }
}
