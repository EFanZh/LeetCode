pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        let mut result = 0;

        nums.sort_unstable();

        for (i, first) in nums.iter().enumerate() {
            let mut k_start = i + 2;

            for (j, second) in (i + 2..).zip(&nums[i + 1..]) {
                let start = k_start.max(j);
                let upper_bound = first + second;

                let count = nums[start..]
                    .binary_search_by(|&x| {
                        if x < upper_bound {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    })
                    .unwrap_err();

                k_start = start + count;
                result += k_start - j;
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn triangle_number(nums: Vec<i32>) -> i32 {
        Self::triangle_number(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
