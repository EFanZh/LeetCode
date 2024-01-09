pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn smallest_equal(nums: Vec<i32>) -> i32 {
        let mut i_mod_10 = 0;

        for (i, num) in (0..).zip(nums) {
            if i_mod_10 == num {
                return i;
            }

            if i_mod_10 == 9 {
                i_mod_10 = 0;
            } else {
                i_mod_10 += 1;
            }
        }

        -1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn smallest_equal(nums: Vec<i32>) -> i32 {
        Self::smallest_equal(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
