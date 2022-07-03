pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;
use std::convert::TryInto;

impl Solution {
    pub fn min_area_free_rect(points: Vec<Vec<i32>>) -> f64 {
        let points = points
            .iter()
            .map(|point| {
                let [x, y]: [_; 2] = point.as_slice().try_into().unwrap();

                (x, y)
            })
            .collect::<Vec<_>>();

        let mut iter = points.iter().copied();

        let points = iter.clone().collect::<HashSet<_>>();
        let mut result = i32::MAX;

        while let Some((x_1, y_1)) = iter.next() {
            let mut iter_2 = iter.clone();

            while let Some((x_2, y_2)) = iter_2.next() {
                for (x_3, y_3) in iter_2.clone() {
                    let squared_distance_12 = (x_2 - x_1).pow(2) + (y_2 - y_1).pow(2);
                    let squared_distance_13 = (x_3 - x_1).pow(2) + (y_3 - y_1).pow(2);
                    let squared_distance_23 = (x_3 - x_2).pow(2) + (y_3 - y_2).pow(2);

                    let (p1, p2, p3) = if squared_distance_12 + squared_distance_13 == squared_distance_23 {
                        ((x_1, y_1), (x_2, y_2), (x_3, y_3))
                    } else if squared_distance_12 + squared_distance_23 == squared_distance_13 {
                        ((x_2, y_2), (x_1, y_1), (x_3, y_3))
                    } else if squared_distance_13 + squared_distance_23 == squared_distance_12 {
                        ((x_3, y_3), (x_1, y_1), (x_2, y_2))
                    } else {
                        continue;
                    };

                    let candidate_area = (x_1 * (y_2 - y_3) + x_2 * (y_3 - y_1) + x_3 * (y_1 - y_2)).abs();

                    if candidate_area < result && points.contains(&(p3.0 + p2.0 - p1.0, p3.1 + p2.1 - p1.1)) {
                        result = candidate_area;
                    }
                }
            }
        }

        f64::from(if result == i32::MAX { 0 } else { result })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_area_free_rect(points: Vec<Vec<i32>>) -> f64 {
        Self::min_area_free_rect(points)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
