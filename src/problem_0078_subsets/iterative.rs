pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut stack = Vec::with_capacity(nums.len());
        let mut slice = nums.as_slice();
        let mut base = Vec::with_capacity(nums.len());

        loop {
            if let Some((first, rest)) = slice.split_first() {
                base.push(*first);
                stack.push(rest);
                slice = rest;
            } else {
                result.push(base.clone());

                if let Some(rest) = stack.pop() {
                    base.pop();
                    slice = rest;
                } else {
                    break;
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Self::subsets(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
