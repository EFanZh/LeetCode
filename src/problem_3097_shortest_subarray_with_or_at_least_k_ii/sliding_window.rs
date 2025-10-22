pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn iter_bits(mut x: u32, mut f: impl FnMut(u32)) {
        while x != 0 {
            f(x.trailing_zeros());
            x &= x - 1;
        }
    }

    fn shrink_left(nums: &[u32], k: u32, start: &mut usize, acc_or: &mut u32, counts: &mut [u32; 32]) {
        loop {
            let start_num = nums[*start];
            let mut start_remove_bits = 0;

            Self::iter_bits(start_num, |bit| {
                if counts[bit as usize] == 1 {
                    start_remove_bits |= 1 << bit;
                }
            });

            let candidate_acc_or = *acc_or ^ start_remove_bits;

            if candidate_acc_or >= k {
                *start += 1;
                *acc_or = candidate_acc_or;
                Self::iter_bits(start_num, |bit| counts[bit as usize] -= 1);
            } else {
                break;
            }
        }
    }

    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        if k == 0 {
            return 1;
        }

        let nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let k = k.cast_unsigned();
        let mut acc_or = 0;
        let mut counts = [0; 32];
        let mut i = 0;

        loop {
            if let Some(&num) = nums.get(i) {
                i += 1;
                acc_or |= num;
                Self::iter_bits(num, |bit| counts[bit as usize] += 1);

                if acc_or >= k {
                    break;
                }
            } else {
                return -1;
            }
        }

        let mut start = 0;

        Self::shrink_left(&nums, k, &mut start, &mut acc_or, &mut counts);

        let mut result = i - start;

        while let Some(&num) = nums.get(i) {
            i += 1;
            acc_or |= num;
            Self::iter_bits(num, |bit| counts[bit as usize] += 1);
            Self::shrink_left(&nums, k, &mut start, &mut acc_or, &mut counts);

            result = result.min(i - start);
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        Self::minimum_subarray_length(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
