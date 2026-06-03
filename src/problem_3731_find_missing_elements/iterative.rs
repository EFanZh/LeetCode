pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_missing_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut exists = [false; 100];
        let mut min = u32::MAX;
        let mut max = 0;

        while let Some(num) = nums.pop() {
            let num = num.cast_unsigned();

            min = min.min(num);
            max = max.max(num);
            exists[num as usize - 1] = true;
        }

        nums.extend(
            (min..=max)
                .zip(&exists[min as usize - 1..])
                .filter_map(|(x, &exists)| (!exists).then_some(x.cast_signed())),
        );

        nums
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_missing_elements(nums: Vec<i32>) -> Vec<i32> {
        Self::find_missing_elements(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
