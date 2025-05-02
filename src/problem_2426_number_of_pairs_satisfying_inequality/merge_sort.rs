pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn assign((target, source): (&mut i32, i32)) {
        *target = source;
    }

    fn merge(nums: &[i32], buffer: &mut [i32], diff: i32, result: &mut u64) {
        let (nums_left, nums_right) = nums.split_at(nums.len() / 2);
        let mut left_iter = nums_left.iter().copied();
        let mut right_iter = nums_right.iter().copied();
        let mut buffer_iter = buffer.iter_mut();

        // Count pairs.

        let mut i = 0;
        let mut left_i = nums_left[i];
        let mut j = 0;

        'outer: while let Some(&right_j) = nums_right.get(j) {
            let upper = right_j + diff;

            loop {
                if left_i > upper {
                    *result += i as u64;

                    break;
                }

                i += 1;

                if let Some(&new_left_i) = nums_left.get(i) {
                    left_i = new_left_i;
                } else {
                    *result += nums_left.len() as u64 * (nums_right.len() - j) as u64;

                    break 'outer;
                }
            }

            j += 1;
        }

        // Merge.

        let mut left = left_iter.next().unwrap();
        let mut write = |num| *buffer_iter.next().unwrap() = num;

        'outer: for right in right_iter.by_ref() {
            while left <= right {
                write(left);

                if let Some(next_left) = left_iter.next() {
                    left = next_left;
                } else {
                    left = right;
                    left_iter = right_iter;

                    break 'outer;
                }
            }

            write(right);
        }

        write(left);
        buffer_iter.zip(left_iter).for_each(Self::assign);
    }

    fn merge_sort_in_place(nums: &mut [i32], buffer: &mut [i32], diff: i32, result: &mut u64) {
        let n = nums.len();

        if n > 1 {
            let split = n / 2;
            let (left, right) = nums.split_at_mut(split);
            let (left_buffer, right_buffer) = buffer.split_at_mut(split);

            Self::merge_sort_to_buffer(left, left_buffer, diff, result);
            Self::merge_sort_to_buffer(right, right_buffer, diff, result);
            Self::merge(buffer, nums, diff, result);
        }
    }

    fn merge_sort_to_buffer(nums: &mut [i32], buffer: &mut [i32], diff: i32, result: &mut u64) {
        let n = nums.len();

        if n > 1 {
            let split = n / 2;
            let (left, right) = nums.split_at_mut(split);
            let (left_buffer, right_buffer) = buffer.split_at_mut(split);

            Self::merge_sort_in_place(left, left_buffer, diff, result);
            Self::merge_sort_in_place(right, right_buffer, diff, result);
            Self::merge(nums, buffer, diff, result);
        } else {
            buffer.iter_mut().zip(nums.iter().copied()).for_each(Self::assign);
        }
    }

    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, diff: i32) -> i64 {
        let mut nums1 = nums1;
        let mut nums2 = nums2;

        nums1.iter_mut().zip(&nums2).for_each(|(lhs, rhs)| *lhs -= rhs);

        let mut result = 0;

        Self::merge_sort_in_place(&mut nums1, &mut nums2, diff, &mut result);

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, diff: i32) -> i64 {
        Self::number_of_pairs(nums1, nums2, diff)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
