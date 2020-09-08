pub struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![Vec::new()];

        result.reserve((1 << nums.len()) - 1);

        for num in nums {
            for i in 0..result.len() {
                let mut item = result[i].clone();

                item.push(num);

                result.push(item);
            }
        }

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
