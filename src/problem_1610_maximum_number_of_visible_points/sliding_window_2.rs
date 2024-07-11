pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::f64::consts;

impl Solution {
    fn parse_point(point: Vec<i32>) -> [i32; 2] {
        point.as_slice().try_into().ok().unwrap()
    }

    fn to_angles(points: Vec<Vec<i32>>, location: Vec<i32>) -> (usize, Vec<f64>) {
        let [center_x, center_y]: [_; 2] = Self::parse_point(location);
        let mut extra = 0;
        let mut result = Vec::with_capacity(points.len());

        result.extend(points.into_iter().filter_map(|point| {
            let [x, y]: [_; 2] = Self::parse_point(point);

            if x == center_x && y == center_y {
                extra += 1;

                None
            } else {
                Some(f64::from(y - center_y).atan2(f64::from(x - center_x)))
            }
        }));

        result.sort_unstable_by(|lhs, rhs| lhs.partial_cmp(rhs).unwrap());

        (extra, result)
    }

    pub fn visible_points(points: Vec<Vec<i32>>, angle: i32, location: Vec<i32>) -> i32 {
        let angle = f64::from(angle).to_radians();
        let (extra, angles) = Self::to_angles(points, location);
        let mut result = 0;
        let mut start = 0;
        let mut i = 0;

        for x in &angles {
            i += 1;

            let min_angle = x - angle;

            while angles[start] < min_angle {
                start += 1;
            }

            result = result.max(i - start);
        }

        let diff = consts::TAU - angle;

        'outer: for x in &angles {
            i += 1;

            let min_angle = x + diff;

            loop {
                if let Some(&left_angle) = angles.get(start) {
                    if left_angle < min_angle {
                        start += 1;
                    } else {
                        break;
                    }
                } else {
                    break 'outer;
                }
            }

            result = result.max(i - start);
        }

        (result + extra) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn visible_points(points: Vec<Vec<i32>>, angle: i32, location: Vec<i32>) -> i32 {
        Self::visible_points(points, angle, location)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
