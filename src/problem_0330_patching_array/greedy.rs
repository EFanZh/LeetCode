pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let n = n as u32;
        let mut result = 0;
        let mut max = 1;
        let mut iter = nums.into_iter();

        while max <= n {
            if let Some(num) = iter.next().map(|x| x as u32) {
                while num > max {
                    max *= 2;
                    result += 1;

                    if max > n {
                        return result;
                    }
                }

                max += num;
            } else {
                loop {
                    max *= 2;
                    result += 1;

                    if max > n {
                        return result;
                    }
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        Self::min_patches(nums, n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
