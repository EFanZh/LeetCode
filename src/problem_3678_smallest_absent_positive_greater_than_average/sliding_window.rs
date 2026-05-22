pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn smallest_absent(nums: Vec<i32>) -> i32 {
        let mut exists = [false; 100];
        let mut sum = 0;
        let n = nums.len() as _;

        for num in nums {
            sum += num;

            if let Some(exists) = exists.get_mut((num as usize).wrapping_sub(1)) {
                *exists = true;
            }
        }

        let min_value = sum.div_euclid(n).max(0) + 1;

        exists
            .get((min_value as usize - 1)..)
            .and_then(|candidates| candidates.iter().position(|&exists| !exists))
            .map_or(101, |i| min_value + i as i32)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn smallest_absent(nums: Vec<i32>) -> i32 {
        Self::smallest_absent(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
