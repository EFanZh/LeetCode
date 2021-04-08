pub struct Solution;

impl Solution {
    fn helper_2(required: i32, nums: &[i32]) -> bool {
        if required == 0 {
            true
        } else if let Some((first, rest)) = nums.split_first() {
            Self::helper_2(required - first, rest) || Self::helper_2(required, rest)
        } else {
            false
        }
    }

    fn helper_1(target: i32, required: i32, nums: &mut [i32]) -> bool {
        if required == 0 {
            let (&mut first, rest) = nums.split_first_mut().unwrap();

            Self::helper_2(target - first, rest)
        } else {
            for i in 0..nums.len() {
                let num = nums[i];

                if num <= required {
                    nums.swap(0, i);

                    if Self::helper_1(target, required - num, &mut nums[1..]) {
                        return true;
                    }

                    nums.swap(0, i);
                }
            }

            false
        }
    }

    fn helper_0(target: i32, required: i32, nums: &mut [i32]) -> bool {
        if required == 0 {
            let (&mut first, rest) = nums.split_first_mut().unwrap();

            Self::helper_1(target, target - first, rest)
        } else {
            for i in 0..nums.len() {
                let num = nums[i];

                if num <= required {
                    nums.swap(0, i);

                    if Self::helper_0(target, required - num, &mut nums[1..]) {
                        return true;
                    }

                    nums.swap(0, i);
                }
            }

            false
        }
    }

    pub fn makesquare(mut nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>();

        if sum != 0 && sum % 4 == 0 {
            let target = sum / 4;

            if nums.iter().any(|&num| num > target) {
                false
            } else {
                let (&mut first, rest) = nums.split_first_mut().unwrap();

                Self::helper_0(target, target - first, rest)
            }
        } else {
            false
        }
    }
}

impl super::Solution for Solution {
    fn makesquare(nums: Vec<i32>) -> bool {
        Self::makesquare(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
