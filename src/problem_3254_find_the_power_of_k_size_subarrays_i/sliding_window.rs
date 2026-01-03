pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k.cast_unsigned() as usize;
        let (left, right) = nums.split_at(k - 1);
        let mut length = 0;
        let mut expected = 0;

        for &num in left {
            if num == expected {
                length += 1;
                expected += 1;
            } else {
                length = 1;
                expected = num + 1;
            }
        }

        right
            .iter()
            .map(|&num| {
                if num == expected {
                    length += 1;
                    expected += 1;
                } else {
                    length = 1;
                    expected = num + 1;
                }

                if length >= k { num } else { -1 }
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        Self::results_array(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
