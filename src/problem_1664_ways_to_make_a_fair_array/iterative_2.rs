pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn ways_to_make_fair(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let buffer = nums.as_mut_slice();
        let mut even_sum = 0;
        let mut odd_sum = 0;

        // Calculate sums.

        {
            let mut iter = buffer.iter_mut();

            while let Some(target) = iter.next() {
                even_sum += *target;
                *target = even_sum;

                if let Some(target) = iter.next() {
                    odd_sum += *target;
                    *target = odd_sum;
                } else {
                    break;
                }
            }
        }

        let mut result = 0;
        let mut prev_even_sum = 0;
        let mut prev_odd_sum = 0;

        let mut iter = buffer.iter().copied();

        while let Some(sum) = iter.next() {
            result += i32::from((prev_even_sum + (odd_sum - prev_odd_sum)) == (prev_odd_sum + (even_sum - sum)));

            prev_even_sum = sum;

            if let Some(sum) = iter.next() {
                result += i32::from((prev_odd_sum + (even_sum - prev_even_sum)) == (prev_even_sum + (odd_sum - sum)));

                prev_odd_sum = sum;
            } else {
                break;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn ways_to_make_fair(nums: Vec<i32>) -> i32 {
        Self::ways_to_make_fair(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
