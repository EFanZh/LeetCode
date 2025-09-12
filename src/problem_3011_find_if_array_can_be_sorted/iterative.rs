pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn can_sort_array(nums: Vec<i32>) -> bool {
        let nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let mut prev_max = 0;
        let mut current_min = 0;
        let mut current_max = 0;
        let mut current_bits = 0;

        for num in nums {
            let bits = num.count_ones();

            if bits == current_bits {
                if num < current_min {
                    current_min = num;
                } else if num > current_max {
                    current_max = num;
                }
            } else {
                if current_min < prev_max {
                    return false;
                }

                prev_max = current_max;
                current_min = num;
                current_max = num;
                current_bits = bits;
            }
        }

        current_min >= prev_max
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn can_sort_array(nums: Vec<i32>) -> bool {
        Self::can_sort_array(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
