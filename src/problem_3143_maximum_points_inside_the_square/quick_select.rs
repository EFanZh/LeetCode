pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn partition<T>(values: &mut [T], mut f: impl FnMut(&T) -> bool) -> usize {
        let mut result = 0;
        let mut iter = values.iter_mut();

        'outer: while let Some(left) = iter.next() {
            if !f(left) {
                loop {
                    if let Some(right) = iter.next_back() {
                        if f(right) {
                            mem::swap(left, right);

                            break;
                        }
                    } else {
                        break 'outer;
                    }
                }
            }

            result += 1;
        }

        result
    }

    pub fn max_points_inside_square(points: Vec<Vec<i32>>, s: String) -> i32 {
        let mut points = points
            .into_iter()
            .zip(s.into_bytes())
            .map(|(point, tag)| {
                let [x, y] = point.try_into().unwrap();

                (x.unsigned_abs().max(y.unsigned_abs()), 1 << (tag - b'a'))
            })
            .collect::<Vec<_>>();

        let sort_key = |&(x, _): &_| x;

        if points.len() < 27 {
            points.sort_unstable_by_key(sort_key);
        } else {
            let key = points.select_nth_unstable_by_key(25, sort_key).1.0;
            let n = Self::partition(&mut points[26..], |&(x, _)| x == key);

            points.truncate(26 + n);
            points[..25].sort_unstable_by_key(sort_key);
        }

        points.dedup_by(|right, left| {
            let is_same = left.0 == right.0;

            if is_same {
                if left.1 & right.1 == 0 {
                    left.1 |= right.1;
                } else {
                    left.1 = u32::MAX;
                }
            }

            is_same
        });

        let mut bits = 0;

        for point in points {
            if point.1 != u32::MAX && point.1 & bits == 0 {
                bits |= point.1;
            } else {
                break;
            }
        }

        bits.count_ones().cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_points_inside_square(points: Vec<Vec<i32>>, s: String) -> i32 {
        Self::max_points_inside_square(points, s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
