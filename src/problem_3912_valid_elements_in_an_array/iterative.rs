pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_valid_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let mut right_maxes = [0; 100];
        let mut max = 0;

        right_maxes.iter_mut().zip(&nums).rev().for_each(|(target, &num)| {
            *target = max;
            max = max.max(num);
        });

        max = 0;

        let mut i = 0;

        nums.retain(|&num| {
            let retain = num > max || num > right_maxes[i];

            max = max.max(num);
            i += 1;

            retain
        });

        nums.into_iter().map(u32::cast_signed).collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_valid_elements(nums: Vec<i32>) -> Vec<i32> {
        Self::find_valid_elements(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
