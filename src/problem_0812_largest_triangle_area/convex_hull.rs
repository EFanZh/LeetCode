pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::convert::TryInto;

impl Solution {
    fn vec_subtract((lhs_x, lhs_y): (i32, i32), (rhs_x, rhs_y): (i32, i32)) -> (i32, i32) {
        (lhs_x - rhs_x, lhs_y - rhs_y)
    }

    fn compare_vec_direction((lhs_x, lhs_y): (i32, i32), (rhs_x, rhs_y): (i32, i32)) -> Ordering {
        (rhs_x * lhs_y).cmp(&(lhs_x * rhs_y))
    }

    fn deduplicate_by_angle(points: &mut Vec<(i32, i32)>) {
        let mut length = 1;
        let mut prev = points[1];

        for i in 2..points.len() {
            let point = points[i];

            if Self::compare_vec_direction(point, prev) == Ordering::Equal {
                if point.0.abs() > prev.0.abs() || point.1.abs() > prev.1.abs() {
                    prev = point;
                }
            } else {
                points[length] = prev;
                length += 1;
                prev = point;
            }
        }

        points[length] = prev;

        length += 1;

        points.truncate(length);
    }

    fn sort_points(points: &[Vec<i32>]) -> Vec<(i32, i32)> {
        let mut points = points
            .iter()
            .map(|p| {
                let [x, y]: [i32; 2] = p.as_slice().try_into().unwrap();

                (x, y)
            })
            .collect::<Vec<_>>();

        // Find the left most point of the bottom row.

        let (pivot_index, pivot) = points
            .iter()
            .copied()
            .enumerate()
            .min_by_key(|&(_, (x, y))| (y, x))
            .unwrap();

        points.swap(0, pivot_index);

        // Move pivot to `(0, 0)`.

        for point in &mut points {
            point.0 -= pivot.0;
            point.1 -= pivot.1;
        }

        //  Sort points by polar angle.

        points[1..].sort_unstable_by(|&lhs, &rhs| Self::compare_vec_direction(lhs, rhs));

        // Deduplicate.

        Self::deduplicate_by_angle(&mut points);

        points
    }

    fn calculate_convex_hull(points: Vec<Vec<i32>>) -> Vec<(i32, i32)> {
        let points = Self::sort_points(&points);
        let (left, right) = points.split_at(3);
        let mut stack = left.to_vec();

        for &p in right {
            while let Some(&[p_0, p_1]) = stack.get(stack.len().wrapping_sub(2)..) {
                let d_1 = Self::vec_subtract(p_1, p_0);
                let d = Self::vec_subtract(p, p_0);

                if Self::compare_vec_direction(d_1, d) == Ordering::Greater {
                    stack.pop();
                } else {
                    break;
                }
            }

            stack.push(p);
        }

        stack
    }

    fn calculate_triangle_area_times_2(p0: (i32, i32), p1: (i32, i32), p2: (i32, i32)) -> i32 {
        p0.0 * (p1.1 - p2.1) + p1.0 * (p2.1 - p0.1) + p2.0 * (p0.1 - p1.1)
    }

    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let points = Self::calculate_convex_hull(points);
        let mut max_area_times_2 = 0;

        for (i, &p0) in points.iter().enumerate() {
            let mut k = i + 2;

            for &p1 in &points[i + 1..] {
                let mut prev_area_times_2 = 0;

                while let Some(&p2) = points.get(k) {
                    let area_times_2 = Self::calculate_triangle_area_times_2(p0, p1, p2);

                    if area_times_2 < prev_area_times_2 {
                        break;
                    }

                    prev_area_times_2 = area_times_2;
                    max_area_times_2 = max_area_times_2.max(area_times_2);
                    k += 1;
                }

                k -= 1;
            }
        }

        f64::from(max_area_times_2) * 0.5
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        Self::largest_triangle_area(points)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
