pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn get_largest_outlier(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut map = [0_u8; 2001];

        for &num in &nums {
            sum += num;

            let state = &mut map[(num as usize).wrapping_add(1000)];

            *state += u8::from(*state < 2);
        }

        let mut result = i32::MIN;

        for &num in &nums {
            if num > result {
                let rest = sum - num;

                if rest & 1 == 0 {
                    let half = rest / 2;

                    if map
                        .get((half as usize).wrapping_add(1000))
                        .is_some_and(|&state| state > u8::from(num == half))
                    {
                        result = num;
                    }
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_largest_outlier(nums: Vec<i32>) -> i32 {
        Self::get_largest_outlier(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
