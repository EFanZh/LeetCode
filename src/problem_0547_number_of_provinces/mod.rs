pub mod bfs;

pub trait Solution {
    fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 1, 0], [1, 1, 0], [0, 0, 1]] as &dyn Matrix<_>, 2),
            (&[[1, 0, 0], [0, 1, 0], [0, 0, 1]], 3),
        ];

        for (is_connected, expected) in test_cases {
            assert_eq!(S::find_circle_num(is_connected.to_vec()), expected);
        }
    }
}
