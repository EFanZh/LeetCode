pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 0..nums.len() {
            let mut num = nums[i];
            let expected = i as i32 + 1;

            while num != expected {
                let j = (num - 1) as _;
                let target = nums[j];

                if num == target {
                    break;
                }

                nums.swap(i, j);
                num = target;
            }
        }

        (1..=nums.len() as _)
            .filter_map(|i| {
                let num = nums[(i - 1) as usize];

                if num == i {
                    None
                } else {
                    Some(num)
                }
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        Self::find_duplicates(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
