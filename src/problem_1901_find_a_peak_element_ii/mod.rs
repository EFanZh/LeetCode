pub mod binary_search;

pub trait Solution {
    fn find_peak_grid(mat: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            &[[1, 4], [3, 2]] as &dyn Matrix<_>,
            &[[10, 20, 15], [21, 30, 14], [7, 16, 32]],
            &[
                [41, 8, 2, 48, 18],
                [16, 15, 9, 7, 44],
                [48, 35, 6, 38, 28],
                [3, 2, 14, 15, 33],
                [39, 36, 13, 46, 42],
            ],
        ];

        for mat in test_cases {
            let mat = mat.to_vec();
            let [y, x]: [_; 2] = S::find_peak_grid(mat.clone()).try_into().ok().unwrap();
            let y = y as usize;
            let x = x as usize;
            let center = mat[y][x];

            assert!(center > mat.get(y.wrapping_sub(1)).map_or(-1, |row| row[x]));
            assert!(center > mat[y].get(x.wrapping_sub(1)).copied().unwrap_or(-1));
            assert!(center > mat[y].get(x + 1).copied().unwrap_or(-1));
            assert!(center > mat.get(y + 1).map_or(-1, |row| row[x]));
        }
    }
}
