pub struct Solution;

impl Solution {
    fn subsets_helper(mut slice: &[i32], base: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        while let Some((first, rest)) = slice.split_first() {
            base.push(*first);
            Self::subsets_helper(rest, base, result);
            base.pop();

            slice = rest;
        }

        result.push(base.clone());
    }

    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        Self::subsets_helper(&nums, &mut Vec::with_capacity(nums.len()), &mut result);

        result
    }
}

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
