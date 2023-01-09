pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn merge_to_buffer(data: &[(u32, i32)], buffer: &mut [(u32, i32)]) {
        let (left, right) = data.split_at(data.len() / 2);
        let mut left_iter = left.iter().copied();
        let mut right_iter = right.iter().copied();
        let mut left = left_iter.next().unwrap();
        let mut right = right_iter.next().unwrap();
        let mut count = 0;
        let mut writer = buffer.iter_mut();
        let mut write = |value| *writer.next().unwrap() = value;

        loop {
            if right.0 <= (left.0 + count) {
                count += 1;
                write(right);

                if let Some(new_right) = right_iter.next() {
                    right = new_right;
                } else {
                    left.0 += count;
                    write(left);

                    for (target, mut value) in writer.zip(left_iter) {
                        value.0 += count;
                        *target = value;
                    }

                    break;
                }
            } else {
                left.0 += count;
                write(left);

                if let Some(new_left) = left_iter.next() {
                    left = new_left;
                } else {
                    write(right);

                    for (target, value) in writer.zip(right_iter) {
                        *target = value;
                    }

                    break;
                }
            }
        }
    }

    fn merge_sort_to_buffer(data: &mut [(u32, i32)], buffer: &mut [(u32, i32)]) {
        let n = data.len();

        if n > 1 {
            let (left, right) = data.split_at_mut(n / 2);

            Self::merge_sort_to_self(left, buffer);
            Self::merge_sort_to_self(right, buffer);
            Self::merge_to_buffer(data, buffer);
        } else {
            buffer[..n].copy_from_slice(data);
        }
    }

    fn merge_sort_to_self(data: &mut [(u32, i32)], buffer: &mut [(u32, i32)]) {
        let n = data.len();

        if n > 1 {
            let (left, right) = data.split_at_mut(n / 2);
            let (buffer_left, buffer_right) = buffer.split_at_mut(n / 2);

            Self::merge_sort_to_buffer(left, buffer_left);
            Self::merge_sort_to_buffer(right, buffer_right);
            Self::merge_to_buffer(&buffer[..n], data);
        }
    }

    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let n = nums.len();

        let mut data = nums
            .iter()
            .zip(index)
            .map(|(&num, index)| (index as u32, num))
            .collect::<Vec<_>>();

        Self::merge_sort_to_self(&mut data, &mut vec![(0, 0); n]);

        for (index, num) in data {
            nums[index as usize] = num;
        }

        nums
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        Self::create_target_array(nums, index)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
