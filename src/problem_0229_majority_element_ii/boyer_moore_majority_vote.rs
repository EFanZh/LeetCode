pub struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut result_1 = 0;
        let mut votes_1 = 0;
        let mut result_2 = 0;
        let mut votes_2 = 0;

        for &num in &nums {
            if num == result_1 {
                votes_1 += 1;
            } else if num == result_2 {
                votes_2 += 1;
            } else if votes_1 == 0 {
                result_1 = num;
                votes_1 = 1;
            } else if votes_2 == 0 {
                result_2 = num;
                votes_2 = 1;
            } else {
                votes_1 -= 1;
                votes_2 -= 1;
            }
        }

        let mut result = Vec::with_capacity(2);
        let mut count_1 = 0;
        let mut count_2 = 0;

        for &num in &nums {
            if num == result_1 {
                count_1 += 1;
            } else if num == result_2 {
                count_2 += 1;
            }
        }

        let one_third = nums.len() / 3;

        if count_1 > one_third {
            result.push(result_1);
        }

        if count_2 > one_third {
            result.push(result_2);
        }

        result
    }
}

impl super::Solution for Solution {
    fn majority_element(nums: Vec<i32>) -> Vec<i32> {
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
