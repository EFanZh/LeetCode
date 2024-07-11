pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut iter = points.iter().map(|p| {
            let [x, y]: [_; 2] = p.as_slice().try_into().unwrap();

            (x, y)
        });

        let mut result = 0;
        let mut prev = iter.next().unwrap();

        for point in iter {
            let x_diff = point.0 - prev.0;
            let y_diff = point.1 - prev.1;

            result += x_diff.abs().max(y_diff.abs());

            prev = point;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        Self::min_time_to_visit_all_points(points)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
