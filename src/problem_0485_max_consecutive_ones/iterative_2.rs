pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut result = 0;

        while let Some(&num) = nums.get(i) {
            if num == 0 {
                i += 1;
            } else {
                let start = i;

                i += 1;

                loop {
                    if let Some(&num) = nums.get(i) {
                        if num == 0 {
                            result = result.max(i - start);

                            i += 1;

                            break;
                        }

                        i += 1;
                    } else {
                        return result.max(i - start) as _;
                    }
                }
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        Self::find_max_consecutive_ones(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
