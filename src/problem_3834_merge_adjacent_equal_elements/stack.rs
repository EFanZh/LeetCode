pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn merge_adjacent(nums: Vec<i32>) -> Vec<i64> {
        let mut result = Vec::new();

        for num in nums {
            let mut num = i64::from(num);

            while result.pop_if(|last| *last == num).is_some() {
                num *= 2;
            }

            result.push(num);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn merge_adjacent(nums: Vec<i32>) -> Vec<i64> {
        Self::merge_adjacent(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
