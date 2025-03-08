pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        let mut result = -1;
        let mut min_distance = i32::MAX;

        for (i, point) in (0..).zip(&points) {
            let [p_x, p_y] = point.as_slice().try_into().ok().unwrap();

            let distance = if p_x == x {
                (p_y - y).abs()
            } else if p_y == y {
                (p_x - x).abs()
            } else {
                continue;
            };

            if distance < min_distance {
                min_distance = distance;
                result = i;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        Self::nearest_valid_point(x, y, points)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
