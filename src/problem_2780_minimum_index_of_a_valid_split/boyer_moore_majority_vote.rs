pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_index(nums: Vec<i32>) -> i32 {
        let mut candidate = 0;
        let mut count = 0;

        for &num in &nums {
            if count == 0 {
                candidate = num;
                count = 1;
            } else if num == candidate {
                count += 1;
            } else {
                count -= 1;
            }
        }

        let total_count = nums.iter().filter(|&&num| num == candidate).count();

        if total_count * 2 < nums.len() + 2 {
            -1
        } else {
            let mut result = 0;

            count = 0;

            for num in nums {
                if num == candidate {
                    count += 1;

                    if count * 2 > result + 1 {
                        break;
                    }
                }

                result += 1;
            }

            result
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_index(nums: Vec<i32>) -> i32 {
        Self::minimum_index(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
