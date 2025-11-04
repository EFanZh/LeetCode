pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn can_make_square(grid: Vec<Vec<char>>) -> bool {
        let [[m00, m01, m02], [m10, m11, m12], [m20, m21, m22]] = <[_; 3]>::try_from(grid)
            .ok()
            .unwrap()
            .map(|row| <[_; 3]>::try_from(row).ok().unwrap());

        [
            [m00, m01, m10, m11],
            [m01, m02, m11, m12],
            [m10, m11, m20, m21],
            [m11, m12, m21, m22],
        ]
        .iter()
        .any(|values| values.iter().filter(|&&c| c == 'B').count() != 2)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn can_make_square(grid: Vec<Vec<char>>) -> bool {
        Self::can_make_square(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
