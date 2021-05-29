pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::fmt::Error;

impl Solution {
    fn helper(nums: Vec<i32>) -> Result<String, Error> {
        use std::fmt::Write;

        let mut result = String::new();
        let mut iter = nums.into_iter();

        write!(&mut result, "{}", iter.next().unwrap())?;

        if let Some(second) = iter.next() {
            if iter.len() == 0 {
                write!(&mut result, "/{}", second)?;
            } else {
                write!(&mut result, "/({}", second)?;

                for num in iter {
                    write!(&mut result, "/{}", num)?;
                }

                result.push(')');
            }
        }

        Ok(result)
    }

    pub fn optimal_division(nums: Vec<i32>) -> String {
        Self::helper(nums).unwrap()
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
