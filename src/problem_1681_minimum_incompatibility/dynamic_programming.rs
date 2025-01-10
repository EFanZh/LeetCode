pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn normalize_nums(mut nums: Vec<i32>) -> ([u8; 16], u8) {
        nums.sort_unstable();

        let mut result = [0_u8; 16];
        let n = nums.len() as u8;

        for (target, source) in result.iter_mut().zip(nums) {
            *target = source as _;
        }

        (result, n)
    }

    fn combinations(selected: u16, mut available: u16, mut available_count: u8, required: u8, f: &mut impl FnMut(u16)) {
        if required == 0 {
            f(selected);
        } else {
            for _ in 0..available_count - (required - 1) {
                let last_bit = available & available.wrapping_neg();

                available ^= last_bit;
                available_count -= 1;

                Self::combinations(selected | last_bit, available, available_count, required - 1, f);
            }
        }
    }

    pub fn minimum_incompatibility(nums: Vec<i32>, k: i32) -> i32 {
        let (nums, n) = Self::normalize_nums(nums);
        let total_bits = if n < 16 { (1_u16 << n) - 1 } else { u16::MAX };

        #[expect(clippy::large_stack_arrays, reason = "by-design")]
        let mut cache = [0_u8; 65536];

        #[expect(clippy::large_stack_arrays, reason = "by-design")]
        let mut set_incompatibility = [u8::MAX; 65536];

        let set_size = n / k as u8;

        Self::combinations(0, total_bits, n, set_size, &mut |bits| {
            let mut x = bits;
            let mut prev = 0;

            while x != 0 {
                let num = nums[x.trailing_zeros() as usize];

                if num == prev {
                    return;
                }

                prev = num;
                x &= x - 1;
            }

            let min_value = nums[bits.trailing_zeros() as usize];
            let max_value = nums[15 - bits.leading_zeros() as usize];

            set_incompatibility[usize::from(bits)] = max_value - min_value;
        });

        let mut total_size = set_size;

        while total_size <= n {
            Self::combinations(0, total_bits, n, total_size, &mut |selected| {
                let last_bit = selected & selected.wrapping_neg();
                let mut min_incompatibility = u8::MAX;

                Self::combinations(
                    last_bit,
                    selected ^ last_bit,
                    total_size - 1,
                    set_size - 1,
                    &mut |last_set| {
                        min_incompatibility = min_incompatibility.min(
                            cache[usize::from(selected ^ last_set)]
                                .saturating_add(set_incompatibility[usize::from(last_set)]),
                        );
                    },
                );

                cache[usize::from(selected)] = min_incompatibility;
            });

            total_size += set_size;
        }

        i32::from(cache[usize::from(total_bits)] as i8)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_incompatibility(nums: Vec<i32>, k: i32) -> i32 {
        Self::minimum_incompatibility(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
