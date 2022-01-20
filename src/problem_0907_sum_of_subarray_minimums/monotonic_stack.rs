pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        let mut left_boundaries = Vec::with_capacity(arr.len());
        let mut stack = Vec::new();

        for (i, &value) in (0_u32..).zip(&arr) {
            let left_boundary = loop {
                if let Some(&(top_index, top)) = stack.last() {
                    if top < value {
                        break top_index;
                    }

                    stack.pop();
                } else {
                    break u32::MAX;
                }
            };

            left_boundaries.push(left_boundary);
            stack.push((i, value));
        }

        let n = arr.len() as u32;
        let mut result = 0;

        stack.clear();

        for ((i, &value), &left_boundary) in (0..n).zip(&arr).zip(&left_boundaries).rev() {
            let right_boundary = loop {
                if let Some(&(top_index, top)) = stack.last() {
                    if top <= value {
                        break top_index;
                    }

                    stack.pop();
                } else {
                    break n;
                }
            };

            result += u64::from(value as u32) * u64::from((i.wrapping_sub(left_boundary)) * (right_boundary - i));

            stack.push((i, value));
        }

        (result % 1_000_000_007) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        Self::sum_subarray_mins(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
