pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryFrom;

impl Solution {
    pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
        let [p_0, p_1, p_2] = <[_; 3]>::try_from(points).unwrap();
        let [x_0, y_0] = <[_; 2]>::try_from(p_0).unwrap();
        let [x_1, y_1] = <[_; 2]>::try_from(p_1).unwrap();
        let [x_2, y_2] = <[_; 2]>::try_from(p_2).unwrap();

        if x_1 == x_0 {
            (y_1 != y_0) && (x_2 != x_0)
        } else {
            (x_1 - x_0) * (y_2 - y_0) != (x_2 - x_0) * (y_1 - y_0)
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
        Self::is_boomerang(points)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
