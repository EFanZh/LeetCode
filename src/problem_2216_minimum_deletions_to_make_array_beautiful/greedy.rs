pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_deletion(nums: Vec<i32>) -> i32 {
        let mut iter = nums.iter().copied();
        let mut result = 0;

        'outer: while let Some(first) = iter.next() {
            loop {
                if let Some(second) = iter.next() {
                    if first == second {
                        result += 1;
                    } else {
                        break;
                    }
                } else {
                    result += 1;

                    break 'outer;
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_deletion(nums: Vec<i32>) -> i32 {
        Self::min_deletion(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
