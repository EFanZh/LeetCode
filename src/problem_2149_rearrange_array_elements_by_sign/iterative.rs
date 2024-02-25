pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; nums.len()];
        let mut positive_index = 0;
        let mut negative_index = 1;

        for num in nums {
            let index = if num < 0 {
                &mut negative_index
            } else {
                &mut positive_index
            };

            result[*index] = num;
            *index += 2;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        Self::rearrange_array(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
