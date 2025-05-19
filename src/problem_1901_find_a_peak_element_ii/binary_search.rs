pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn check(mat: &[Vec<i32>], i: usize) -> bool {
        let current_max = mat[i].iter().copied().max().unwrap();
        let next_max = mat[i + 1].iter().copied().max().unwrap();

        current_max < next_max
    }

    pub fn find_peak_grid(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut left = 0;
        let mut right = mat.len() - 1;

        while left < right {
            let middle = usize::midpoint(left, right);

            if Self::check(&mat, middle) {
                left = middle + 1;
            } else {
                right = middle;
            }
        }

        let mut max_x = 0;
        let mut max_value = 0;

        for (i, &value) in mat[left].iter().enumerate() {
            if value > max_value {
                max_x = i;
                max_value = value;
            }
        }

        vec![left as _, max_x as _]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_peak_grid(mat: Vec<Vec<i32>>) -> Vec<i32> {
        Self::find_peak_grid(mat)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
