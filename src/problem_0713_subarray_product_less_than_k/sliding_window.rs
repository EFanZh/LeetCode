pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as u32;
        let mut result = 0;

        if k > 1 {
            let mut start = 0;
            let mut end = 0;
            let mut product = 1;

            for &num in &nums {
                let num = num as u32;

                end += 1;
                product *= num;

                while product >= k {
                    product /= nums[start] as u32;
                    start += 1;
                }

                result += end - start;
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        Self::num_subarray_product_less_than_k(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
