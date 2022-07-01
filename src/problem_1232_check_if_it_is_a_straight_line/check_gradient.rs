pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::{TryFrom, TryInto};

impl Solution {
    fn get_point(p: &[i32]) -> [i32; 2] {
        p.try_into().unwrap()
    }

    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let (left, right) = coordinates.split_at(2);
        let [p_0, p_1] = <&[_; 2]>::try_from(left).unwrap();
        let [x_0, y_0] = Self::get_point(p_0);
        let [x_1, y_1] = Self::get_point(p_1);

        let d_x = x_1 - x_0;
        let d_y = y_1 - y_0;

        right.iter().all(|p| {
            let [x, y] = Self::get_point(p);

            (y - y_0) * d_x == d_y * (x - x_0)
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        Self::check_straight_line(coordinates)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
