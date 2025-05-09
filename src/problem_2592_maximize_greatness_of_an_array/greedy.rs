pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximize_greatness(nums: Vec<i32>) -> i32 {
        let mut nums = nums.into_iter().map(|num| num as u32).collect::<Vec<_>>();

        nums.sort_unstable();

        let iter_1 = nums.iter().copied();
        let mut iter_2 = iter_1.clone();
        let mut result = 0;

        for num in iter_1 {
            if iter_2.any(|x| x > num) {
                result += 1;
            } else {
                break;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximize_greatness(nums: Vec<i32>) -> i32 {
        Self::maximize_greatness(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
