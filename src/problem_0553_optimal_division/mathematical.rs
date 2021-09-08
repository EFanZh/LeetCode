pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn helper(nums: Vec<i32>) -> String {
        use std::fmt::Write;

        let mut result = String::new();
        let mut iter = nums.into_iter();

        write!(&mut result, "{}", iter.next().unwrap()).unwrap();

        if let Some(second) = iter.next() {
            if iter.len() == 0 {
                write!(&mut result, "/{}", second).unwrap();
            } else {
                write!(&mut result, "/({}", second).unwrap();

                for num in iter {
                    write!(&mut result, "/{}", num).unwrap();
                }

                result.push(')');
            }
        }

        result
    }

    pub fn optimal_division(nums: Vec<i32>) -> String {
        Self::helper(nums)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn optimal_division(nums: Vec<i32>) -> String {
        Self::optimal_division(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
