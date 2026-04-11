pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximum_possible_size(nums: Vec<i32>) -> i32 {
        let mut prev = 0;
        let mut result = 0;

        for num in nums {
            let num = num.cast_unsigned();

            if num >= prev {
                prev = num;
                result += 1;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_possible_size(nums: Vec<i32>) -> i32 {
        Self::maximum_possible_size(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
