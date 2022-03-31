pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut zeros = 0;
        let mut i = 0;

        while zeros != k {
            if let Some(&num) = nums.get(i) {
                i += 1;

                if num == 0 {
                    zeros += 1;
                }
            } else {
                return i as _;
            }
        }

        let mut result = i;
        let mut start = 0;

        while let Some(&num) = nums.get(i) {
            i += 1;

            if num == 0 {
                while nums[start] != 0 {
                    start += 1;
                }

                start += 1;
            }

            result = result.max(i - start);
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        Self::longest_ones(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
