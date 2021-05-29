pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn merge_sort_to_self(nums: &mut [i32], buffer: &mut [i32], result: &mut i32) {
        if nums.len() > 1 {
            let half = nums.len() / 2;
            let (left, right) = nums.split_at_mut(half);
            let (left_buffer, right_buffer) = buffer.split_at_mut(half);

            Self::merge_sort_to_buffer(left, left_buffer, result);
            Self::merge_sort_to_buffer(right, right_buffer, result);
            Self::count_reverse_pairs(left_buffer, right_buffer, result);
            Self::merge_to_buffer(left_buffer, right_buffer, nums);
        }
    }

    fn merge_sort_to_buffer(nums: &mut [i32], buffer: &mut [i32], result: &mut i32) {
        if nums.len() > 1 {
            let half = nums.len() / 2;
            let (left, right) = nums.split_at_mut(half);
            let (left_buffer, right_buffer) = buffer.split_at_mut(half);

            Self::merge_sort_to_self(left, left_buffer, result);
            Self::merge_sort_to_self(right, right_buffer, result);
            Self::count_reverse_pairs(left, right, result);
            Self::merge_to_buffer(left, right, buffer);
        } else {
            buffer.copy_from_slice(nums);
        }
    }

    fn count_reverse_pairs(left: &[i32], right: &[i32], result: &mut i32) {
        let mut i = 0;
        let mut j = 0;
        let mut left_num = i64::from(left[i]);
        let mut right_num = i64::from(right[j]) * 2;

        loop {
            if right_num < left_num {
                j += 1;

                if let Some(&next_right_num) = right.get(j) {
                    right_num = i64::from(next_right_num) * 2;
                } else {
                    *result += (j * (left.len() - i)) as i32;

                    break;
                }
            } else {
                *result += j as i32;
                i += 1;

                if let Some(&next_left_num) = left.get(i) {
                    left_num = i64::from(next_left_num);
                } else {
                    break;
                }
            }
        }
    }

    fn merge_to_buffer(left: &[i32], right: &[i32], buffer: &mut [i32]) {
        let mut left_iter = left.iter().copied();
        let mut right_iter = right.iter().copied();
        let mut buffer_iter = buffer.iter_mut();
        let mut left_num = left_iter.next().unwrap();
        let mut right_num = right_iter.next().unwrap();

        let rest = loop {
            if right_num < left_num {
                *buffer_iter.next().unwrap() = right_num;

                if let Some(next_right_num) = right_iter.next() {
                    right_num = next_right_num;
                } else {
                    break (left_num, left_iter);
                }
            } else {
                *buffer_iter.next().unwrap() = left_num;

                if let Some(next_left_num) = left_iter.next() {
                    left_num = next_left_num;
                } else {
                    break (right_num, right_iter);
                }
            }
        };

        *buffer_iter.next().unwrap() = rest.0;

        for (target, num) in buffer_iter.zip(rest.1) {
            *target = num;
        }
    }

    pub fn reverse_pairs(mut nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut buffer = vec![0; nums.len()];

        Self::merge_sort_to_self(&mut nums, &mut buffer, &mut result);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn reverse_pairs(nums: Vec<i32>) -> i32 {
        Self::reverse_pairs(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
