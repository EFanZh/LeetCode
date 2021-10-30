pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::convert::TryInto;

impl Solution {
    fn vec_subtract((lhs_x, lhs_y): (i32, i32), (rhs_x, rhs_y): (i32, i32)) -> (i32, i32) {
        (lhs_x - rhs_x, lhs_y - rhs_y)
    }

    fn vec_squared_norm((x, y): (i32, i32)) -> i32 {
        x * x + y * y
    }

    fn compare_vec_direction((lhs_x, lhs_y): (i32, i32), (rhs_x, rhs_y): (i32, i32)) -> Ordering {
        (rhs_x * lhs_y).cmp(&(lhs_x * rhs_y))
    }

    fn sort_points(points: &[Vec<i32>]) -> Option<Vec<(i32, i32)>> {
        let mut points = points
            .iter()
            .map(|p| {
                let [x, y]: [i32; 2] = p.as_slice().try_into().unwrap();

                (x, y)
            })
            .collect::<Vec<_>>();

        // Find the left most point of the bottom row.

        let min_index = (0..points.len())
            .min_by_key(|&i| {
                let (x, y) = points[i];

                (y, x)
            })
            .unwrap();

        points.swap(0, min_index);

        //  Sort points by polar angle.

        let p_0 = points[0];

        points[1..].sort_unstable_by(|&lhs, &rhs| {
            let diff_lhs = Self::vec_subtract(lhs, p_0);
            let diff_rhs = Self::vec_subtract(rhs, p_0);

            Self::compare_vec_direction(diff_lhs, diff_rhs)
                .then_with(|| Self::vec_squared_norm(diff_lhs).cmp(&Self::vec_squared_norm(diff_rhs)))
        });

        // Fix point order on the returning direction.

        let last_point = *points.last().unwrap();
        let last_point_vec = Self::vec_subtract(last_point, p_0);

        let i = points[..points.len() - 1].iter().rposition(|&p| {
            Self::compare_vec_direction(Self::vec_subtract(p, p_0), last_point_vec) != Ordering::Equal
        })?;

        points[i + 1..].reverse();

        Some(points)
    }

    pub fn outer_trees(points: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut points = points;

        if points.len() >= 4 {
            if let Some(sorted_points) = Self::sort_points(&points) {
                let (left, right) = sorted_points.split_at(3);
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

                points.truncate(stack.len());

                for (target, (x, y)) in points.iter_mut().zip(stack) {
                    target.as_mut_slice().copy_from_slice(&[x, y]);
                }
            }
        }

        points
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn outer_trees(points: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::outer_trees(points)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
