pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut farest = 0;

        for (i, num) in (0..).zip(nums) {
            if i <= farest {
                farest = farest.max(i + num);
            } else {
                return false;
            }
        }

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn can_jump(nums: Vec<i32>) -> bool {
        Self::can_jump(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
