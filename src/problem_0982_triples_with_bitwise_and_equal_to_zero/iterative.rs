pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_triplets(nums: Vec<i32>) -> i32 {
        let nums = || nums.iter().map(|&num| num as u16);

        #[expect(clippy::large_stack_arrays, reason = "by-design")]
        let mut counts = [0_u32; 65536];

        for left in nums() {
            for right in nums() {
                counts[usize::from(left & right)] += 1;
            }
        }

        let mut result = 0;

        for left in nums() {
            let mut right = 0_u16;

            for &count in &counts {
                if right & left == 0 {
                    result += count;
                }

                right = right.wrapping_add(1);
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_triplets(nums: Vec<i32>) -> i32 {
        Self::count_triplets(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
