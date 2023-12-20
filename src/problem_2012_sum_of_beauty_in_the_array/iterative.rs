pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn sum_of_beauties(nums: Vec<i32>) -> i32 {
        let iter_1 = nums.iter().copied();
        let mut iter_2 = iter_1.clone();

        iter_2.next();

        let mut iter_3 = iter_2.clone();

        iter_3.next();

        // Calculate right maxes.

        let mut right_mins = vec![0; nums.len() - 2];
        let mut right_min = i32::MAX;

        for (target, num) in right_mins.iter_mut().zip(iter_3.clone()).rev() {
            right_min = right_min.min(num);
            *target = right_min;
        }

        let mut result = 0;

        let mut left_max = i32::MIN;

        for (((left, middle), right), right_min) in iter_1.zip(iter_2).zip(iter_3).zip(right_mins) {
            left_max = left_max.max(left);

            if left_max < middle && middle < right_min {
                result += 2;
            } else if left < middle && middle < right {
                result += 1;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sum_of_beauties(nums: Vec<i32>) -> i32 {
        Self::sum_of_beauties(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
