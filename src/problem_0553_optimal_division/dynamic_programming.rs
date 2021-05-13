pub struct Solution;

impl Solution {
    fn select_min(item: &(f64, usize, f64, usize)) -> usize {
        item.1
    }

    fn select_max(item: &(f64, usize, f64, usize)) -> usize {
        item.3
    }

    fn build_expression(
        nums: &[i32],
        cache: &[(f64, usize, f64, usize)],
        start: usize,
        length: usize,
        selector_1: impl FnOnce(&(f64, usize, f64, usize)) -> usize + Copy,
        selector_2: impl FnOnce(&(f64, usize, f64, usize)) -> usize + Copy,
        target: &mut String,
    ) {
        use std::fmt::Write;

        if length == 1 {
            write!(target, "{}", nums[start]).unwrap();
        } else {
            let n = nums.len();
            let left_length = selector_1(&cache[n * (length - 1) + start]);
            let right_start = start + left_length;
            let right_length = length - left_length;

            Self::build_expression(nums, cache, start, left_length, selector_1, selector_2, target);

            target.push('/');

            if right_length == 1 {
                Self::build_expression(nums, cache, right_start, right_length, selector_2, selector_1, target);
            } else {
                target.push('(');
                Self::build_expression(nums, cache, right_start, right_length, selector_2, selector_1, target);
                target.push(')');
            }
        }
    }

    pub fn optimal_division(nums: Vec<i32>) -> String {
        let n = nums.len();
        let mut cache = vec![(0.0, 0, 0.0, 0); n * n];

        for (slot, &num) in cache.iter_mut().zip(&nums) {
            *slot = (num.into(), 0, num.into(), 0);
        }

        for length in 2..=n {
            let (top, row) = cache[..n * (length - 1) + (n + 1 - length)].split_at_mut(n * (length - 1));

            for (start, slot) in row.iter_mut().enumerate() {
                let mut min = std::f64::INFINITY;
                let mut min_left_length = 0;
                let mut max = std::f64::NEG_INFINITY;
                let mut max_left_length = 0;

                for left_length in 1..length {
                    let (left_min, _, left_max, _) = top[n * (left_length - 1) + start];
                    let (right_min, _, right_max, _) = top[n * (length - left_length - 1) + start + left_length];
                    let current_min = left_min / right_max;

                    if current_min < min {
                        min = current_min;
                        min_left_length = left_length;
                    }

                    let current_max = left_max / right_min;

                    if current_max > max {
                        max = current_max;
                        max_left_length = left_length;
                    }
                }

                *slot = (min, min_left_length, max, max_left_length);
            }
        }

        let mut result = String::new();

        Self::build_expression(
            &nums,
            &cache,
            0,
            nums.len(),
            Self::select_max,
            Self::select_min,
            &mut result,
        );

        result
    }
}

impl super::Solution for Solution {
    fn optimal_division(nums: Vec<i32>) -> String {
        Self::optimal_division(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
