pub struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        // https://en.wikipedia.org/wiki/Boyer%E2%80%93Moore_majority_vote_algorithm.

        let mut result = 0;
        let mut votes = 0;

        for num in nums {
            if num == result {
                votes += 1;
            } else if votes == 0 {
                result = num;
                votes = 1;
            } else {
                votes -= 1;
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn majority_element(nums: Vec<i32>) -> i32 {
        Self::majority_element(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
