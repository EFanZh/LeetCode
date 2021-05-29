pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn merge_sort_to_self(sums: &mut [i64], buffer: &mut [i64], lower: i32, upper: i32, result: &mut i32) {
        if sums.len() > 1 {
            let half = sums.len() / 2;
            let (left_sums, right_sums) = sums.split_at_mut(half);
            let (left_buffer, right_buffer) = buffer.split_at_mut(half);

            Self::merge_sort_to_buffer(left_sums, left_buffer, lower, upper, result);
            Self::merge_sort_to_buffer(right_sums, right_buffer, lower, upper, result);
            Self::merge_to_buffer(left_buffer, right_buffer, sums, lower, upper, result);
        }
    }

    fn merge_sort_to_buffer(sums: &mut [i64], buffer: &mut [i64], lower: i32, upper: i32, result: &mut i32) {
        if sums.len() > 1 {
            let half = sums.len() / 2;
            let (left_sums, right_sums) = sums.split_at_mut(half);
            let (left_buffer, right_buffer) = buffer.split_at_mut(half);

            Self::merge_sort_to_self(left_sums, left_buffer, lower, upper, result);
            Self::merge_sort_to_self(right_sums, right_buffer, lower, upper, result);
            Self::merge_to_buffer(left_sums, right_sums, buffer, lower, upper, result);
        } else {
            buffer.copy_from_slice(sums);
        }
    }

    fn count_range(left_sums: &[i64], right_sums: &[i64], lower: i32, upper: i32, result: &mut i32) {
        let mut i = 0;
        let mut j = 0;

        'outer: for left_sum in left_sums {
            let right_lower = left_sum + i64::from(lower);
            let right_upper = left_sum + i64::from(upper);

            loop {
                if let Some(&right_sum) = right_sums.get(i) {
                    if right_sum < right_lower {
                        i += 1;
                    } else {
                        break;
                    }
                } else {
                    break 'outer;
                }
            }

            j = j.max(i);

            while let Some(&right_sum) = right_sums.get(j) {
                if right_sum <= right_upper {
                    j += 1;
                } else {
                    break;
                }
            }

            *result += (j - i) as i32;
        }
    }

    fn merge_to_buffer(
        left_sums: &[i64],
        right_sums: &[i64],
        buffer: &mut [i64],
        lower: i32,
        upper: i32,
        result: &mut i32,
    ) {
        Self::count_range(left_sums, right_sums, lower, upper, result);

        let mut i = 0;
        let mut j = 0;
        let mut k = 0;
        let mut left_sum = left_sums[0];
        let mut right_sum = right_sums[0];

        loop {
            if right_sum < left_sum {
                buffer[k] = right_sum;

                j += 1;
                k += 1;

                if let Some(&next_right_sum) = right_sums.get(j) {
                    right_sum = next_right_sum;
                } else {
                    buffer[k..].copy_from_slice(&left_sums[i..]);

                    break;
                }
            } else {
                buffer[k] = left_sum;

                i += 1;
                k += 1;

                if let Some(&next_left_sum) = left_sums.get(i) {
                    left_sum = next_left_sum;
                } else {
                    buffer[k..].copy_from_slice(&right_sums[j..]);

                    break;
                }
            }
        }
    }

    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let mut sum = 0;
        let mut prefix_sums = Vec::with_capacity(nums.len() + 1);

        prefix_sums.push(0_i64);

        prefix_sums.extend(nums.into_iter().map(|num| {
            sum += i64::from(num);

            sum
        }));

        let mut buffer = vec![0; prefix_sums.len()];
        let mut result = 0;

        Self::merge_sort_to_self(&mut prefix_sums, &mut buffer, lower, upper, &mut result);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        Self::count_range_sum(nums, lower, upper)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
