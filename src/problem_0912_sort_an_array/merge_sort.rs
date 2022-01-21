pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn merge(left: &[i32], right: &[i32], buffer: &mut [i32]) {
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

    fn merge_sort_into_buffer(nums: &mut [i32], buffer: &mut [i32]) {
        if nums.len() > 1 {
            let half = nums.len() / 2;
            let (left, right) = nums.split_at_mut(half);
            let (left_buffer, right_buffer) = buffer.split_at_mut(half);

            Self::merge_sort_into_self(left, left_buffer);
            Self::merge_sort_into_self(right, right_buffer);
            Self::merge(left, right, buffer);
        } else {
            buffer.copy_from_slice(nums);
        }
    }

    fn merge_sort_into_self(nums: &mut [i32], buffer: &mut [i32]) {
        if nums.len() > 1 {
            let half = nums.len() / 2;
            let (left, right) = nums.split_at_mut(half);
            let (left_buffer, right_buffer) = buffer.split_at_mut(half);

            Self::merge_sort_into_buffer(left, left_buffer);
            Self::merge_sort_into_buffer(right, right_buffer);
            Self::merge(left_buffer, right_buffer, nums);
        }
    }

    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut buffer = vec![0; nums.len()];

        Self::merge_sort_into_self(&mut nums, &mut buffer);

        nums
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        Self::sort_array(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
