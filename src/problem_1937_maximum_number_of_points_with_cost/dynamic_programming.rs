pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i64 {
        let columns = points.first().map_or(0, Vec::len);
        let mut buffer = vec![0_u64; columns * 2].into_boxed_slice();
        let (mut cache, mut temp) = buffer.split_at_mut(columns);

        for row in points {
            let mut left_max = 0;

            for (i, (target, &top)) in (0..).zip(temp.iter_mut().zip(&*cache)) {
                left_max = left_max.max(top + i);
                *target = left_max - i;
            }

            let mut right_max = 0;

            for (i, ((target, &top), &value)) in (0..).zip(temp.iter_mut().zip(&*cache).zip(&row).rev()) {
                right_max = right_max.max(top + i);
                *target = (*target).max(right_max - i) + u64::from(value as u32);
            }

            mem::swap(&mut cache, &mut temp);
        }

        cache.iter().copied().max().unwrap() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_points(points: Vec<Vec<i32>>) -> i64 {
        Self::max_points(points)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
