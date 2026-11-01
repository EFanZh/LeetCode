pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let k = k.cast_unsigned() as usize;
        let mut sum = 0;

        for num in &mut nums {
            let mut digits = [0; 10];
            let mut i = 0;
            let mut x = *num;

            while x != 0 {
                digits[i] = x % 10;
                x /= 10;
                i += 1;
            }

            let digits = &mut digits[..i];

            digits.sort_unstable();

            let max = digits.iter().rev().fold(0, |max, &x| max * 10 + x);

            sum += u64::from(*num);
            *num = max - *num;
        }

        let n = nums.len();

        nums.select_nth_unstable(n - k);

        nums[n - k..]
            .iter()
            .fold(sum, |sum, &x| sum + u64::from(x))
            .cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximize_sum(nums: Vec<i32>, k: i32) -> i64 {
        Self::maximize_sum(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
