pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut min_remove_1 = 0;
        let mut min_remove_2 = 0;
        let mut min_remove_3 = 0;

        for num in nums {
            match num {
                1 => {
                    min_remove_3 += 1;
                    min_remove_2 += 1;
                }
                2 => {
                    min_remove_3 += 1;
                    min_remove_2 = u32::min(min_remove_1, min_remove_2);
                    min_remove_1 += 1;
                }
                _ => {
                    min_remove_3 = u32::min(u32::min(min_remove_1, min_remove_2), min_remove_3);
                    min_remove_2 += 1;
                    min_remove_1 += 1;
                }
            }
        }

        u32::min(u32::min(min_remove_1, min_remove_2), min_remove_3) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_operations(nums: Vec<i32>) -> i32 {
        Self::minimum_operations(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
