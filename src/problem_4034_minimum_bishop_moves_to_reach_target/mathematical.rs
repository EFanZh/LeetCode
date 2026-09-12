pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_bishop_moves(source: Vec<i32>, target: Vec<i32>) -> i32 {
        let [x_1, y_1] = <[_; 2]>::try_from(source).ok().unwrap();
        let [x_2, y_2] = <[_; 2]>::try_from(target).ok().unwrap();

        if ((x_1 ^ y_1) ^ (x_2 ^ y_2)) & 1 == 0 {
            1 << u8::from(x_1.abs_diff(x_2) != y_1.abs_diff(y_2))
        } else {
            -1
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_bishop_moves(source: Vec<i32>, target: Vec<i32>) -> i32 {
        Self::min_bishop_moves(source, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
