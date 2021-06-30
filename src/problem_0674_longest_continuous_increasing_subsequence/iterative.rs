pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let (&first, rest) = nums.split_first().unwrap();
        let mut prev = first;
        let mut length = 1;
        let mut result = 1;

        for &num in rest {
            if num > prev {
                length += 1;
            } else {
                result = result.max(length);
                length = 1;
            }

            prev = num;
        }

        result.max(length)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        Self::find_length_of_lcis(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
