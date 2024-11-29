pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut buckets = [0_u32; 85];
        let mut result = -1;

        for &num in &nums {
            let num = num as u32;
            let mut bucket = 0_usize;
            let mut x = num;

            while x != 0 {
                bucket += (x % 10) as usize;
                x /= 10;
            }

            let max = &mut buckets[bucket];

            if *max != 0 {
                result = result.max((*max + num) as i32);
            }

            *max = (*max).max(num);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_sum(nums: Vec<i32>) -> i32 {
        Self::maximum_sum(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
