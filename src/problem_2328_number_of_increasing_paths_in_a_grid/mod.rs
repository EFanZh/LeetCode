pub mod bfs;

pub trait Solution {
    fn count_paths(grid: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 1], [3, 4]] as &dyn Matrix<_>, 8),
            (&[[1], [2]], 3),
            (
                &[[
                    12469, 18741, 68716, 30594, 65029, 44019, 92944, 84784, 92781, 5655, 43120, 81333, 54113, 88220,
                    23446, 6129, 2904, 48677, 20506, 79604, 82841, 3938, 46511, 60870, 10825, 31759, 78612, 19776,
                    43160, 86915, 74498, 38366, 28228, 23687, 40729, 42613, 61154, 22726, 51028, 45603, 53586, 44657,
                    97573, 61067, 27187, 4619, 6135, 24668, 69634, 24564, 30255, 51939, 67573, 87012, 4106, 76312,
                    28737, 7704, 35798,
                ]],
                148,
            ),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::count_paths(grid.to_vec()), expected);
        }
    }
}
