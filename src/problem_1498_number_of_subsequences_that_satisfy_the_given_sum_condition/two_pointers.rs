pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

const MODULUS: u32 = 1_000_000_007;

impl Solution {
    /// Calculates `2.pow(x) % MODULUS`.
    fn exp_mod(buffer: &mut Vec<u32>, x: usize) -> u32 {
        if let Some(&result) = buffer.get(x) {
            result
        } else {
            let mut result = 500_000_004_u32;

            buffer.extend((0..=x).map(|_| {
                result *= 2;
                result = result.checked_sub(MODULUS).unwrap_or(result);

                result
            }));

            result
        }
    }

    pub fn num_subseq(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;

        nums.sort_unstable();

        let mut iter = nums.iter().copied();
        let mut left = iter.next().unwrap();
        let mut buffer = Vec::new();
        let mut result = 0_u32;

        'outer: while let Some(right) = iter.next_back() {
            while left + right <= target {
                result += Self::exp_mod(&mut buffer, iter.len() + 1);
                result = result.checked_sub(MODULUS).unwrap_or(result);

                if let Some(new_left) = iter.next() {
                    left = new_left;
                } else {
                    left = right;

                    break 'outer;
                }
            }
        }

        if left * 2 <= target {
            result += 1;
            result = result.checked_sub(MODULUS).unwrap_or(result);
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_subseq(nums: Vec<i32>, target: i32) -> i32 {
        Self::num_subseq(nums, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
