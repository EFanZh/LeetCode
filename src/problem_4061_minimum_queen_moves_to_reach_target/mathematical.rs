pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_queen_moves(source: Vec<i32>, target: Vec<i32>) -> i32 {
        let [x_1, y_1] = <[_; 2]>::try_from(source).ok().unwrap();
        let [x_2, y_2] = <[_; 2]>::try_from(target).ok().unwrap();

        #[expect(clippy::suspicious_operation_groupings, reason = "by design")]
        if x_1 == x_2 {
            if y_1 == y_2 {
                return 0;
            }
        } else if y_1 != y_2 && x_1.abs_diff(x_2) != y_1.abs_diff(y_2) {
            return 2;
        }

        1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_queen_moves(source: Vec<i32>, target: Vec<i32>) -> i32 {
        Self::min_queen_moves(source, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
