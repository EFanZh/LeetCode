pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut result = 0;

        loop {
            match nums.get(i) {
                None => return result as _,
                Some(0) => i += 1,
                Some(_) => {
                    let start = i;

                    i += 1;

                    loop {
                        match nums.get(i) {
                            None => return result.max(i - start) as _,
                            Some(0) => {
                                result = result.max(i - start);

                                i += 1;

                                break;
                            }
                            Some(_) => i += 1,
                        }
                    }
                }
            }
        }
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
