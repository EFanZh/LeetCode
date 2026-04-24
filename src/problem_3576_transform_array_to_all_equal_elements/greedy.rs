pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn can_make_equal(nums: Vec<i32>, k: i32) -> bool {
        nums.split_last().is_none_or(|(&last, nums)| {
            'outer: for target in [-1, 1] {
                let neg_target = -target;
                let mut budget = k;
                let mut expected = target;

                for &num in nums {
                    expected = if num == expected {
                        target
                    } else {
                        if budget == 0 {
                            continue 'outer;
                        }

                        budget -= 1;

                        neg_target
                    };
                }

                if last == expected {
                    return true;
                }
            }

            false
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn can_make_equal(nums: Vec<i32>, k: i32) -> bool {
        Self::can_make_equal(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
