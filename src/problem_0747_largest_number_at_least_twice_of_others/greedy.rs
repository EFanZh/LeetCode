pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let mut max = -1;
        let mut max_index = -1;
        let mut second_max = -1;

        for (i, num) in (0..).zip(nums) {
            if num > second_max {
                if num > max {
                    second_max = max;
                    max = num;
                    max_index = i;
                } else {
                    second_max = num;
                }
            }
        }

        if max >= second_max * 2 {
            max_index
        } else {
            -1
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn dominant_index(nums: Vec<i32>) -> i32 {
        Self::dominant_index(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
