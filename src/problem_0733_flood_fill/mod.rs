pub mod bfs;

pub trait Solution {
    fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[[1, 1, 1], [1, 1, 0], [1, 0, 1]] as &dyn Matrix<_>, 1, 1, 2),
                &[[2, 2, 2], [2, 2, 0], [2, 0, 1]] as &dyn Matrix<_>,
            ),
            (
                (&[[1, 1, 1], [1, 1, 0], [1, 0, 1]], 1, 1, 1),
                &[[1, 1, 1], [1, 1, 0], [1, 0, 1]],
            ),
        ];

        for ((image, sr, sc, new_color), expected) in test_cases {
            assert_eq!(S::flood_fill(image.to_vec(), sr, sc, new_color), expected);
        }
    }
}
