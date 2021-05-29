pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn merge_sort_to_self(indices: &mut [usize], buffer: &mut [usize], nums: &[i32], result: &mut [i32]) {
        if indices.len() > 1 {
            let half = indices.len() / 2;
            let (left, right) = indices.split_at_mut(half);
            let (left_buffer, right_buffer) = buffer.split_at_mut(half);

            Self::merge_sort_to_buffer(left, left_buffer, nums, result);
            Self::merge_sort_to_buffer(right, right_buffer, nums, result);

            Self::merge_to_buffer(left_buffer, right_buffer, indices, nums, result);
        }
    }

    fn merge_sort_to_buffer(indices: &mut [usize], buffer: &mut [usize], nums: &[i32], result: &mut [i32]) {
        if indices.len() > 1 {
            let half = indices.len() / 2;
            let (left, right) = indices.split_at_mut(half);
            let (left_buffer, right_buffer) = buffer.split_at_mut(half);

            Self::merge_sort_to_self(left, left_buffer, nums, result);
            Self::merge_sort_to_self(right, right_buffer, nums, result);
            Self::merge_to_buffer(left, right, buffer, nums, result);
        } else {
            buffer.copy_from_slice(indices);
        }
    }

    fn merge_to_buffer(left: &[usize], right: &[usize], buffer: &mut [usize], nums: &[i32], result: &mut [i32]) {
        let mut i = 0;
        let mut j = 0;
        let mut k = 0;
        let mut left_index = left[0];
        let mut right_index = right[0];

        loop {
            if nums[right_index] < nums[left_index] {
                buffer[k] = right_index;
                k += 1;
                j += 1;

                if let Some(&next_right_index) = right.get(j) {
                    right_index = next_right_index;
                } else {
                    for &left_index in &left[i..] {
                        result[left_index] += j as i32;
                    }

                    buffer[k..].copy_from_slice(&left[i..]);

                    break;
                }
            } else {
                buffer[k] = left_index;
                result[left_index] += j as i32;
                k += 1;
                i += 1;

                if let Some(&next_left_index) = left.get(i) {
                    left_index = next_left_index;
                } else {
                    buffer[k..].copy_from_slice(&right[j..]);

                    break;
                }
            }
        }
    }

    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![0; n];

        Self::merge_sort_to_self(&mut (0..n).collect::<Box<_>>(), &mut vec![0; n], &nums, &mut result);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        Self::count_smaller(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
