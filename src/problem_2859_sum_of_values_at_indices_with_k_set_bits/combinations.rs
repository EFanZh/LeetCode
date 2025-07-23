pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn combinations(n: u8, mut k: u8, base: usize, f: &mut impl FnMut(usize) -> bool) -> bool {
        if k == 0 {
            f(base)
        } else {
            k -= 1;

            for i in k..n {
                if Self::combinations(i, k, base | (1 << i), f) {
                    return true;
                }
            }

            false
        }
    }

    pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;

        Self::combinations(16, k as _, 0, &mut |index| {
            nums.get(index).is_none_or(|num| {
                result += num;

                false
            })
        });

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
        Self::sum_indices_with_k_set_bits(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
