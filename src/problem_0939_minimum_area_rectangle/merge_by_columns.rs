pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::HashMap;
use std::convert::TryInto;
use std::iter;

impl Solution {
    fn get_aligned_points<'a>(left: &'a [i32], right: &'a [i32]) -> impl Iterator<Item = i32> + 'a {
        let mut left_iter = left.iter().copied();
        let mut right_iter = right.iter().copied();

        iter::from_fn(move || loop {
            if let Some(mut left) = left_iter.next() {
                if let Some(mut right) = right_iter.next() {
                    loop {
                        match left.cmp(&right) {
                            Ordering::Less => {
                                if let Some(next_left) = left_iter.next() {
                                    left = next_left;
                                } else {
                                    return None;
                                }
                            }
                            Ordering::Equal => return Some(left),
                            Ordering::Greater => {
                                if let Some(next_right) = right_iter.next() {
                                    right = next_right;
                                } else {
                                    return None;
                                }
                            }
                        }
                    }
                } else {
                    return None;
                }
            } else {
                return None;
            }
        })
    }

    fn get_min_gap(left: &[i32], right: &[i32]) -> Option<i32> {
        let mut iter = Self::get_aligned_points(left, right);
        let first = iter.next()?;

        iter.next().map(|second| {
            let mut result = second - first;
            let mut prev = second;

            for y in iter {
                result = result.min(y - prev);
                prev = y;
            }

            result
        })
    }

    pub fn min_area_rect(points: Vec<Vec<i32>>) -> i32 {
        let mut points_by_x = HashMap::new();

        for point in &points {
            let [x, y]: [_; 2] = point.as_slice().try_into().unwrap();

            points_by_x
                .entry(x)
                .and_modify(|ys| Vec::push(ys, y))
                .or_insert_with(|| vec![y]);
        }

        let mut points = points_by_x.into_iter().collect::<Vec<_>>();

        for (_, ys) in &mut points {
            ys.sort_unstable();
        }

        points.sort_unstable_by_key(|&(x, _)| x);

        let mut result = i32::MAX;

        for (i, (right, right_points)) in points.iter().enumerate() {
            let right = *right;

            for (left, left_points) in points[..i].iter().rev() {
                let left = *left;

                if left > right - result {
                    if let Some(gap) = Self::get_min_gap(left_points, right_points) {
                        result = result.min((right - left) * gap);
                    }
                } else {
                    break;
                }
            }
        }

        if result == i32::MAX {
            0
        } else {
            result
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_area_rect(points: Vec<Vec<i32>>) -> i32 {
        Self::min_area_rect(points)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
