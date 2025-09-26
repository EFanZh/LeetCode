pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn is_prime(num: u32) -> bool {
        if num < 2 {
            false
        } else {
            let sqrt = num.isqrt();

            for x in 2..=sqrt {
                if num.is_multiple_of(x) {
                    return false;
                }
            }

            true
        }
    }

    pub fn diagonal_prime(nums: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut result = 0;

        for (i, row) in nums.iter().enumerate() {
            let x = row[i] as u32;
            let y = row[n - 1 - i] as u32;

            if Self::is_prime(x) {
                result = result.max(x);
            }

            if Self::is_prime(y) {
                result = result.max(y);
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn diagonal_prime(nums: Vec<Vec<i32>>) -> i32 {
        Self::diagonal_prime(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
