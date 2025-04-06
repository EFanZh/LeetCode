pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
        let mut diff = 0;

        for x in nums {
            diff += x;

            let mut x = x as u32;

            while x != 0 {
                diff -= (x % 10) as i32;
                x /= 10;
            }
        }

        diff.abs()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn difference_of_sum(nums: Vec<i32>) -> i32 {
        Self::difference_of_sum(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
